use crate::{error::ModuleGetFunctionError, Result};

#[derive(Debug)]
pub struct Module {
  pub(crate) path: String,
  pub module: v8::Global<v8::Module>,
}

impl Module {
  pub(crate) fn new(path: String, module: v8::Global<v8::Module>) -> Module {
    return Module { path, module };
  }

  pub fn get_function(
    &self,
    handle_scope: &mut v8::HandleScope<'_>,
    name: &str,
  ) -> Result<v8::Global<v8::Function>> {
    let module = v8::Local::new(handle_scope, self.module.clone());

    let module_namespace_obj = v8::Local::<v8::Object>::try_from(module.get_module_namespace())
      .map_err(|e| ModuleGetFunctionError::DataError(e).top())?;

    let source =
      v8::String::new(handle_scope, name).ok_or(ModuleGetFunctionError::StringConversion.top())?;

    let page_fn_value = module_namespace_obj
      .get(handle_scope, source.cast())
      .ok_or(ModuleGetFunctionError::NoFunction.top())?;

    if !page_fn_value.is_function() {
      return Err(ModuleGetFunctionError::NotFunction.top())?;
    }

    return Ok(v8::Global::new(
      handle_scope,
      v8::Local::<v8::Function>::try_from(page_fn_value).unwrap(),
    ));
  }
}
