import init from 'snake_game';

init().then((wasm) => {
  wasm.greet(123);
});
