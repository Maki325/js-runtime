use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use runtime::{v8, Runtime};
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod globals;

runtime::startup!(start);
// runtime::startup!(testing);

#[allow(unused)]
async fn testing() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let rt = Runtime::get();
  add_globals(rt);

  let promise = {
    let module = rt
      .module_from_file("./app/test.js", runtime::Reload::No)
      .unwrap();
    let function = module.get_function("default").unwrap();

    let handle_scope = &mut runtime::prelude::handle_scope();

    let page_fn = v8::Local::new(handle_scope, function.clone());
    let this = v8::null(handle_scope);

    let returned_value = page_fn
      .call(handle_scope, this.cast(), &[])
      .ok_or("\"default\" function returned an exception!")
      .unwrap();

    println!(
      "returned_value.type_repr(): {:#?} {:#?}",
      returned_value.type_repr(),
      returned_value.to_rust_string_lossy(handle_scope),
    );

    let promise = v8::Local::<v8::Promise>::try_from(returned_value).unwrap();
    runtime::futures::Promise { promise }
  };

  let promise = promise.await;

  let _response = {
    let handle_scope = &mut runtime::prelude::handle_scope();

    let promise = v8::Local::new(handle_scope, promise.clone());
    let state = promise.state();

    match state {
      v8::PromiseState::Rejected => {
        let promise_result = promise.result(handle_scope);

        let error_msg = promise_result
          .to_string(handle_scope)
          .unwrap()
          .to_rust_string_lossy(handle_scope);

        panic!("{error_msg}");
      }
      v8::PromiseState::Fulfilled => {
        let promise_result = promise.result(handle_scope);

        let response = promise_result
          .to_string(handle_scope)
          .unwrap()
          .to_rust_string_lossy(handle_scope);

        response
      }
      v8::PromiseState::Pending => {
        unreachable!("Pending???");
      }
    }
  };

  println!("_response: {_response:#?}");

  return Ok(());
}

#[allow(unused)]
async fn shutdown_signal() {
  // Wait for the CTRL+C signal
  tokio::signal::ctrl_c()
    .await
    .expect("failed to install CTRL+C signal handler");
}

#[allow(unused)]
async fn start() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  {
    let rt = runtime::Runtime::get();
    add_globals(rt);
  }

  let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

  let listener = TcpListener::bind(addr).await?;

  let http = http1::Builder::new();
  let graceful = hyper_util::server::graceful::GracefulShutdown::new();
  let mut signal = std::pin::pin!(shutdown_signal());

  loop {
    tokio::select! {
      Ok((stream, _addr)) = listener.accept() => {
        println!("Connected! {_addr:#?}");
        let io = TokioIo::new(stream);
        let conn = http.serve_connection(io, service_fn(hello));
        let fut = graceful.watch(conn);
        tokio::spawn(async move {
          if let Err(e) = fut.await {
            eprintln!("Error serving connection: {:?}", e);
          }
        });
      },

      _ = &mut signal => {
        eprintln!("graceful shutdown signal received");
        break;
      }
    }
  }

  tokio::select! {
    _ = graceful.shutdown() => {
      eprintln!("all connections gracefully closed");
    },
    _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
      eprintln!("timed out wait for all connections to close");
    }
  }

  Ok(())
}

fn add_globals(rt: &mut runtime::Runtime) {
  rt.add_global_fn(
    globals::stringify::FRAMEWORK_JS_STRINGIFY,
    globals::stringify::framework_js_stringify,
  );
  rt.add_global_fn(
    globals::style_name::FRAMEWORK_JS_STYLE_NAME,
    globals::style_name::framework_js_style_name,
  );
  rt.add_global_fn(
    globals::style_value::FRAMEWORK_JS_STYLE_VALUE,
    globals::style_value::framework_js_style_value,
  );
  rt.add_global_fn(
    globals::style_object::FRAMEWORK_JS_STYLE_OBJECT,
    globals::style_object::framework_js_style_object,
  );

  rt.add_global_fn(globals::test::TEST, globals::test::test);
}

#[allow(unused)]
async fn hello(
  _req: Request<hyper::body::Incoming>,
) -> Result<
  Response<http_body_util::combinators::BoxBody<Bytes, std::io::Error>>,
  Box<dyn std::error::Error + Send + Sync>,
