import init, { World, Direction } from 'snake_game';

const CELL_SIZE = 25;
const WORLD_WIDTH = 16;
let fps = 10;

init().then((_) => {
  const world = World.new(WORLD_WIDTH);
  const worldWidth = world.width();

  const canvas = <HTMLCanvasElement>document.getElementById('snake-canvas');
  const ctx = canvas.getContext('2d');

  canvas.height = worldWidth * CELL_SIZE;
  canvas.width = worldWidth * CELL_SIZE;

  document.addEventListener('keydown', (e) => {
    switch (e.code) {
      case 'ArrowUp':
      case 'KeyW':
        world.change_snake_dir(Direction.Up);
        break;
      case 'ArrowRight':
      case 'KeyD':
        world.change_snake_dir(Direction.Right);
        break;
      case 'ArrowDown':
      case 'KeyS':
        world.change_snake_dir(Direction.Down);
        break;
      case 'ArrowLeft':
      case 'KeyA':
        world.change_snake_dir(Direction.Left);
        break;
    }
  });

  const drawWorld = () => {
    ctx.beginPath();

    // 세로 줄
    for (let x = 0; x < worldWidth + 1; x++) {
      ctx.moveTo(CELL_SIZE * x, 0);
      ctx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE);
    }

    // 가로 줄
    for (let y = 0; y < worldWidth + 1; y++) {
      ctx.moveTo(0, CELL_SIZE * y);
      ctx.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y);
    }

    ctx.stroke();
  };

  const drawSnake = () => {
    const snakeIdx = world.snake_head_idx();

    ctx.beginPath();

    ctx.fillRect(
      (snakeIdx % worldWidth) * CELL_SIZE,
      Math.floor(snakeIdx / worldWidth) * CELL_SIZE,
      CELL_SIZE,
      CELL_SIZE
    );

    ctx.stroke();
  };

  const paint = () => {
    drawWorld();
    drawSnake();
  };

  const update = () => {
    setTimeout(() => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      world.update();
      paint();
      requestAnimationFrame(update);
    }, 1000 / fps);
  };

  paint();
  update();
});
