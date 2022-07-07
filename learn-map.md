# 1. 具备前端基础

1. 错误处理
2. wasm、yew
3. tauri桌面应用
4. wrap（这个桌面非常适合前端）
5. webGPU（风险比较大，但研究技术无所谓）

# 2. 无前端基础

1. 基础算法
2. 错误处理和async/await
3. 命令行工具（可选）
4. wrap或者是rocket
5. 用reqwest写微服务或者爬虫
6. tonic写gRPC
7. benchmark

# 3. 注意点

Rust用于提升前端性能是主流

招聘往往会提到C++

unsafe并不是非常的Rust

Rust可能在云计算和微服务方面会有较大的发展

# 3. Rust框架

Flask：小而美、自由灵活

Django：大而全，适合企业级应用，快速开发

Rocket 框架最受欢迎，只要不是逻辑错误；强烈推荐，安全、高速迭代

Actix-web：对性能有极端要求的场景，rust只要实现了async和await的性能都非常的OK

Wrap：对性能有一定要求，小而美的场景，需求灵活，大量中间件

Axum：现在不推荐

# 4. WebAssembly Game

## 4.1 WebAssembly 介绍

在浏览器中取代Javascript来进行计算密集型应用的方式

webassembly二进制文件配合前端三件套进行运行

webassembly应用从场景：前端、动画演示、游戏

容器（docker）、虚拟机

区块链

**各个编程语言对webassembly的支持**

Rust：Debug工具还需完善,最适合

不要用C/C++

其他语言都有运行时性能问题

## **4.2 wasm与wat**

WebAssembly .wasm

.wat

调试

创建webassembly项目

```
cargo new --lib wasm_game
在根目录创建www
```

```
进入并创建npm项目
npm init -y
```

```
npm install --save webpack webpack-cli //安装依赖
```

```
npm install --save-dev webpack-dev-server
```

```
npm install --save copy-webpack-plugin
```

在www下创建public文件夹

再创建两个index文件，并做引入

```
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Document</title>
  </head>
  <body>
    <h1>WebAssembly Game</h1>
    <script src="./index.js"></script>
  </body>
</html>
```

写配置文件

```
//配置文件，引入依赖
const path = require("path");
const CopyWebpackPlugin = require("copy-webpack-plugin");

//导出

module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "public"),
    filename: "index.js",
  },

  mode: "development",

  plugins: [
    new CopyWebpackPlugin({ patterns: [{ from: "./index.html", to: "./" }] }),
  ],
};
```

写个命令

```
"scripts": {
    "dev":"webpack-dev-server", //npm run dev启动即可
    "build":"webpack build", //npm run build 启动即可
    "test": "echo \"Error: no test specified\" && exit 1"
  },
```

现在build 之后可以运行dev，查看网页能够正常看到内容

**下载wasm文件，导入项目中并运行**

异步加载

```
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
```

## 4.3 WebAssembly 中的Rust与JS交互

### 4.3.1 生成wasm文件，导入npm项目

```
cargo install wasm-pack
```

配置cargo.toml文件

```
[package]
name = "wasm_game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
wasm-bindgen = "0.2.78"
[lib]
crate-type = ["cdylib"]

[package.metadata.wasm-pack.release]
wasm-opt = false
```

编写rust代码

```
use wasm_bindgen::prelude::*; //js和rust交互的包

//在rust中调用js,必须说明是外部的方法
#[wasm_bindgen]
extern "C" {

    pub fn alert(s: &str);

}

#[wasm_bindgen] //宏用来标识是webassembly
pub fn hello(name: &str) {
    //在此处调用
    alert(name);
}
```

生成wasm文件

```
wasm-pack build --target web
```

修改package文件

```
"wasm_game":"file:../pkg",
```

注释js所有内容，引入并调用

```

//引入
import init, { hello } from "wasm_game";

//调用
init().then(() => {
  hello("shiyivei");
  console.log("Ok");
});
```

