import { a } from './test.js';

export default function Page() {
  // setTimeout(() => resolve(), 5000)
  // return setTimeout(Page, 5);
  // return someString;
  return new Promise((resolve) => setTimeout(() => resolve("Whaaaa"), 100));
  // return Promise.resolve("Yoooo");
  // return a();
}
