import init, { World, Direction } from 'snake_game';

const CELL_SIZE = 25;
const WORLD_WIDTH = 16;
const INIT_SNAKE_LEN = 3;
let fps = 10;

init().then((wasm: { memory: { buffer: ArrayBufferLike } }) => {
  const world = World.new(WORLD_WIDTH, INIT_SNAKE_LEN);
  const worldWidth = world.width();

  const gameControlBtn = document.getElementById('game-control-btn');
  const gameStatus = document.getElementById('game-status');
  const canvas = <HTMLCanvasElement>document.getElementById('snake-canvas');
  const ctx = canvas.getContext('2d');

  canvas.height = worldWidth * CELL_SIZE;
  canvas.width = worldWidth * CELL_SIZE;

  gameControlBtn.addEventListener('click', () => {
    if (world.game_status()) {
      location.reload();
    } else {
      gameControlBtn.innerText = 'Restart';
      world.start_game();
      play();
    }
  });

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
    ctx.fillStyle = '#dbdb78';
    ctx.strokeStyle = '#78aadb';

    ctx.fillRect(0, 0, canvas.width, canvas.height);

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
    const snakeCells = new Uint32Array(
      wasm.memory.buffer,
      world.snake_cells(),
      world.snake_length()
    );

    snakeCells
      // .filter((cellIdx, i) => !(i > 0 && cellIdx === snakeCells[0]))
      .slice()
      .reverse()
      .forEach((cellIdx, i) => {
        ctx.fillStyle = i === snakeCells.length - 1 ? '#db78db' : '#db7878';

        ctx.beginPath();

        fillRect(cellIdx);
      });

    ctx.stroke();
  };

  const drawReward = () => {
    const idx = world.reward_cell();

    if (idx === 1000) {
      alert('Win!');
    }

    ctx.beginPath();
    ctx.fillStyle = '#7878db';
    fillRect(idx);

    ctx.stroke();
  };

  const fillRect = (idx: number) => {
    ctx.fillRect(
      (idx % worldWidth) * CELL_SIZE,
      Math.floor(idx / worldWidth) * CELL_SIZE,
      CELL_SIZE,
      CELL_SIZE
    );
  };

  const drawGameStatus = () => {
    gameStatus.textContent = world.game_status_text();
  };

  const paint = () => {
    drawWorld();
    drawSnake();
    drawReward();
    drawGameStatus();
  };

  const play = () => {
    setTimeout(() => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      world.step();
      paint();
      requestAnimationFrame(play);
    }, 1000 / fps);
  };

  paint();
});
