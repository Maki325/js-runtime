mod error;
mod intrinsics;
mod module;
mod module_map;
mod result;
mod runtime;

pub use v8;
pub mod futures;
pub mod prelude;

pub use error::Error;
pub use module::Module;
pub use result::Result;
pub use runtime::Runtime;

pub fn init() {
  let platform = v8::new_default_platform(0, false).make_shared();
  v8::V8::initialize_platform(platform);
  v8::V8::initialize();
}

pub fn dispose() {
  unsafe {
    v8::V8::dispose();
  }
  v8::V8::dispose_platform();
}

#[cfg(test)]
mod tests {
  type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

  #[test]
  fn get_exported_module_values() -> Result<()> {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    {
      let isolate = &mut v8::Isolate::new(v8::CreateParams::default());
      let isolate_scope = &mut v8::HandleScope::new(isolate);
      let context = v8::Context::new(isolate_scope, Default::default());
      let mut context_scope = v8::ContextScope::new(isolate_scope, context);

      let source = r#"
        export function randomNumber() {
          // Chosen at random
          return 35;
        }
        export const yo = "Yo!";
      "#;

      let source = v8::String::new(&mut context_scope, &source)
        .ok_or("Couldn't conert source to v8::String!")?;

      let main_module_name = v8::String::new(&mut context_scope, "main")
        .ok_or("Couldn't conert \"main\" module name to v8::String!")?;
      let script_origin = v8::ScriptOrigin::new(
        &mut context_scope,
        main_module_name.cast(),
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

      let module = v8::script_compiler::compile_module(&mut context_scope, source)
        .ok_or("Couldn't compile module!")?;

      let instantiated = module
        .instantiate_module(
          &mut context_scope,
          |_ctx: v8::Local<'_, v8::Context>,
           _name: v8::Local<'_, v8::String>,
           _idk: v8::Local<'_, v8::FixedArray>,
           _module: v8::Local<'_, v8::Module>| None,
        )
        .ok_or("Couldn't instantiate module!")?;

      assert!(instantiated, "Module not instantiated!");

      module
        .evaluate(&mut context_scope)
        .ok_or("Couldn't evaluate module!")?;

      let module_namespace = module.get_module_namespace();
      assert!(
        module_namespace.is_object(),
        "Value returned from module.get_module_namespace is NOT an object!"
      );

      let module_namespace_obj = v8::Local::<v8::Object>::try_from(module.get_module_namespace())?;

      let own_property_names = module_namespace_obj
        .get_own_property_names(
          &mut context_scope,
          v8::GetPropertyNamesArgs {
            ..Default::default()
          },
        )
        .ok_or("Couldn't get own property names for module namespace!")?;
      assert!(
        own_property_names.length() == 2,
        "own_property_names does NOT have 2 elements!"
      );

      {
        let key = v8::Number::new(&mut context_scope, 0.0).cast();
        let value = own_property_names
          .get(&mut context_scope, key)
          .ok_or("Couldn't get value from own_property_names at index 0!")?;

        assert!(
          value.to_rust_string_lossy(&mut context_scope) == "randomNumber",
          "Key at index 0 is not \"randomNumber\"!"
        );

        let random_number_value = module_namespace_obj
          .get(&mut context_scope, value)
          .ok_or("Couldn't get value from module_namespace_obj with key \"randomNumber\"!")?;

        assert!(
          random_number_value.is_function(),
          "random_number_value is not a function!"
        );

        let random_number_fn = v8::Local::<v8::Function>::try_from(random_number_value).unwrap();

        let this = v8::null(&mut context_scope);
        let returned_value = random_number_fn
          .call(&mut context_scope, this.cast(), &[])
          .ok_or("\"randomNumber\" function returned an exception!")?;

        assert!(
          returned_value.is_number(),
          "\"randomNumber\" function didn't return a number"
        );
        let returned_value_number = returned_value
          .number_value(&mut context_scope)
          .ok_or("Could NOT convert returned_value to a number!")?;
        assert!(
          returned_value_number == 35.0,
          "returned_value_number is not correct!"
        );
      }

      {
        let key = v8::Number::new(&mut context_scope, 1.0).cast();
        let value = own_property_names
          .get(&mut context_scope, key)
          .ok_or("Couldn't get value from own_property_names at index 1!")?;

        assert!(
          value.to_rust_string_lossy(&mut context_scope) == "yo",
          "Key at index 1 is not \"yo\"!"
        );

        let yo_value = module_namespace_obj
          .get(&mut context_scope, value)
          .ok_or("Couldn't get value from module_namespace_obj with key \"yo\"!")?;

        assert!(yo_value.is_string(), "random_number_value is not a string!");

        assert!(
          yo_value.to_rust_string_lossy(&mut context_scope) == "Yo!",
          "yo_value not correct!"
        );
      }
    }

    unsafe {
      v8::V8::dispose();
    }
    v8::V8::dispose_platform();

    return Ok(());
  }
}
