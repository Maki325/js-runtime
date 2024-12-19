use std::collections::HashMap;

use crate::{module_map::ModuleMap, Module};

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
}

#[derive(Debug)]
pub(crate) struct RuntimeData {
  pub(crate) isolate: v8::OwnedIsolate,
  pub(crate) context: v8::Global<v8::Context>,
}

impl RuntimeData {
  pub fn new() -> RuntimeData {
    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    isolate.set_host_import_module_dynamically_callback(dynamic_import);

    let context = Self::setup_context(&mut isolate);

    let mut runtime_data = RuntimeData { isolate, context };

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

      let set_timeout = v8::Function::new(context_scope, crate::intrinsics::set_timeout).unwrap();
      let global_obj = context.global(context_scope);
      let name = v8::String::new(context_scope, "setTimeout").unwrap().into();
      global_obj.set(context_scope, name, set_timeout.into());
    }

    return v8::Global::new(handle_scope, context);
  }
}

extern "C" fn promise_hook(
  promise_hook_type: v8::PromiseHookType,
  promise: v8::Local<'_, v8::Promise>,
  parent: v8::Local<'_, v8::Value>,
) {
  println!("PromiseHookType: {promise_hook_type:#?}, promise: {promise:#?}, parent: {parent:#?}");
  let handle_scope = &mut crate::prelude::handle_scope();
  println!(
    "In? PromiseHookType: {promise_hook_type:#?}, promise: {promise:#?}, parent: {parent:#?}"
  );
  let private_name = v8::String::new(handle_scope, "WakerKey").unwrap();
  let private = v8::Private::new(handle_scope, Some(private_name));

  let value = promise.get_private(handle_scope, private);
  println!("Value: {value:#?}");
  if let Some(value) = value {
    println!("Value some: {value:#?} | Is number: {}", value.is_number());
    if value.is_number() {
      println!("Value Number: {:#?}", value.number_value(handle_scope));
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

    // drop(isolate);
  }
}

impl Runtime {
  pub fn new() -> Runtime {
    return Runtime {
      map: ModuleMap::new(),
      data: Box::leak(Box::new(RuntimeData::new())),

      waker_id: 0,
      waker_map: HashMap::new(),
    };
  }

  pub fn module_from_file<'a>(&'a mut self, path: &str) -> Option<&'a mut Module> {
    // let handle_scope = &mut v8::HandleScope::new(&mut unsafe { &mut *self.data }.isolate);
    // handle_scope.set_slot(self.data);

    let path = std::path::absolute(path).unwrap();
    let path = path.to_str().unwrap();

    // let context = v8::Local::new(handle_scope, unsafe { &mut *self.data }.context.clone());

    return self.map.get_module(path);
  }

  pub fn with_handle_scope<R, F: Fn(&mut v8::HandleScope<'_, ()>, v8::Local<v8::Context>) -> R>(
    &mut self,
    f: F,
  ) -> R {
    let handle_scope = &mut v8::HandleScope::new(&mut unsafe { &mut *self.data }.isolate);
    let context = v8::Local::new(handle_scope, unsafe { &mut *self.data }.context.clone());
    return f(handle_scope, context);
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
}
