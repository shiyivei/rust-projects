// console.log("webassembly game");
// async function run() {
//   const importObject = {
//     console: {
//       log: () => {
//         console.log("log.info!");
//       },
//       error: () => {
//         console.log("error info!");
//       },
//     },
//   };
//   const response = await fetch("jqq.wasm");
//   //1 buffer
//   const buffer = await response.arrayBuffer();
//   debugger;
//   //2 wasm
//   const wasm = await WebAssembly.instantiate(buffer, importObject);

//   //3 调用
//   const addFunction = wasm.instance.exports.add;
//   const result = addFunction(10, 20);
//   console.log(result);
// }

// run();

//引入
import init, { World, Direction, GameStatus } from "wasm_game";
import { random } from "./utils/random"; // ./同级目录下

//调用
init().then((wasm) => {
  const CELL_SIZE = 20;
  const WORLD_WIDTH = 8;
  const snakeIndex = random(WORLD_WIDTH * WORLD_WIDTH);
  const world = World.new(WORLD_WIDTH, snakeIndex);
  const worldWidth = world.width();
  const fps = 5;

  //通过ID拿到变量
  const gameStatus = document.getElementById("game-status");
  const gameControlBtn = document.getElementById("game-ccontrol-btn");

  const canvas = <HTMLCanvasElement>document.getElementById("snake-world");
  const context = canvas.getContext("2d");

  canvas.width = worldWidth * CELL_SIZE;
  canvas.height = worldWidth * CELL_SIZE;

  //给变量添加事件
  gameControlBtn.addEventListener("click", () => {
    const status = world.game_status();
    if (status == undefined) {
      gameControlBtn.textContent = "you playing..";
      world.start_game();
      run();
    } else {
      location.reload();
    }
  });

  document.addEventListener("keydown", (e) => {
    switch (e.code) {
      case "ArrowUp":
        world.change_snake_direction(Direction.Up);
        break;
      case "ArrowDown":
        world.change_snake_direction(Direction.Down);
        break;
      case "ArrowLeft":
        world.change_snake_direction(Direction.Left);
        break;
      case "ArrowRight":
        world.change_snake_direction(Direction.Right);
        break;
    }
  });

  function drawWorld() {
    context.beginPath();

    for (let x = 0; x < worldWidth + 1; x++) {
      context.moveTo(CELL_SIZE * x, 0);
      context.lineTo(CELL_SIZE * x, CELL_SIZE * worldWidth);
    }

    for (let y = 0; y < worldWidth + 1; y++) {
      context.moveTo(0, CELL_SIZE * y);
      context.lineTo(CELL_SIZE * worldWidth, CELL_SIZE * y);
    }
    context.stroke();
  }

  function drawSnake() {
    const snakeCells = new Uint32Array(
      wasm.memory.buffer,
      world.snake_cells(),
      world.snake_length()
    );

    console.log(world.snake_length);

    snakeCells
      .filter((cellIdx, i) => !(i > 0 && cellIdx == snakeCells[0]))
      .forEach((cellIndex, i) => {
        const col = cellIndex % worldWidth;
        const row = Math.floor(cellIndex / worldWidth);
        context.beginPath();
        context.fillStyle = i === 0 ? "#787878" : "#000000";
        context.fillRect(
          col * CELL_SIZE,
          row * CELL_SIZE,
          CELL_SIZE,
          CELL_SIZE
        );
      });
    context.stroke();
  }

  function drawReward() {
    const index = world.reward_cell();
    const row = Math.floor(index / worldWidth);
    const col = index % worldWidth;

    context.beginPath();
    //color
    context.fillStyle = "#FF0000";
    context.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    context.stroke();

    if (index === 123456789) {
      alert(" you won");
    }
  }
  function drawGameStatus() {
    gameStatus.textContent = world.game_status_info();
  }

  function draw() {
    drawWorld();
    drawSnake();
    drawReward();
    drawGameStatus();
  }

  function run() {
    const status = world.game_status();
    if (status === GameStatus.Won || status == GameStatus.Lost) {
      gameControlBtn.textContent = "Play again?";
      return;
    }
    setTimeout(() => {
      context.clearRect(0, 0, canvas.width, canvas.height);
      world.update();
      draw();
      requestAnimationFrame(run);
    }, 1000 / fps);
  }
  draw();
});
