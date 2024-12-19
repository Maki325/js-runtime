pub struct Promise {
  pub promise: v8::Global<v8::Promise>,
}

impl std::future::Future for Promise {
  type Output = v8::Global<v8::Promise>;
  fn poll(
    self: std::pin::Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Self::Output> {
    let result = {
      let (result, id) = {
        let handle_scope = crate::prelude::handle_scope();
        let promise = v8::Local::new(handle_scope, self.promise.clone());

        if promise.state() == v8::PromiseState::Pending {
          // {
          //   let waker_2 = cx.waker().clone();
          //   // let waker_2 = std::sync::Arc::new(cx.waker().clone());
          //   let then_wake = move |_handle_scope: &mut v8::HandleScope,
          //                         _args: v8::FunctionCallbackArguments,
          //                         _rv: v8::ReturnValue| {
          //     println!("Called!");
          //     // let a = waker_2.clone();
          //     // a.wake();
          //     // A
          //   };
          //   // context_scope.set_slot(value);
          //   // context_scope.set_promise_hook(hook);

          //   // SOOOOO
          //   // Every time the promise is polled
          //   // We add the Promise and the Waker to a map
          //   // That way, when we get a set_promise_hook with type Resolve
          //   // We can tell that Waker to wake up

          //   let then_wake = v8::Function::new(context_scope, then_wake).unwrap();
          //   // then_wake.
          //   // then_wake.set_private(context_scope, v8::Private::new(), value)
          //   // let global_obj = context.global(context_scope);
          //   // let name = v8::String::new(context_scope, "setTimeout").unwrap().into();
          //   // global_obj.set(context_scope, name, set_timeout.into());
          //   promise.then(context_scope, then_wake);
          // }
          // v8::Local::<v8::Function>

          let private_name = v8::String::new(handle_scope, "WakerKey").unwrap();
          let private = v8::Private::new(handle_scope, Some(private_name));

          // let context_scope = &mut v8::ContextScope::new(handle_scope, context);
          // if let Some(val) = promise.get_private(context_scope, private) {
          //   // rt.insert_waker(
          //   //   cx.waker().clone(),
          //   //   Some(val.to_number(context_scope).unwrap().value() as usize),
          //   // );
          // } else {
          //   // None
          //   // let value = v8::Number::new(handle_scope, 5 as f64);
          //   // promise.set_private(context_scope, private, value.into());
          // }
          let id = promise
            .get_private(handle_scope, private)
            .map(|val| val.to_number(handle_scope).unwrap().value() as u32);

          (std::task::Poll::Pending, id)
        } else {
          (std::task::Poll::Ready(self.promise.clone()), None)
        }
      };

      if let std::task::Poll::Pending = result {
        let should_set_id = id.is_none();
        let rt = crate::prelude::get_runtime();
        let id = rt.insert_waker(cx.waker().clone(), id);
        if should_set_id {
          let handle_scope = crate::prelude::handle_scope();
          let promise = v8::Local::new(handle_scope, self.promise.clone());
          let private_name = v8::String::new(handle_scope, "WakerKey").unwrap();
          let private = v8::Private::new(handle_scope, Some(private_name));
          let value = v8::Number::new(handle_scope, id as f64);

          promise.set_private(handle_scope, private, value.into());
        }
      }

      result
    };

    println!("Result! {result:#?}");

    // if let std::task::Poll::Pending = result {
    //   cx.waker().wake_by_ref();
    // }

    return result;
  }
}
