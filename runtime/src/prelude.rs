use std::cell::{OnceCell, RefCell};

use crate::Runtime;

thread_local! {
  pub(crate) static RUNTIME: RefCell<OnceCell<Runtime>> = RefCell::new(OnceCell::new());
}

// pub fn with_runtime<T, F: FnOnce(&mut Runtime) -> T>(f: F) -> T {
//   println!("with_runtime");
//   RUNTIME.with_borrow_mut(|rt| f(rt.get_mut().unwrap()))
// }

// pub fn with_runtime2<T, F: FnOnce(&mut Runtime) -> T>(f: F) -> Result<T, ()> {
//   // RUNTIME.try_with(|rt| f(rt.get_mut().unwrap())).unwrap()
//   RUNTIME.with(|cell| {
//     match &mut cell.try_borrow_mut() {
//       Ok(rt) => Ok(f(rt.get_mut().unwrap())),
//       Err(e) => {
//         println!("Err: {e:#?}");
//         Err(())
//       }
//     }
//     // if let  else {
//     //   println!("err?");
//     //   Err(())
//     // }
//   })
//   // if let Ok(_) = RUNTIME.try_with(|_| {}) {
//   //   Ok(RUNTIME.with_borrow_mut(|rt| f(rt.get_mut().unwrap())))
//   // } else {
//   //   Err(())
//   // }
// }

macro_rules! get_data_mut {
  ($rt:ident) => {
    unsafe { &mut *((($rt.data as *mut usize).clone()) as *mut crate::runtime::RuntimeData) }
  };
}
pub(crate) use get_data_mut;

macro_rules! get_data {
  ($rt:ident) => {
    unsafe { &mut *((($rt.data as *const usize).clone()) as *const crate::runtime::RuntimeData) }
  };
}
pub(crate) use get_data;

macro_rules! fake_clone {
  ($data:tt, $t:ty) => {
    unsafe { &mut *(((($data as *mut $t) as *mut usize).clone()) as *mut $t) }
  };
}
pub(crate) use fake_clone;

pub fn get_runtime<'s>() -> &'s mut Runtime {
  return RUNTIME.with_borrow_mut(|rt| {
    return fake_clone!({ rt.get_mut().unwrap() }, crate::runtime::Runtime);
  });
}

pub fn handle_scope() -> &'static mut v8::HandleScope<'static> {
  return RUNTIME.with_borrow_mut(|rt| {
    let rt = rt.get_mut().unwrap();
    // let data = rt.get_data_mut();
    let data = get_data_mut!(rt);
    let a = &mut *data.handle_scope;
    return fake_clone!({ a }, v8::HandleScope<'static>);
  });
}

pub fn setup() {
  RUNTIME.with(|rt| {
    if let Some(_) = rt.borrow().get() {
      return;
    }
    rt.borrow_mut().set(Runtime::new()).unwrap();
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
