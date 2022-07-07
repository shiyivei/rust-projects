//配置文件，引入依赖
const path = require("path");
const CopyWebpackPlugin = require("copy-webpack-plugin");

//导出

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "public"),
    filename: "bootstrap.js",
  },

  mode: "development",

  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
  },

  plugins: [
    new CopyWebpackPlugin({ patterns: [{ from: "./index.html", to: "./" }] }),
  ],
};
