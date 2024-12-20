use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use runtime::{v8, Runtime};
use std::net::SocketAddr;
use tokio::net::TcpListener;

runtime::startup!(start);
// runtime::startup!(testing);

#[allow(unused)]
async fn testing() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let rt = Runtime::get();
  let promise = {
    let module = rt
      .module_from_file("./app/page.js", runtime::Reload::No)
      .unwrap();
    let function = module.get_function("default").unwrap();

    let handle_scope = &mut runtime::prelude::handle_scope();

    let page_fn = v8::Local::new(handle_scope, function.clone());
    let this = v8::null(handle_scope);

    let returned_value = page_fn
      .call(handle_scope, this.cast(), &[])
      .ok_or("\"default\" function returned an exception!")
      .unwrap();

    let promise = v8::Local::<v8::Promise>::try_from(returned_value).unwrap();
    runtime::futures::Promise {
      promise: v8::Global::new(handle_scope, promise),
    }
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

#[allow(unused)]
async fn hello(_req: Request<hyper::body::Incoming>) -> hyper::Result<Response<Full<Bytes>>> {
  enum Res {
    String(String),
    Promise(runtime::futures::Promise),
  }
  let response = {
    let rt = Runtime::get();
    let module = rt
      .module_from_file("./app/page.js", runtime::Reload::Yes)
      .unwrap();
    let function = module.get_function("default").unwrap();

    let handle_scope = rt.handle_scope();
    let page_fn = v8::Local::new(handle_scope, function.clone());
    let this = v8::null(handle_scope);

    let returned_value = page_fn
      .call(handle_scope, this.cast(), &[])
      .ok_or("\"default\" function returned an exception!")
      .unwrap();

    if returned_value.is_promise() {
      let promise = rt.make_global(v8::Local::<v8::Promise>::try_from(returned_value).unwrap());
      Res::Promise(runtime::futures::Promise { promise: promise })
    } else {
      Res::String(
        returned_value
          .to_string(handle_scope)
          .unwrap()
          .to_rust_string_lossy(handle_scope),
      )
    }
  };

  let response = match response {
    Res::String(s) => s,
    Res::Promise(promise) => {
      let promise = promise.await;

      let handle_scope = runtime::prelude::handle_scope();
      let promise = v8::Local::new(handle_scope, promise);

      promise
        .result(handle_scope)
        .to_string(handle_scope)
        .unwrap()
        .to_rust_string_lossy(handle_scope)
    }
  };

  Ok(Response::new(Full::new(Bytes::from(response))))
}
