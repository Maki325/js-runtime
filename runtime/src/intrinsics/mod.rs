use std::ops::{Deref, DerefMut};

pub fn set_timeout(
  handle_scope: &mut v8::HandleScope,
  args: v8::FunctionCallbackArguments,
  _rv: v8::ReturnValue,
) {
  // TODO: Return an positive integer that can be used in clearTimeout - https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout

  match args.length() {
    0 => {
      let message = v8::String::new(
        handle_scope,
        "The \"callback\" argument must be of type function. Received undefined",
      )
      .unwrap();
      let exception = v8::Exception::type_error(handle_scope, message);
      handle_scope.throw_exception(exception);
      return;
    }
    1 => {
      let message = v8::String::new(
        handle_scope,
        "The \"timeout\" argument must be of type number. Received undefined",
      )
      .unwrap();
      let exception = v8::Exception::type_error(handle_scope, message);
      handle_scope.throw_exception(exception);
      return;
    }
    _ => {}
  }

  let callback = args.get(0);

  if !callback.is_function() {
    let message = v8::String::new(
      handle_scope,
      &format!(
        "The \"callback\" argument must be of type function. Received {}",
        callback.type_repr()
      ),
    )
    .unwrap();
    let exception = v8::Exception::type_error(handle_scope, message);
    handle_scope.throw_exception(exception);
    return;
  }

  let timeout = args.get(1);

  if !timeout.is_number() {
    let message = v8::String::new(
      handle_scope,
      &format!(
        "The \"timeout\" argument must be of type function. Received {}",
        timeout.type_repr()
      ),
    )
    .unwrap();
    let exception = v8::Exception::type_error(handle_scope, message);
    handle_scope.throw_exception(exception);
    return;
  }

  let callback = v8::Local::<v8::Function>::try_from(callback).unwrap();
  let timeout = timeout.to_number(handle_scope).unwrap();

  let callback = v8::Global::new(handle_scope, callback);
  let callback = Box::into_pin(unsafe {
    Box::from_raw(Box::leak(Box::new(callback)) as *mut v8::Global<v8::Function> as *mut usize)
  });
  // let callback = std::pin::Pin::new(a);
  let timeout = timeout.value() as u64;

  tokio::spawn(async move {
    tokio::time::sleep(std::time::Duration::from_millis(timeout)).await;
    println!("HERE!!!");
    let handle_scope = crate::prelude::handle_scope();
    // let callback = callback.clone().0 as *mut v8::Global<v8::Function>;
    let mut callback = callback;
    let callback = callback.deref_mut() as *mut usize as *mut v8::Global<v8::Function>;
    println!("HERE 222!!!");
    let callback = unsafe { &mut *callback };
    println!("HERE 333!!!");
    let callback = v8::Local::new(handle_scope, callback.clone());
    println!("HERE 444!!!");

    let this = v8::null(handle_scope);
    println!("HERE 555!!!");

    let stack_trace = v8::StackTrace::current_stack_trace(handle_scope, 128).unwrap();
    println!("HERE 666!!!");
    println!("Trace len: {:#?}", stack_trace.get_frame_count());
    println!("HERE 777!!!");
    for i in 0..stack_trace.get_frame_count() {
      let frame = stack_trace.get_frame(handle_scope, i).unwrap();
      println!(
        "{:#?}:{}:{}",
        frame.get_script_name(handle_scope),
        frame.get_line_number(),
        frame.get_column()
      );
    }

    let tc = &mut v8::TryCatch::new(handle_scope);
    callback.call(tc, this.into(), &[]);
    // println!("tc.exception(): {:#?}", tc.exception());
    if let Some(exception) = tc.exception() {
      println!("tc.exception(): {:#?}", exception.type_repr());
      let obj = v8::Local::<v8::Object>::try_from(exception).unwrap();
      println!("obj: {obj:#?}");
      let stack = v8::String::new(tc, "stack").unwrap().into();
      let stack = obj.get(tc, stack).unwrap();
      println!(
        "stack: {stack:#?} {} {:#?}",
        stack.type_repr(),
        stack.to_rust_string_lossy(tc)
      );
      let cause = v8::String::new(tc, "cause").unwrap().into();
      let cause = obj.get(tc, cause).unwrap();
      println!(
        "cause: {cause:#?} {} {:#?}",
        cause.type_repr(),
        cause.to_rust_string_lossy(tc)
      );

      let stack_trace = v8::Exception::get_stack_trace(tc, exception).unwrap();
      println!("stack_trace: {stack_trace:#?}",);

      println!("Trace len: {:#?}", stack_trace.get_frame_count());
      for i in 0..stack_trace.get_frame_count() {
        let frame = stack_trace.get_frame(tc, i).unwrap();
        println!(
          "{:#?}:{}:{}",
          frame.get_script_name(tc),
          frame.get_line_number(),
          frame.get_column()
        );
      }
      panic!("A");

      // v8::Exception::R
    }
    println!("AFTER!");
  });
}

#[derive(Clone)]
pub struct A(*mut dyn std::any::Any);
unsafe impl Send for A {}
