use std::cell::{OnceCell, RefCell};

thread_local! {
  pub(crate) static RUNTIME: RefCell<OnceCell<crate::Runtime>> = RefCell::new(OnceCell::new());
}

pub fn handle_scope() -> &'static mut v8::HandleScope<'static> {
  return RUNTIME.with_borrow_mut(|rt| rt.get_mut().unwrap().handle_scope());
}

pub fn setup() {
  RUNTIME.with(|rt| {
    if let Some(_) = rt.borrow().get() {
      return;
    }
    rt.borrow_mut().set(crate::Runtime::new()).unwrap();
  });
}

#[macro_export]
macro_rules! startup {
  ($main:path) => {
    fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
      runtime::init();

      runtime::prelude::setup();
      // let result = tokio::runtime::Builder::new_multi_thread()
      //   .on_thread_start(|| runtime::prelude::setup())
      let result = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on({ $main() });

      runtime::dispose();

      return result;
    }
  };
}
