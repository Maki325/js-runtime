use std::sync::Mutex;

// thread_local! {
//   pub(crate) static RUNTIME: RefCell<OnceCell<crate::Runtime>> = RefCell::new(OnceCell::new());
// }
pub(crate) static RUNTIME: Mutex<usize> = Mutex::new(0);

pub fn handle_scope() -> &'static mut v8::HandleScope<'static> {
  println!("handle_scope!!");
  return crate::Runtime::get().handle_scope();
}

pub async fn setup() {
  let rt = Box::leak(Box::new(crate::Runtime::new()));
  let ptr = ((rt as *mut crate::Runtime) as *mut usize) as usize;
  let mut lock = RUNTIME.lock().unwrap();
  println!("setup lock: {lock:#?}");
  println!("setup ptr: {ptr:#?}");
  _ = std::mem::replace(&mut *lock, ptr);
}

#[macro_export]
macro_rules! startup {
  ($main:path) => {
    fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
      runtime::init();

      let result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on({ _main() });

      runtime::dispose();

      return result;
    }

    async fn _main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
      runtime::prelude::setup().await;
      $main().await
    }
  };
}
