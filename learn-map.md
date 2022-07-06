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

