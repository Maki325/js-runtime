use runtime::v8;

runtime::ascii_str! {
  FRAMEWORK_JS_STYLE_VALUE = "___FRAMEWORK_JS_STYLE_VALUE___";
}

pub fn framework_js_style_value(
  handle_scope: &mut v8::HandleScope,
  args: v8::FunctionCallbackArguments,
  mut rv: v8::ReturnValue,
) {
  let style_v8_value = args.get(0);
  let style_v8_name = args.get(1);
  if args.length() != 2 || !style_v8_name.is_string() {
    return;
  }

  let value = style_value(handle_scope, style_v8_value, style_v8_name);
  let value = v8::String::new(handle_scope, &value).unwrap();
  rv.set(value.into());
}

pub fn style_value(
  handle_scope: &mut v8::HandleScope,
  style_value: v8::Local<v8::Value>,
  style_name: v8::Local<v8::Value>,
) -> String {
  let style_name = style_name.to_rust_string_lossy(handle_scope);
  let style_value_string = style_value.to_rust_string_lossy(handle_scope);

  if !style_name.starts_with("--") && style_value.is_number() {
    let style_value_num = style_value.number_value(handle_scope).unwrap();
    if style_value_num != 0.0 && !is_unitless_number(&style_name) {
      return format!("{style_value_string}px");
    } else {
      return style_value_string;
    }
  }

  if style_value.is_number() || style_value.is_boolean() || style_value.is_big_int() {
    return style_value_string;
  }

  return escape_html(style_value_string);
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

const UNITLESS_NUMBERS: &'static [&'static str] = &[
  "animationIterationCount",
  "aspectRatio",
  "borderImageOutset",
  "borderImageSlice",
  "borderImageWidth",
  "boxFlex",
  "boxFlexGroup",
  "boxOrdinalGroup",
  "columnCount",
  "columns",
  "flex",
  "flexGrow",
  "flexPositive",
  "flexShrink",
  "flexNegative",
  "flexOrder",
  "gridArea",
  "gridRow",
  "gridRowEnd",
  "gridRowSpan",
  "gridRowStart",
  "gridColumn",
  "gridColumnEnd",
  "gridColumnSpan",
  "gridColumnStart",
  "fontWeight",
  "lineClamp",
  "lineHeight",
  "opacity",
  "order",
  "orphans",
  "scale",
  "tabSize",
  "widows",
  "zIndex",
  "zoom",
  "fillOpacity", // SVG-related properties
  "floodOpacity",
  "stopOpacity",
  "strokeDasharray",
  "strokeDashoffset",
  "strokeMiterlimit",
  "strokeOpacity",
  "strokeWidth",
  "MozAnimationIterationCount", // Known Prefixed Properties
  "MozBoxFlex",                 // TODO: Remove these since they shouldn't be used in modern code
  "MozBoxFlexGroup",
  "MozLineClamp",
  "msAnimationIterationCount",
  "msFlex",
  "msZoom",
  "msFlexGrow",
  "msFlexNegative",
  "msFlexOrder",
  "msFlexPositive",
  "msFlexShrink",
  "msGridColumn",
  "msGridColumnSpan",
  "msGridRow",
  "msGridRowSpan",
  "WebkitAnimationIterationCount",
  "WebkitBoxFlex",
  "WebKitBoxFlexGroup",
  "WebkitBoxOrdinalGroup",
  "WebkitColumnCount",
  "WebkitColumns",
  "WebkitFlex",
  "WebkitFlexGrow",
  "WebkitFlexPositive",
  "WebkitFlexShrink",
  "WebkitLineClamp",
];

fn is_unitless_number(name: &str) -> bool {
  return UNITLESS_NUMBERS.iter().any(|a| a.eq(&name));
}
