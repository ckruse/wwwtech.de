const path = require("path");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");

module.exports = (env, argv) => {
  const IS_PROD = argv.mode === "production";

  return {
    mode: "development",
    devtool: "inline-source-map",
    entry: "./src/index.js",
    output: {
      path: path.resolve(__dirname, "../static/"),
      filename: "app.js",
    },
    module: {
      rules: [
        {
          test: /\.s?css$/,
          use: [
            { loader: MiniCssExtractPlugin.loader },
            { loader: "css-loader" },
            { loader: "sass-loader" },
            { loader: "postcss-loader" },
          ],
        },
      ],
    },
    plugins: [
      new MiniCssExtractPlugin({
        filename: "app.css",
        chunkFilename: IS_PROD ? "[id].[chunkhash].css" : "[id].css",
      }),

      new CopyWebpackPlugin({
        patterns: [
          {
            context: "./static",
            from: "**/*",
            to: path.resolve(__dirname, "../static/"),
          },
        ],
      }),
    ],
  };
};