运行前端文件

```
npm run dev
```

在浏览器查看结果

小结：rust写好代码以后，用wasm-pack编译成二进制文件，然后前端项目在js文件中引入调用，大概是这个过程

### 4.3.2 weealloc 内存分配与bootstrap.js捕获错误

**weealoc**

是一个轻量的wasm分配器

```
use wasm_bindgen::prelude::*; //js和rust交互的包
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

//在rust中调用js,必须说明是外部的方法
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen] //宏用来标识是webassembly
pub fn hello(name: &str) {
    //在此处调用
    alert(name);
}
```

```
wasm-pack build --target web //查看是否编译正常
```

使用bootstrap.js

```
import("./index.js").catch((e) => {
  console.error("Error: ", e);
});
```

改一下配置文件以及html索引文件

```
bootstrap
```

运行

```
http://127.0.0.1:8080/
```

依旧能够正常运行

## 4.4 创建画布

```
<div class="content-wrapper">
      <canvas id="snake-world"> </canvas>
</div>
```

创建样式

```
 <style>
      .content-wrapper {
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        position: absolute;
        display: flex;
        align-items: center;
        justify-content: center;
        flex-direction: column;
      }
    </style>
```

编写rust代码

```
use wasm_bindgen::prelude::*; //js和rust交互的包
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

//在rust中调用js,必须说明是外部的方法
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen] //宏用来标识是webassembly
pub fn hello(name: &str) {
    //在此处调用
    alert(name);
}

#[wasm_bindgen]
struct World {
    width: usize,
    size: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize) -> Self {
        Self {
            width,
            size: width * width,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
}
```

生成wasm

```
wasm-pack build --target web
```

引入并调用

```
//引入
import init, { World } from "wasm_game";

//调用
init().then(() => {
  const CELL_SIZE = 20;
  const world = World.new(36);
  const worldWidth = world.width();

  const canvas = document.getElementById("snake-world");
  const context = canvas.getContext("2d");

  canvas.width = worldWidth * CELL_SIZE;
  canvas.height = worldWidth * CELL_SIZE;

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
  1;

  drawWorld();
});
```

## 4.5 创建蛇头

 编写rust代码

编译

```
wasm-pack build --target web
```

画蛇

```

  function drawSnake() {
    const snake_index = world.snake_head_index();
    const row = Math.floor(snake_index / worldWidth);
    const col = snake_index % worldWidth;

    context.beginPath();
    context.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    context.stroke();
  }

  function draw() {
    drawWorld();
    drawSnake();
  }

  function run() {
    setTimeout(() => {
      context.clearRect(0, 0, canvas.width, canvas.height);
      world.update();
      draw();
      requestAnimationFrame(run);
    }, 1000 / fps);
  }

  draw();
  run();
```

## 4.6 WebAssembly 与TS的交互

安装TS

```
npm install typescript
npm install --save typescript ts-loader
```

做一些切换流程，具体步骤见原子视频

## 4.7 控制蛇头

增加参数

```
pub fn new(width: usize, snake_index: usize) -> Self {
        Self {
            width,
            size: width * width,
            snake: Snake::new(snake_index),
        }
    }
```

编译为wasm

```
wasm-pack build --target web
```

随机化初始位置

```
  const CELL_SIZE = 20;
  const fps = 2;
  const WORLD_WIDTH = 8;
  const snakeIndex = Date.now() % (WORLD_WIDTH * WORLD_WIDTH);
  const world = World.new(WORLD_WIDTH, snakeIndex);
  const worldWidth = world.width();
```

编写rust代码并编译

监听事件

## 4.8 生成蛋

更新蛇头生成的随机数源

即时总结：rust代码 - wasm - ts/js - 网页运行

## 4.9 蛇吞蛋

流程上都是一样的要注意的就是仔细到每一个细节

## 4.10 设定游戏状态

增加状态枚举	