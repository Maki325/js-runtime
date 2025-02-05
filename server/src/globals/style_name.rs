use runtime::v8;
use std::{
  cell::{OnceCell, RefCell},
  collections::HashMap,
};

runtime::ascii_str! {
  FRAMEWORK_JS_STYLE_NAME = "___FRAMEWORK_JS_STYLE_NAME___";
}

pub fn framework_js_style_name(
  handle_scope: &mut v8::HandleScope,
  args: v8::FunctionCallbackArguments,
  mut rv: v8::ReturnValue,
) {
  let item = args.get(0);
  if args.length() == 0 || !item.is_string() {
    return;
  }
  let str: &'static str = style_name(handle_scope, item);
  rv.set(
    v8::String::new_external_onebyte_static(handle_scope, str.as_bytes())
      .unwrap()
      .into(),
  );
}

pub fn style_name(handle_scope: &mut v8::HandleScope, item: v8::Local<v8::Value>) -> &'static str {
  let str = item.to_rust_string_lossy(handle_scope);

  return process_style_name(str);
}

thread_local! {
  pub(crate) static STYLE_NAME_CACHE: RefCell<OnceCell<HashMap<String, &'static str>>> = RefCell::new(OnceCell::new());
}

fn process_style_name(name: String) -> &'static str {
  return STYLE_NAME_CACHE.with_borrow_mut(|map| {
    map.get_or_init(|| HashMap::new());
    let map = map.get_mut().unwrap();

    if let Some(name) = map.get(&name) {
      return (*name) as &'static str;
    }

    if name.starts_with("--") {
      let value = escape_html(name.clone());
      map.insert(name.clone(), Box::new(value).leak());

      return map.get(&name).unwrap();
    }

    let value = escape_html(hyphenate_style_name(&name));
    map.insert(name.clone(), Box::new(value).leak());

    return map.get(&name).unwrap();
  });
}

/**
 * Reimplemented from React
 * https://github.com/facebook/react/blob/9defcd56bc3cd53ac2901ed93f29218007010434/packages/react-dom-bindings/src/shared/hyphenateStyleName.js#L26
 *
 * Hyphenates a camelcased CSS property name, for example:
 *
 *   > hyphenateStyleName('backgroundColor')
 *   < "background-color"
 *   > hyphenateStyleName('MozTransition')
 *   < "-moz-transition"
 *   > hyphenateStyleName('msTransition')
 *   < "-ms-transition"
 *
 * As Modernizr suggests (http://modernizr.com/docs/#prefixed), an `ms` prefix
 * is converted to `-ms-`.
 */
fn hyphenate_style_name<S: AsRef<str>>(name: S) -> String {
  let name = name.as_ref().chars().collect::<Vec<_>>();
  let mut vec = Vec::<char>::with_capacity(name.len() + 10);

  if name.starts_with(&['m', 's']) && name[2].is_ascii_uppercase() {
    vec.push('-');
  }

  for c in name {
    if c.is_ascii_uppercase() {
      vec.push('-');
      vec.push(c.to_ascii_lowercase());
    } else {
      vec.push(c);
    }
  }

  return vec.into_iter().collect();
}

fn escape_html(value: String) -> String {
  let mut vec = Vec::<char>::with_capacity(value.len() + 20);

  for c in value.chars() {
    match c {
      '"' => vec.extend_from_slice(&['&', 'q', 'u', 'o', 't', ';']),
      '&' => vec.extend_from_slice(&['&', 'a', 'm', 'p', ';']),
      '\'' => vec.extend_from_slice(&['&', '#', 'x', '2', '7', ';']),
      '<' => vec.extend_from_slice(&['&', 'l', 't', ';']),
      '>' => vec.extend_from_slice(&['&', 'g', 't', ';']),
      c => vec.push(c),
    }
  }

  return vec.into_iter().collect();
}
