use crate::{module_map::ModuleMap, Module};
use std::collections::HashMap;

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
  #[allow(dead_code)]
  pub(crate) context: v8::Global<v8::Context>,
  pub(crate) handle_scope: &'static mut v8::HandleScope<'static>,
  pub(crate) waker_key: v8::Global<v8::Private>,
}

impl RuntimeData {
  pub fn new() -> RuntimeData {
    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    isolate.set_host_import_module_dynamically_callback(dynamic_import);
    let context = Self::setup_context(&mut isolate);

    let isolate_clone = fake_clone!({ &mut isolate }, v8::OwnedIsolate);
    let handle_scope = Box::leak(Box::new(v8::HandleScope::with_context(
      isolate_clone,
      &context,
    )));

    let private_name = v8::String::new(handle_scope, "WakerKey").unwrap();
    let private = v8::Private::new(handle_scope, Some(private_name));
    let waker_key = v8::Global::new(&mut isolate, private);

    let mut runtime_data = RuntimeData {
      isolate,
      context,
      handle_scope,
      waker_key,
    };

    let module_map = ModuleMap::new();

    runtime_data.isolate.set_slot(module_map);
    runtime_data.isolate.set_promise_hook(promise_hook);

    return runtime_data;
  }

  fn setup_context(isolate: &mut v8::Isolate) -> v8::Global<v8::Context> {
    let handle_scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(handle_scope, Default::default());

    {
      let context_scope = &mut v8::ContextScope::new(handle_scope, context);

      let global_obj = context.global(context_scope);

      let set_timeout = v8::Function::new(context_scope, crate::intrinsics::set_timeout).unwrap();
      let name = v8::String::new(context_scope, "setTimeout").unwrap().into();
      global_obj.set(context_scope, name, set_timeout.into());

      let log = v8::Function::new(context_scope, crate::intrinsics::log).unwrap();
      let name = v8::String::new(context_scope, "log").unwrap().into();
      global_obj.set(context_scope, name, log.into());
    }

    return v8::Global::new(handle_scope, context);
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

impl Drop for RuntimeData {
  fn drop(&mut self) {
    let RuntimeData { isolate, .. } = self;

    // Drop all the Modules from the map
    if let Some(module_map) = isolate.remove_slot::<ModuleMap>() {
      module_map.drain();
    }
  }
}

impl Runtime {
  pub fn new() -> Runtime {
    return Runtime {
      map: ModuleMap::new(),
      data: Box::leak(Box::new(RuntimeData::new())),

      waker_id: 0,
      waker_map: HashMap::new(),
      enqueue_id: 0,
      enqueue_map: HashMap::new(),
    };
  }

  pub fn module_from_file<'a>(
    &'a mut self,
    path: &str,
    reload: crate::module_map::Reload,
  ) -> Option<&'a mut Module> {
    let path = std::path::absolute(path).unwrap();
    let path = path.to_str().unwrap();
    let handle_scope = &mut self.context_scope();

    return self.map.get_module(handle_scope, path, reload);
  }

  pub fn context<'s>(&mut self, scope: &mut v8::HandleScope<'s, ()>) -> v8::Local<'s, v8::Context> {
    let data = get_data_mut!(self);
    return v8::Local::new(scope, data.context.clone());
  }

  pub fn waker_key<'s>(
    &mut self,
    scope: &mut v8::HandleScope<'s, ()>,
  ) -> v8::Local<'s, v8::Private> {
    let a = get_data_mut!(self);
    return v8::Local::new(scope, a.waker_key.clone());
  }

  pub fn handle_scope(&mut self) -> &'static mut v8::HandleScope<'static> {
    let data = get_data_mut!(self);
    let handle_scope = &mut *data.handle_scope;
    return fake_clone!({ handle_scope }, v8::HandleScope<'static>);
  }

  pub fn handle_scope_new<'a>(&mut self) -> v8::HandleScope<'a> {
    let data = get_data_mut!(self);
    let handle_scope = &mut v8::HandleScope::new(&mut data.isolate);

    let ctx = self.context(handle_scope);
    let data2 = get_data_mut!(self);
    return v8::HandleScope::with_context(&mut data2.isolate, ctx);
    // return fake_clone!({ handle_scope }, v8::HandleScope<'static>);
  }

  pub fn context_scope<'s>(&mut self) -> v8::ContextScope<'s, v8::HandleScope<'static>> {
    let handle_scope = self.handle_scope();
    let ctx = self.context(handle_scope);
    return v8::ContextScope::new(handle_scope, ctx);
  }

  pub fn make_global<T>(&mut self, local: v8::Local<T>) -> v8::Global<T> {
    let data = get_data_mut!(self);
    let isolate = &mut *data.isolate;
    return v8::Global::new(isolate, local);
  }

  // pub fn make_local<'s, T>(&mut self, global: v8::Global<T>) -> v8::Local<'s, T> {
  //   let data = get_data_mut!(self);
  //   let isolate = &mut *data.isolate;
  //   return v8::Local::new(isolate, global);
  // }

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

  pub fn get<'s>() -> &'s mut Self {
    return crate::prelude::RUNTIME.with_borrow_mut(|rt| {
      return fake_clone!({ rt.get_mut().unwrap() }, crate::runtime::Runtime);
    });
  }
}
