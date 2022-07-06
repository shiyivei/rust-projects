console.log("webassembly game");
async function run() {
  const response = await fetch("syv.wasm");
  //1 buffer
  const buffer = await response.arrayBuffer();
  //2 wasm
  const wasm = await WebAssembly.instantiate(buffer);

  //3 调用
  const addTwoFunction = wasm.instance.exports.addTwo;
  const result = addTwoFunction(10, 20);
  console.log(result);
}

run();
