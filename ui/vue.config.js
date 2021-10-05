module.exports = {
  publicPath: "/public",
  devServer: {
    port: 8090,
    proxy: {
      '^/api': {
          target: "http://localhost:8080/api",
          changeOrigin: true,
          pathRewrite: { "^/api/": "" }
      }
    }
  }
};