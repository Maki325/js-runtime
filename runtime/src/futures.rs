pub struct Promise<'s> {
  pub promise: v8::Local<'s, v8::Promise>,
}

unsafe impl Send for Promise<'_> {}

impl<'s> std::future::Future for Promise<'s> {
  type Output = v8::Local<'s, v8::Promise>;
  fn poll(
    self: std::pin::Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Self::Output> {
    let promise = self.promise;
    let callback_scope = &mut unsafe { v8::CallbackScope::new(promise) };

    if promise.state() != v8::PromiseState::Pending {
      return std::task::Poll::Ready(self.promise);
    }

    let rt = crate::Runtime::get();

    let waker_key = rt.waker_key(callback_scope);
    let id = match promise.get_private(callback_scope, waker_key) {
      Some(val) => {
        if val.is_number() {
          Some(val.to_number(callback_scope).unwrap().value() as u32)
        } else {
          None
        }
      }
      None => None,
    };

    let should_set_id = id.is_none();
    let id = rt.insert_waker(cx.waker().clone(), id);
    if should_set_id {
      let promise = v8::Local::new(callback_scope, self.promise.clone());
      let value = v8::Number::new(callback_scope, id as f64);

      promise.set_private(callback_scope, waker_key, value.into());
    }

    return std::task::Poll::Pending;
  }
}
