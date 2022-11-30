async function init() {
  const res = await fetch('sum.wasm');
  const buffer = await res.arrayBuffer();
  const byteArray = new Int8Array(buffer);

  // debugger;
  const wasm = await WebAssembly.instantiate(byteArray.buffer);
  const sumFunction = wasm.instance.exports.sum;
  const result = sumFunction(100, 15);

  console.log(result);
}

init();
