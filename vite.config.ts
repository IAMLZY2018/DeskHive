import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { copyFileSync } from 'fs'
import { resolve } from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    // 自定义插件：复制设置页面到构建目录
    {
      name: 'copy-settings-html',
      writeBundle() {
        try {
          copyFileSync(
            resolve(__dirname, 'settings.html'),
            resolve(__dirname, 'dist/settings.html')
          )
          console.log('✓ 设置页面已复制到构建目录')
        } catch (error) {
          console.warn('⚠ 复制设置页面失败:', error)
        }
      }
    }
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    }
  }
})
