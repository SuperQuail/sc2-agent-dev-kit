import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

// Tauri 的 devUrl 写死了 5184，所以端口不能让 vite 自动换。
export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: { port: 5184, strictPort: true },
  // WebView2 是 Chromium，直接按它来，不向下兼容老浏览器。
  build: { target: "chrome105", outDir: "dist", emptyOutDir: true },
});
