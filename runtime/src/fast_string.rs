// Copied from `deno_core` `core/fast_string.rs`

use std::{
  borrow::Borrow,
  fmt::{Debug, Display, Formatter},
  hash::Hash,
  ops::Deref,
};

static EMPTY_STRING: v8::OneByteConst = v8::String::create_external_onebyte_const("".as_bytes());

/// A static string that is compile-time checked to be ASCII and is stored in the
/// most efficient possible way to create V8 strings.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct FastStaticString {
  s: &'static v8::OneByteConst,
}

impl FastStaticString {
  pub const fn new(s: &'static v8::OneByteConst) -> Self {
    FastStaticString { s }
  }

  pub fn as_str(&self) -> &'static str {
    self.s.as_ref()
  }

  pub fn as_bytes(&self) -> &'static [u8] {
    self.s.as_ref()
  }

  #[doc(hidden)]
  pub const fn create_external_onebyte_const(s: &'static [u8]) -> v8::OneByteConst {
    v8::String::create_external_onebyte_const(s)
  }

  pub fn v8_string<'s>(&self, scope: &mut v8::HandleScope<'s>) -> v8::Local<'s, v8::String> {
    v8::String::new_from_onebyte_const(scope, self.s).unwrap()
  }

  pub const fn into_v8_const_ptr(&self) -> *const v8::OneByteConst {
    self.s as _
  }
}

impl From<&'static v8::OneByteConst> for FastStaticString {
  fn from(s: &'static v8::OneByteConst) -> Self {
    Self::new(s)
  }
}

impl From<FastStaticString> for *const v8::OneByteConst {
  fn from(val: FastStaticString) -> Self {
    val.into_v8_const_ptr()
  }
}

impl Hash for FastStaticString {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.as_str().hash(state)
  }
}

impl AsRef<str> for FastStaticString {
  fn as_ref(&self) -> &str {
    self.as_str()
  }
}

impl Deref for FastStaticString {
  type Target = str;
  fn deref(&self) -> &Self::Target {
    self.as_str()
  }
}

impl Borrow<str> for FastStaticString {
  fn borrow(&self) -> &str {
    self.as_str()
  }
}

impl Debug for FastStaticString {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Debug::fmt(self.as_str(), f)
  }
}

impl Default for FastStaticString {
  fn default() -> Self {
    FastStaticString { s: &EMPTY_STRING }
  }
}

impl PartialEq for FastStaticString {
  fn eq(&self, other: &Self) -> bool {
    self.as_bytes() == other.as_bytes()
  }
}

impl Eq for FastStaticString {}

impl Display for FastStaticString {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Include a fast string in the binary. This string is asserted at compile-time to be 7-bit ASCII for optimal
/// v8 performance.
///
/// This macro creates a [`FastStaticString`].
#[macro_export]
macro_rules! ascii_str_include {
  ($file:expr) => {{
    const STR: runtime::v8::OneByteConst = runtime::FastStaticString::create_external_onebyte_const(
      ::std::include_str!($file).as_bytes(),
    );
    let s: &'static runtime::v8::OneByteConst = &STR;
    runtime::FastStaticString::new(s)
  }};
}

/// Include a fast string in the binary from a string literal. This string is asserted at compile-time to be
/// 7-bit ASCII for optimal v8 performance.
///
/// This macro creates a [`FastStaticString`].
#[macro_export]
macro_rules! ascii_str {
  ($str:expr) => {{
    const C: runtime::v8::OneByteConst =
      runtime::FastStaticString::create_external_onebyte_const($str.as_bytes());
    runtime::FastStaticString::new(&C)
  }};
}
