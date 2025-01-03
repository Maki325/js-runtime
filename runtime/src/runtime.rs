use crate::{module_map::ModuleMap, Module};
use std::collections::{HashMap, HashSet};

macro_rules! get_data_mut {
  ($rt:ident) => {
    unsafe { &mut *((($rt.data as *mut usize).clone()) as *mut crate::runtime::RuntimeData) }
  };
}

macro_rules! fake_clone {
  ($data:tt, $t:ty) => {
    unsafe { &mut *(((($data as *mut $t) as *mut usize).clone()) as *mut $t) }
  };
}

fn dynamic_import<'s>(
  ctx: &mut v8::HandleScope<'s>,
  _host_defined_options: v8::Local<'s, v8::Data>,
  _resource_name: v8::Local<'s, v8::Value>,
  _specifier: v8::Local<'s, v8::String>,
  _import_attributes: v8::Local<'s, v8::FixedArray>,
) -> Option<v8::Local<'s, v8::Promise>> {
  let resolver = v8::PromiseResolver::new(ctx).unwrap();
  let null = v8::null(ctx);
  resolver.resolve(ctx, null.cast());
  return Some(resolver.get_promise(ctx));
}

// I.e. v8::Isolate
#[derive(Debug)]
pub struct Runtime {
  pub(crate) data: *mut RuntimeData,
  pub(crate) map: ModuleMap,

  pub(crate) waker_id: u32,
  pub(crate) waker_map: HashMap<u32, std::task::Waker>,

  pub(crate) enqueue_id: u32,
  pub(crate) enqueue_map: HashMap<u32, tokio::sync::mpsc::Sender<std::io::Result<String>>>,
}

#[derive(Debug)]
pub(crate) struct RuntimeData {
  pub(crate) isolate: v8::OwnedIsolate,
  pub(crate) context: v8::Global<v8::Context>,
  pub(crate) waker_key: v8::Global<v8::Private>,

  global_keys: HashSet<String>,
}

pub enum RuntimeOptions {
  Default,
  SnapshotCreator,
  FromBlob(&'static [u8]),
}

impl RuntimeData {
  pub fn new(options: RuntimeOptions) -> RuntimeData {
    let mut isolate = match options {
      RuntimeOptions::Default => v8::Isolate::new(v8::CreateParams::default()),
      RuntimeOptions::SnapshotCreator => {
        v8::Isolate::snapshot_creator(None, Some(v8::CreateParams::default()))
      }
      RuntimeOptions::FromBlob(data) => {
        v8::Isolate::new(v8::CreateParams::default().snapshot_blob(data))
      }
    };
    isolate.set_host_import_module_dynamically_callback(dynamic_import);
    isolate.set_promise_hook(promise_hook);

    let (waker_key, context, global_keys) = {
      let handle_scope = &mut v8::HandleScope::new(&mut isolate);
      let context = if let RuntimeOptions::FromBlob(_) = options {
        v8::Context::from_snapshot(handle_scope, 0, Default::default()).unwrap()
      } else {
        v8::Context::new(handle_scope, Default::default())
      };
      let handle_scope = &mut v8::ContextScope::new(handle_scope, context);
      let global_keys = Self::setup_context(handle_scope, context);

      let private_name = v8::String::new(handle_scope, "WakerKey").unwrap();
      let private = v8::Private::new(handle_scope, Some(private_name));
      let waker_key = v8::Global::new(handle_scope, private);

      let context = v8::Global::new(handle_scope, context);

      (waker_key, context, global_keys)
    };

    let runtime_data = RuntimeData {
      isolate,
      context,
      waker_key,
      global_keys,
    };

    return runtime_data;
  }

  pub fn handle_scope<'a>(&'a mut self) -> v8::HandleScope<'a> {
    return v8::HandleScope::with_context(&mut self.isolate, self.context.clone());
  }

  pub fn context<'s>(&self, scope: &mut v8::HandleScope<'s, ()>) -> v8::Local<'s, v8::Context> {
    return v8::Local::new(scope, self.context.clone());
  }

  pub fn add_global_fn(
    &mut self,
    name: crate::FastStaticString,
    f: impl v8::MapFnTo<v8::FunctionCallback>,
  ) {
    self.global_keys.insert(name.to_string());

    let context = self.context.clone();
    let handle_scope = &mut self.handle_scope();
    let context = v8::Local::new(handle_scope, context);
    let context_scope = &mut v8::ContextScope::new(handle_scope, context);

    let global_obj = context.global(context_scope);

    let f = v8::Function::new(context_scope, f).unwrap();
    let name = name.v8_string(context_scope);
    global_obj.set(context_scope, name.into(), f.into());
  }

  pub fn isolate(self) -> v8::OwnedIsolate {
    let RuntimeData {
      mut isolate,
      context,
      waker_key,
      global_keys,
    } = self;

    drop(waker_key);

    {
      let handle_scope = &mut v8::HandleScope::new(&mut isolate);
      let global_scope = v8::Local::new(handle_scope, context);

      global_scope.clear_all_slots();

      {
        let context_scope = &mut v8::ContextScope::new(handle_scope, global_scope);
        let global = global_scope.global(context_scope);
        for key in global_keys {
          let key = v8::String::new(context_scope, &key).unwrap();
          global.delete(context_scope, key.into());
        }
      }

      let default_context = v8::Context::new(handle_scope, Default::default());
      handle_scope.set_default_context(default_context);
      handle_scope.add_context(global_scope);
    }

    return isolate;
  }

  fn setup_context(
    handle_scope: &mut v8::HandleScope<'_>,
    context: v8::Local<v8::Context>,
  ) -> HashSet<String> {
    let mut global_keys = HashSet::new();

    let context = v8::Local::new(handle_scope, &context);
    let context_scope = &mut v8::ContextScope::new(handle_scope, context);

    let global_obj = context.global(context_scope);

    let set_timeout = v8::Function::new(context_scope, crate::intrinsics::set_timeout).unwrap();
    global_keys.insert("setTimeout".to_string());
    let name = v8::String::new(context_scope, "setTimeout").unwrap().into();
    global_obj.set(context_scope, name, set_timeout.into());

    let log = v8::Function::new(context_scope, crate::intrinsics::log).unwrap();
    global_keys.insert("log".to_string());
    let name = v8::String::new(context_scope, "log").unwrap().into();
    global_obj.set(context_scope, name, log.into());

    return global_keys;
  }
}

