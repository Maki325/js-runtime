use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use runtime::v8;
use std::net::SocketAddr;
use tokio::net::TcpListener;

// runtime::startup!(start);
runtime::startup!(testing);

async fn testing() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let rt = runtime::prelude::get_runtime();
  let promise = {
    let module = rt.module_from_file("./app/page.js").unwrap();
    let function = module.get_function("default").unwrap();

    let handle_scope = &mut runtime::prelude::handle_scope();

    let page_fn = v8::Local::new(handle_scope, function.clone());
    let this = v8::null(handle_scope);

    let returned_value = page_fn
      .call(handle_scope, this.cast(), &[])
      .ok_or("\"default\" function returned an exception!")
      .unwrap();

    println!("Is promise: {}", returned_value.is_promise());

    let promise = v8::Local::<v8::Promise>::try_from(returned_value).unwrap();
    println!("Promise state: {:#?}", promise.state());
    runtime::futures::Promise {
      promise: v8::Global::new(handle_scope, promise),
    }
  };

  let promise = promise.await;

  let _response = {
    let handle_scope = &mut runtime::prelude::handle_scope();

    let promise = v8::Local::new(handle_scope, promise.clone());
    let state = promise.state();
    println!("State: {:#?}", state);

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
        println!("Res: {response:#?}");

        response
      }
      v8::PromiseState::Pending => {
        panic!("Pending???");
      }
    }
  };

  // tokio::spawn(tokio::time::timeout(
  //   std::time::Duration::from_secs(2),
  //   async {
  //     println!("A");
  //   },
  // ));

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
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

  let listener = TcpListener::bind(addr).await?;

  let http = http1::Builder::new();
  let graceful = hyper_util::server::graceful::GracefulShutdown::new();
  let mut signal = std::pin::pin!(shutdown_signal());

  loop {
    tokio::select! {
      Ok((stream, _addr)) = listener.accept() => {
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
  // let response = runtime::prelude::with_runtime(|rt| {
  //   let module = rt.module_from_file("./app/page.js").unwrap();
  //   let function = module.get_function("default").unwrap();

  //   rt.with_handle_scope(|handle_scope, context| {
  //     let page_fn = v8::Local::new(handle_scope, function.clone());
  //     let this = v8::null(handle_scope);

  //     let context_scope = &mut v8::ContextScope::new(handle_scope, context);

  //     let returned_value = page_fn
  //       .call(context_scope, this.cast(), &[])
  //       .ok_or("\"default\" function returned an exception!")
  //       .unwrap();

  //     returned_value
  //       .to_string(context_scope)
  //       .unwrap()
  //       .to_rust_string_lossy(context_scope)
  //   })
  // });

  // Ok(Response::new(Full::new(Bytes::from(response))))
  Ok(Response::new(Full::new(Bytes::from(""))))
}
