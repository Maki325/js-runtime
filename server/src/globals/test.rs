use runtime::v8;

runtime::ascii_str! {
  TEST = "__test__";
}

pub fn test(
  handle_scope: &mut v8::HandleScope,
  args: v8::FunctionCallbackArguments,
  mut _rv: v8::ReturnValue,
) {
  let item = args.get(0);
  if !item.is_array() {
    return;
  }
  let item = v8::Local::<v8::Array>::try_from(args.get(0)).unwrap();

  let test = TEST.v8_string(handle_scope);
  println!("Len before! {:#?}", item.length());
  item.set_index(handle_scope, item.length(), test.into());
  println!("Len After! {:#?}", item.length());
  println!("Len before! {:#?}", item.length());
  item.set_index(handle_scope, item.length(), test.into());
  println!("Len After! {:#?}", item.length());

  _rv.set(v8::Number::new(handle_scope, 420.69).into());
}
