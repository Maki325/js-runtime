export default function Test() {
  let arr = [];
  log(arr);
  log(arr.length);
  const b = globalThis.__test__(arr);
  log(arr);
  log(arr.length);
  return b;
}

export function Test2() {
  let arr = [];
  const b = globalThis.___FRAMEWORK_JS_STRINGIFY___(["aaa", () => { }], arr);
  log(arr);
  return b;
}
