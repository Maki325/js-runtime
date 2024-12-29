use runtime::v8;

pub const FRAMEWORK_JS_STRINGIFY: runtime::FastStaticString =
  runtime::ascii_str!("___FRAMEWORK_JS_STRINGIFY___");

const ERROR: runtime::FastStaticString =
  runtime::ascii_str!("Objects are not valid as a JSX child!");

pub fn framework_js_stringify(
  handle_scope: &mut v8::HandleScope,
  args: v8::FunctionCallbackArguments,
  mut rv: v8::ReturnValue,
) {
  let item = args.get(0);
  if item.is_array() {
    if args.length() < 2 {
      return;
    }
    let to_create = v8::Local::<v8::Array>::try_from(args.get(1)).unwrap();

    let item = v8::Local::<v8::Array>::try_from(item).unwrap();
    let str = item.get_index(handle_scope, 0);
    let fun = item.get_index(handle_scope, 1);
    if item.length() == 2 && str.unwrap().is_string() && fun.unwrap().is_function() {
      to_create.set_index(handle_scope, to_create.length(), fun.unwrap());
      rv.set(str.unwrap());
      return;
    }

    let mut output = String::new();

    for i in 0..item.length() {
      let value = match item.get_index(handle_scope, i) {
        Some(item) => item,
        None => continue,
      };
      if value.is_array() {
        let value = v8::Local::<v8::Array>::try_from(value).unwrap();
        let str = value.get_index(handle_scope, 0);
        let fun = value.get_index(handle_scope, 1);
        if value.length() == 2 && str.unwrap().is_string() && fun.unwrap().is_function() {
          to_create.set_index(handle_scope, to_create.length(), fun.unwrap());
          output.push_str(&str.unwrap().to_rust_string_lossy(handle_scope));
          continue;
        }
      }
      output.push_str(&str.unwrap().to_rust_string_lossy(handle_scope));
    }

    rv.set(v8::String::new(handle_scope, &output).unwrap().into());
    return;
  } else if item.is_null() || item.is_undefined() {
    rv.set(v8::String::empty(handle_scope).into());
    return;
  } else if item.is_object() {
    let err = ERROR.v8_string(handle_scope).into();
    handle_scope.throw_exception(err);
    return;
  } else {
    rv.set(item);
    return;
  }
}
