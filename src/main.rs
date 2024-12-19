fn main() {
  runtime::init();

  {
    let mut runtime = runtime::Runtime::new();
    // let module = runtime.module_from_file("./test.js");
    let module = runtime.module_from_file("./page.js");
    println!("Module final: {module:#?}");
    // if let Some(module) = module {
    //   module.run_test();
    // }
  }

  runtime::dispose();
}
