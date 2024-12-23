pub struct Promise {
  pub promise: v8::Global<v8::Promise>,
}

unsafe impl Send for Promise {}

impl std::future::Future for Promise {
  type Output = v8::Global<v8::Promise>;
  fn poll(
    self: std::pin::Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Self::Output> {
    println!("Promise HERE!!!");
    let rt = crate::Runtime::get();
    println!("Promise HERE 2!!!!");

    let handle_scope = rt.handle_scope();
    let promise = v8::Local::new(handle_scope, self.promise.clone());

    if promise.state() != v8::PromiseState::Pending {
      return std::task::Poll::Ready(self.promise.clone());
    }

    let id = match promise.get_private(handle_scope, rt.waker_key()) {
      Some(val) => {
        if val.is_number() {
          Some(val.to_number(handle_scope).unwrap().value() as u32)
        } else {
          None
        }
      }
      None => None,
    };

    let should_set_id = id.is_none();
    let id = rt.insert_waker(cx.waker().clone(), id);
    if should_set_id {
      let handle_scope = crate::prelude::handle_scope();
      let promise = v8::Local::new(handle_scope, self.promise.clone());
      let value = v8::Number::new(handle_scope, id as f64);

      promise.set_private(handle_scope, rt.waker_key(), value.into());
    }

    return std::task::Poll::Pending;
  }
}
