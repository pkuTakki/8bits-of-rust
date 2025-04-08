const { defineConfig } = require("@vue/cli-service")
module.exports = defineConfig({
  transpileDependencies: true,
  lintOnSave: false,
  configureWebpack: (config) => {
    config.experiments = {
      asyncWebAssembly: true // 启用异步WebAssembly支持
    }
  },
})