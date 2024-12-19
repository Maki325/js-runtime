use crate::Module;
use std::{collections::HashMap, path::Path};

pub type ModuleId = i32;

#[derive(Debug)]
pub(crate) struct ModuleMap {
  map: HashMap<ModuleId, Module>,
  path_to_module_id: HashMap<String, ModuleId>,
}

impl ModuleMap {
  pub fn new() -> ModuleMap {
    return ModuleMap {
      map: HashMap::new(),
      path_to_module_id: HashMap::new(),
    };
  }

  pub fn drain(mut self) {
    self.map.drain();
    self.path_to_module_id.drain();
  }

  pub fn get_module<'a, 'b>(&'a mut self, path: &str) -> Option<&'a mut Module>
  where
    'a: 'b,
  {
    println!("get_module 1!!!!");
    let module_exists = self.path_to_module_id.contains_key(path);

    println!("get_module 2!!!!");
    let handle_scope = &mut crate::prelude::handle_scope();
    println!("get_module 3!!!!");

    let stuff = if module_exists {
      self
        .map
        .get_mut(self.path_to_module_id.get(path).unwrap())
        .unwrap()
    } else {
      let (id, global_module) = {
        let specifier = v8::String::new(handle_scope, path).unwrap();

        let source = v8::String::new(
          handle_scope,
          std::fs::read_to_string(path).unwrap().as_str(),
        )
        .unwrap();

        let script_origin = v8::ScriptOrigin::new(
          handle_scope,
          specifier.cast(),
          0,
          0,
          false,
          0,
          None,
          false,
          false,
          true,
          None,
        );

        let source = &mut v8::script_compiler::Source::new(source, Some(&script_origin));

        let module = v8::script_compiler::compile_module(handle_scope, source).unwrap();

        let id = module.script_id().unwrap();

        (id, v8::Global::new(handle_scope, module))
      };

      let module = Module::new(path.to_owned(), global_module.clone());

      self.map.insert(id, module);
      self.path_to_module_id.insert(path.to_owned(), id);

      {
        let local_module = v8::Local::new(handle_scope, global_module);

        let instantiated = local_module
          .instantiate_module(handle_scope, Self::resolve_module)
          .unwrap();

        if !instantiated {
          let mut lock = std::io::stderr().lock();
          use std::io::Write;
          writeln!(lock, "Couldn't instantiate module \"{path}\"").unwrap();
          return None;
        }

        local_module.evaluate(handle_scope).unwrap();
      }

      self.map.get_mut(&id).unwrap()
    };

    return Some(stuff);
  }

  fn resolve_module<'s>(
    ctx: v8::Local<'s, v8::Context>,
    specifier: v8::Local<'s, v8::String>,
    _import_assertions: v8::Local<'s, v8::FixedArray>,
    referrer: v8::Local<'s, v8::Module>,
  ) -> Option<v8::Local<'s, v8::Module>> {
    println!("HEREE 1!");
    let module_map = &mut crate::prelude::get_runtime().map;
    println!("HEREE 2!");
    // let module_map = unsafe { &mut *(*ctx.get_slot::<*mut Self>().unwrap()) };
    let mut scope = unsafe { v8::CallbackScope::new(ctx) };
    println!("HEREE 3!");

    let path = specifier.to_rust_string_lossy(&mut scope);

    let path = {
      let mut base_path = Path::new(
        &module_map
          .map
          .get(&referrer.script_id().unwrap())
          .unwrap()
          .path,
      )
      .parent()
      .unwrap()
      .to_owned();

      base_path.push(path);

      std::path::absolute(base_path).unwrap()
    };

    let path = path.to_str().unwrap();

    let already_exists = module_map.path_to_module_id.contains_key(path);

    if already_exists {
      let module = {
        module_map
          .map
          .get(module_map.path_to_module_id.get(path).unwrap())
          .unwrap()
          .module
          .clone()
      };
      return Some(v8::Local::new(&mut scope, module));
    }

    println!("HERE 4!!!!!");
    let module = module_map.get_module(&path);
    println!("HERE 5!!!!!");

    let mut ctx = unsafe { v8::CallbackScope::new(ctx) };
    return module.map(|m| v8::Local::new(&mut ctx, m.module.clone()));
  }
}