> {
  let promise = {
    let rt = Runtime::get();
    let module = rt
      .module_from_file("./app/page.js", runtime::Reload::Yes)
      .unwrap();
    let function = module.get_function("default").unwrap();

    let handle_scope = rt.handle_scope();
    let tc = &mut v8::TryCatch::new(handle_scope);
    let page_fn = v8::Local::new(tc, function.clone());
    let this = v8::null(tc);

    let returned_value = page_fn
      .call(tc, this.cast(), &[])
      .ok_or("\"default\" function returned an exception!")
      .unwrap();

    let exception = tc.exception();

    if returned_value.is_promise() {
      let promise = v8::Local::<v8::Promise>::try_from(returned_value).unwrap();
      runtime::futures::Promise { promise }
    } else {
      return Err("Not a promise!")?;
    }
  };

  let (obj, receiver, sender_id, value, function) = {
    let promise = promise.await;

    let handle_scope = runtime::prelude::handle_scope();
    let promise = v8::Local::new(handle_scope, promise);

    let result = promise.result(handle_scope);
    if !result.is_array() {
      return Err("Not an array!")?;
    }

    let result = v8::Local::<v8::Array>::try_from(result).unwrap();
    if result.length() != 2 {
      return Err("Array length not 2!")?;
    }

    let value = result.get_index(handle_scope, 0).unwrap();
    if !value.is_string() {
      return Err("Element at index 0 is not a string!")?;
    }
    let value = value.to_rust_string_lossy(handle_scope);

    let function = result.get_index(handle_scope, 1).unwrap();
    if !function.is_function() {
      return Err("Element at index 1 is not a function!")?;
    }
    let function =
      v8::Local::<v8::Function>::try_from(result.get_index(handle_scope, 1).unwrap()).unwrap();
    let function = SendGlobal(v8::Global::new(handle_scope, function));

    let (obj, receiver, sender_id) = get_enquable(handle_scope);
    (obj, receiver, sender_id, value, function)
  };

  {
    let sender = runtime::Runtime::get().get_sender(sender_id);
    sender.unwrap().send(Ok(value)).await.unwrap();
  }

  let receiver_stream = tokio_stream::wrappers::ReceiverStream::new(receiver);

  let stream_body = http_body_util::StreamBody::new(futures_util::TryStreamExt::map_ok(
    futures_util::TryStreamExt::map_ok(receiver_stream, |s| Bytes::from(s)),
    hyper::body::Frame::data,
  ));
  let boxed_body = http_body_util::BodyExt::boxed(stream_body);

  let response = Response::builder()
    .status(hyper::StatusCode::OK)
    .body(boxed_body)
    .unwrap();

  tokio::spawn(async move {
    let function = function;
    let f = function.0;
    let obj = obj;
    let obj = obj.0;

    let resp = {
      let handle_scope = runtime::prelude::handle_scope();
      let f = v8::Local::new(handle_scope, f);
      let obj = v8::Local::new(handle_scope, obj);

      let val = f.call(handle_scope, obj.into(), &[obj.into()]).unwrap();
      if !val.is_promise() {
        panic!("Not a promise!");
      }
      runtime::futures::Promise {
        promise: v8::Local::<v8::Promise>::try_from(val).unwrap(),
      }
    };

    resp.await;
    drop(runtime::Runtime::get().remove_sender(sender_id));
  });

  return Ok(response);
}

struct SendGlobal<T>(v8::Global<T>);
unsafe impl<T> Send for SendGlobal<T> {}

fn get_enquable<'s>(
  handle_scope: &mut v8::HandleScope<'s>,
) -> (
  SendGlobal<v8::Object>,
  tokio::sync::mpsc::Receiver<std::io::Result<String>>,
  u32,
) {
  let obj = v8::Object::new(handle_scope);

  {
    let key = v8::String::new(handle_scope, "enqueue").unwrap();
    let value = v8::Function::new(handle_scope, enqueue).unwrap();
    obj.set(handle_scope, key.into(), value.into());
  }

  let (sender, receiver) = tokio::sync::mpsc::channel::<std::io::Result<String>>(16);
  let sender_id = runtime::Runtime::get().insert_sender(sender);
  {
    let key = v8::String::new(handle_scope, "sender_id").unwrap();
    let value = v8::Number::new(handle_scope, sender_id as f64);
    obj.set(handle_scope, key.into(), value.into());
  }

  return (
    SendGlobal(v8::Global::new(handle_scope, obj)),
    receiver,
    sender_id,
  );
}

fn enqueue(
  handle_scope: &mut v8::HandleScope,
  args: v8::FunctionCallbackArguments,
  _rv: v8::ReturnValue,
) {
  let this = args.this();
  if !this.is_object() {
    return;
  }

  let key = v8::String::new(handle_scope, "sender_id").unwrap();
  let sender_id = match this.get(handle_scope, key.into()) {
    Some(sender_id) => {
      if sender_id.is_number() {
        match sender_id.number_value(handle_scope) {
          Some(sender_id) => sender_id as u32,
          None => return,
        }
      } else {
        return;
      }
    }
    None => return,
  };

  let data = args.get(0);
  if !data.is_string() {
    return;
  }
  if let Some(sender) = runtime::Runtime::get().get_sender(sender_id) {
    let value = data.to_rust_string_lossy(handle_scope);
    tokio::spawn(async {
      sender.clone().send(Ok(value)).await.unwrap();
    });
  }
}
