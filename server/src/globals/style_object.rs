use super::{style_name, style_value};
use runtime::v8;

pub const FRAMEWORK_JS_STYLE_OBJECT: runtime::FastStaticString =
  runtime::ascii_str!("___FRAMEWORK_JS_STYLE_OBJECT___");

pub fn framework_js_style_object(
  handle_scope: &mut v8::HandleScope,
  args: v8::FunctionCallbackArguments,
  mut rv: v8::ReturnValue,
) {
  let style_obj = args.get(0);
  if args.length() != 1 || !style_obj.is_object() {
    return;
  }

  let style_obj = style_obj.to_object(handle_scope).unwrap();

  let keys = style_obj
    .get_own_property_names(handle_scope, v8::GetPropertyNamesArgs::default())
    .unwrap();

  let mut string_style = String::new();
  for i in 0..keys.length() {
    let key = match keys.get_index(handle_scope, i) {
      Some(key) => key,
      None => continue,
    };
    let value = match style_obj.get(handle_scope, key) {
      Some(value) => value,
      None => continue,
    };

    let value = style_value::style_value(handle_scope, value, key);
    let key = style_name::style_name(handle_scope, key);
    if !string_style.is_empty() {
      string_style.push(';');
    }
    string_style.push_str(&key);
    string_style.push(':');
    string_style.push_str(&value);
  }

  let string_style = v8::String::new(handle_scope, &string_style).unwrap();
  rv.set(string_style.into());
}

// globalThis.___FRAMEWORK_JS_STYLE_OBJECT___ = (style) => {
//   return Object.entries(style).map(([key, value]) =>
//     `${globalThis.___FRAMEWORK_JS_STYLE_NAME___(key)}: ${globalThis.___FRAMEWORK_JS_STYLE_VALUE___(value, key)}`
//   ).join(';');
// }
