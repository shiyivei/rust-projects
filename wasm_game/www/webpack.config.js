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
