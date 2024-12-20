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
  let callback = A(Box::leak(Box::new(callback)) as *mut dyn std::any::Any);
  let timeout = timeout.value() as u64;

  tokio::spawn(async move {
    tokio::time::sleep(std::time::Duration::from_millis(timeout)).await;
    let handle_scope = crate::prelude::handle_scope();
    let callback = callback.clone().0 as *mut v8::Global<v8::Function>;
    let callback = unsafe { &mut *callback };
    let callback = v8::Local::new(handle_scope, callback.clone());

    let this = v8::null(handle_scope);
    callback.call(handle_scope, this.into(), &[]);
  });
}

#[derive(Clone)]
pub struct A(*mut dyn std::any::Any);
unsafe impl Send for A {}
