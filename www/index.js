async function init() {
  const importObject = {
    console: {
      log: () => {
        console.log('Just logging semething!');
      },
      error: () => {
        console.log('I am just error');
      },
    },
  };

  const res = await fetch('sum.wasm');
  const buffer = await res.arrayBuffer();
  const byteArray = new Int8Array(buffer);

  // debugger;
  const wasm = await WebAssembly.instantiate(byteArray.buffer, importObject);
  const sumFunction = wasm.instance.exports.sum;
  const result = sumFunction(100, 15);
  console.log(result);

  const wasmMemory = wasm.instance.exports.mem;
  console.dir(wasmMemory);

  const uint8Array = new Uint8Array(wasmMemory.buffer, 0, 2);

  const hiText = new TextDecoder().decode(uint8Array);
  console.log(hiText);
}

init();