extern "C" fn promise_hook(
  promise_hook_type: v8::PromiseHookType,
  promise: v8::Local<'_, v8::Promise>,
  _parent: v8::Local<'_, v8::Value>,
) {
  let scope = &mut unsafe { v8::CallbackScope::new(promise) };
  let handle_scope = &mut v8::HandleScope::new(scope);
  if promise_hook_type != v8::PromiseHookType::Resolve {
    return;
  }
  let rt = Runtime::get();
  let waker_key = rt.waker_key(handle_scope);

  let value = promise.get_private(handle_scope, waker_key);
  if let Some(value) = value {
    if value.is_number() {
      let id = value.number_value(handle_scope);
      if let Some(id) = id {
        promise.delete_private(handle_scope, waker_key);
        rt.wake(id as u32);
      }
    }
  }
}

impl Runtime {
  pub fn new(options: RuntimeOptions) -> Runtime {
    return Runtime {
      map: ModuleMap::new(),
      data: Box::leak(Box::new(RuntimeData::new(options))),

      waker_id: 0,
      waker_map: HashMap::new(),
      enqueue_id: 0,
      enqueue_map: HashMap::new(),
    };
  }

  pub fn module_from_file<'a>(
    &'a mut self,
    handle_scope: &mut v8::HandleScope<'_>,
    path: &str,
    reload: crate::module_map::Reload,
  ) -> Option<&'a mut Module> {
    let path = std::path::absolute(path).unwrap();
    let path = path.to_str().unwrap();

    return self.map.get_module(handle_scope, path, reload);
  }

  pub fn handle_scope<'a>(&self) -> v8::HandleScope<'a> {
    return get_data_mut!(self).handle_scope();
  }

  pub fn context<'s>(&self, scope: &mut v8::HandleScope<'s, ()>) -> v8::Local<'s, v8::Context> {
    return get_data_mut!(self).context(scope);
  }

  pub fn waker_key<'s>(
    &mut self,
    scope: &mut v8::HandleScope<'s, ()>,
  ) -> v8::Local<'s, v8::Private> {
    let a = get_data_mut!(self);
    return v8::Local::new(scope, a.waker_key.clone());
  }

  pub fn insert_waker(&mut self, waker: std::task::Waker, id: Option<u32>) -> u32 {
    if let Some(id) = id {
      self.waker_map.insert(id, waker);
      return id;
    } else {
      let id = self.waker_id;
      self.waker_id += 1;
      self.waker_map.insert(id, waker);
      return id;
    }
  }

  pub fn wake(&mut self, id: u32) {
    self.waker_map.remove(&id).unwrap().wake_by_ref();
  }

  pub fn insert_sender(
    &mut self,
    sender: tokio::sync::mpsc::Sender<std::io::Result<String>>,
  ) -> u32 {
    let id = self.enqueue_id;
    self.enqueue_id += 1;
    self.enqueue_map.insert(id, sender);
    return id;
  }

  pub fn get_sender(
    &mut self,
    id: u32,
  ) -> Option<&mut tokio::sync::mpsc::Sender<std::io::Result<String>>> {
    return self.enqueue_map.get_mut(&id);
  }

  pub fn remove_sender(
    &mut self,
    id: u32,
  ) -> Option<tokio::sync::mpsc::Sender<std::io::Result<String>>> {
    return self.enqueue_map.remove(&id);
  }

  pub fn add_global_fn(
    &self,
    name: crate::FastStaticString,
    f: impl v8::MapFnTo<v8::FunctionCallback>,
  ) {
    let data = get_data_mut!(self);
    data.add_global_fn(name, f);
  }

  pub fn write_blob(self) {
    let Runtime {
      data,
      map,
      waker_id: _,
      waker_map: _,
      enqueue_id: _,
      enqueue_map: _,
    } = self;
    let data = unsafe { Box::from_raw(data) };

    map.drain();

    let isolate = data.isolate();
    let data = isolate.create_blob(v8::FunctionCodeHandling::Keep);
    std::fs::write("./blob.bin", data.unwrap()).unwrap();

    println!("Blog written to file, exiting...");
    std::process::exit(0);
  }

  pub fn get<'s>() -> &'s mut Self {
    return crate::prelude::RUNTIME.with_borrow_mut(|rt| {
      return fake_clone!({ rt.get_mut().unwrap() }, crate::runtime::Runtime);
    });
  }
}
