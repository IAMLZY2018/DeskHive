import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { copyFileSync } from 'fs'
import { resolve } from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    // 复制设置页面
    {
      name: 'copy-settings-html',
      writeBundle() {
        try {
          // 复制构建后的正确文件
          copyFileSync(
            resolve(__dirname, 'dist/settings-simple.html'),
            resolve(__dirname, 'dist/settings.html')
          )
          console.log('✓ 设置页面已正确复制')
        } catch (error) {
          console.warn('⚠ 复制设置页面失败:', error)
        }
      }
    }
  ],
  build: {
    rollupOptions: {
      input: {
        main: resolve(__dirname, 'index.html'),
        settings: resolve(__dirname, 'settings-simple.html')
      },
      output: {
        entryFileNames: 'assets/[name]-[hash].js',
        chunkFileNames: 'assets/[name]-[hash].js',
        assetFileNames: 'assets/[name]-[hash].[ext]'
      }
    }
  },
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    }
  }
})
