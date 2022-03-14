import init, { greet, greet_alert, add, get_now } from "rust-wasm-test";

init().then((_) => {
  const value = greet("Akiyama");
  console.log(`Answer is: ${value}`);
  console.log("hello");
  greet_alert("Akiyama");

  let n1 = 10;
  let n2 = 20;
  console.log(`${n1} + ${n2} = ${add(n1, n2)}`);
  console.log("Now is ", get_now());
});
