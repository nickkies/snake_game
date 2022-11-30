async function init() {
  const memory = new WebAssembly.Memory({ initial: 1 });

  const importObject = {
    js: {
      mem: memory,
    },
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

  console.log(memory);
  debugger;
  await WebAssembly.instantiate(byteArray.buffer, importObject);
  console.log(memory);

  const uint8Array = new Uint8Array(memory.buffer, 0, 2);
  const hiText = new TextDecoder().decode(uint8Array);

  console.log(hiText);
}

init();
