import init, { greet, greet_alert } from "rust-wasm-test";

init().then((_) => {
  const value = greet("Akiyama");
  console.log(`Answer is: ${value}`);
  console.log("hello");
  greet_alert("Akiyama");
});
