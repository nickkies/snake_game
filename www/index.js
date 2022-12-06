import init, { World } from 'snake_game';

init().then((_) => {
  const CELL_SIZE = 25;

  const world = World.new();
  const worldWidth = world.width();

  const canvas = document.getElementById('snake-canvas');
  const ctx = canvas.getContext('2d');

  canvas.height = worldWidth * CELL_SIZE;
  canvas.width = worldWidth * CELL_SIZE;

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
    }, 100);
  };

  paint();
  update();
});
