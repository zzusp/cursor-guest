import { defineConfig } from 'vite';

// https://vitejs.dev/config/
export default defineConfig({
  // 防止Vite遮挡Rust错误
  clearScreen: false,
  // Tauri需要固定端口，如果端口不可用则失败
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 在Windows下使用polling可以改善文件监视
      usePolling: process.platform === 'win32',
      ignored: ['**/node_modules/**', '**/dist/**'],
    },
  },
  // 使用`TAURI_DEBUG`和其他环境变量
  // https://tauri.app/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    // Tauri支持es2021
    target: ['es2021', 'chrome100', 'safari13'],
    // 调试构建不压缩
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    // 为调试构建生成sourcemaps
    sourcemap: !!process.env.TAURI_DEBUG,
  },
}); 