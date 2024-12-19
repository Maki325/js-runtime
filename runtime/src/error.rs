use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
  #[error(transparent)]
  Module(#[from] ModuleError),
}

#[derive(ThisError, Debug)]
pub enum ModuleError {
  #[error(transparent)]
  GetFunction(#[from] ModuleGetFunctionError),
}

impl ModuleError {
  pub(crate) fn top(self) -> Error {
    return Error::Module(self);
  }
}

#[derive(ThisError, Debug)]
pub enum ModuleGetFunctionError {
  #[error("Data Error")]
  DataError(#[from] v8::DataError),
  #[error("Couldn't conert string to v8::String")]
  StringConversion,
  #[error("No function with that name")]
  NoFunction,
  #[error("Not a function")]
  NotFunction,
}

impl ModuleGetFunctionError {
  pub(crate) fn top(self) -> Error {
    return ModuleError::GetFunction(self).top();
  }
}
