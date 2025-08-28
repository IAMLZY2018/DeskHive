<template>
  <div class="container">
    <!-- 左侧边栏 -->
    <div class="sidebar">
      <div class="sidebar-header">
        <h1>设置</h1>
      </div>
      <div class="sidebar-menu">
        <button 
          v-for="(section, key) in sections" 
          :key="key"
          class="menu-item" 
          :class="{ active: activeSection === key }"
          @click="activeSection = key"
        >
          <span class="menu-item-icon">{{ section.icon }}</span>
          {{ section.name }}
        </button>
      </div>
    </div>

    <!-- 右侧内容区 -->
    <div class="content">
      <div class="content-header">
        <h2>{{ sections[activeSection]?.name || '设置' }}</h2>
      </div>

      <div class="content-body">
        <!-- 外观设置 -->
        <div v-if="activeSection === 'appearance'" class="setting-section">
          <div class="section-title">窗口外观</div>
          <div class="setting-group">
            <div class="setting-item">
              <div>
                <div class="setting-label">主窗口透明度</div>
                <div class="setting-description">调整主窗口的透明程度，不影响设置窗口</div>
              </div>
              <div class="setting-control">
                <input 
                  type="range" 
                  v-model="opacityValue" 
                  min="0.5" 
                  max="1" 
                  step="0.1"
                  @input="applyOpacityPreview"
                >
                <span class="range-value">{{ Math.round(settings.opacity * 100) }}%</span>
              </div>
            </div>
            <div class="setting-item">
              <div>
                <div class="setting-label">禁止拖动窗口</div>
                <div class="setting-description">禁用标题栏拖动功能，防止意外移动窗口</div>
              </div>
              <div class="setting-control">
                <div 
                  class="toggle-switch" 
                  :class="{ active: settings.disable_drag }" 
                  @click="settings.disable_drag = !settings.disable_drag"
                ></div>
              </div>
            </div>
          </div>
        </div>

        <!-- 行为设置 -->
        <div v-if="activeSection === 'behavior'" class="setting-section">
          <div class="section-title">启动行为</div>
          <div class="setting-group">
            <div class="setting-item">
              <div>
                <div class="setting-label">开机自启动</div>
                <div class="setting-description">系统启动时自动运行应用程序</div>
              </div>
              <div class="setting-control">
                <div 
                  class="toggle-switch" 
                  :class="{ active: settings.auto_start }" 
                  @click="settings.auto_start = !settings.auto_start"
                ></div>
              </div>
            </div>

          </div>
        </div>

        <!-- 关于页面 -->
        <div v-if="activeSection === 'about'" class="setting-section">
          <div class="section-title">应用信息</div>
          <div class="setting-group">
            <div class="setting-item">
              <div>
                <div class="setting-label">版本</div>
                <div class="setting-description">当前应用版本号</div>
              </div>
              <div class="setting-control">
                <span style="color: #6d6d70;">{{ appVersion }}</span>
              </div>
            </div>
            <div class="setting-item">
              <div>
                <div class="setting-label">构建时间</div>
                <div class="setting-description">应用构建日期</div>
              </div>
              <div class="setting-control">
                <span style="color: #6d6d70;">{{ new Date().toLocaleDateString() }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="content-footer">
        <button class="btn btn-secondary" @click="cancelSettings">取消</button>
        <button class="btn btn-primary" @click="saveSettings">保存设置</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'

interface AppSettings {
  opacity: number
  disable_drag: boolean
  auto_start: boolean
}

type SectionKey = 'appearance' | 'behavior' | 'about'

interface Section {
  name: string
  icon: string
}

const currentWindow = getCurrentWindow()

const activeSection = ref<SectionKey>('appearance')
const originalOpacity = ref(0.95)
const appVersion = ref('...')

const sections: Record<SectionKey, Section> = {
  appearance: { name: '外观', icon: '🎨' },
  behavior: { name: '行为', icon: '⚡' },
  about: { name: '关于', icon: 'ℹ️' }
}

const settings = reactive<AppSettings>({
  opacity: 0.95,
  disable_drag: false,
  auto_start: false
})

// 透明度的计算属性，确保始终为数字类型
const opacityValue = computed({
  get: () => settings.opacity,
  set: (value: string | number) => {
    settings.opacity = typeof value === 'string' ? parseFloat(value) : value
  }
})

// 实时预览透明度（只应用于主窗口）
async function applyOpacityPreview() {
  try {
    // 只对主窗口应用透明度，设置窗口保持不透明
    await invoke('apply_opacity', { opacity: parseFloat(settings.opacity.toString()) })
  } catch (error) {
    console.error('应用透明度预览失败:', error)
  }
}

// 恢复原始透明度（只应用于主窗口）
async function restoreOriginalOpacity() {
  try {
    // 只对主窗口恢复透明度，设置窗口保持不透明
    await invoke('apply_opacity', { opacity: originalOpacity.value })
    console.log('已恢复主窗口原始透明度:', originalOpacity.value)
  } catch (error) {
    console.error('恢复原始透明度失败:', error)
  }
}

// 取消设置
async function cancelSettings() {
  await restoreOriginalOpacity()
  await closeWindow()
}

// 保存设置
async function saveSettings() {
  try {
    console.log('开始保存设置:', settings)
    
    // 确保数据类型正确，避免字符串传递给需要数字的字段
    const settingsToSave = {
      opacity: typeof settings.opacity === 'string' ? parseFloat(settings.opacity) : settings.opacity,
      disable_drag: Boolean(settings.disable_drag),
      auto_start: Boolean(settings.auto_start)
    }
    
    console.log('转换后的设置数据:', settingsToSave)
    
    // 调用 Tauri 命令保存设置
    await invoke('save_app_settings', { settings: settingsToSave })
    console.log('设置保存成功')
    
    // 关闭设置窗口
    await closeWindow()
  } catch (error) {
    console.error('保存设置失败:', error)
    
    // 显示更详细的错误信息
    const errorMessage = error instanceof Error ? error.message : String(error)
    alert(`保存设置失败: ${errorMessage}\n\n请检查应用权限或重新启动应用。`)
  }
}

// 关闭窗口
async function closeWindow() {
  try {
    console.log('调用后端命令关闭设置窗口...')
    await invoke('close_settings_window')
    console.log('设置窗口关闭成功')
  } catch (error) {
    console.error('调用后端关闭命令失败:', error)
    
    // 备用方案：直接调用窗口的 close 方法
    try {
      await currentWindow.close()
      console.log('使用窗口 API 关闭成功')
    } catch (fallbackError) {
      console.error('备用关闭方法也失败:', fallbackError)
    }
  }
}

// 加载设置
async function loadSettings() {
  try {
    console.log('开始加载设置...')
    const loadedSettings = await invoke('load_app_settings') as AppSettings
    console.log('加载的设置:', loadedSettings)
    
    // 保存原始透明度值
    originalOpacity.value = loadedSettings.opacity
    
    // 应用设置到界面
    Object.assign(settings, loadedSettings)
    
    // 注意：不对设置窗口应用透明度，设置窗口保持不透明
    // 透明度设置只应用于主窗口（Todo窗口）
    
    console.log('设置加载完成，设置窗口保持不透明')
  } catch (error) {
    console.error('加载设置失败:', error)
    // 使用默认值，已经在 reactive 中设置
  }
}

// 加载应用版本
async function loadAppVersion() {
  try {
    console.log('开始加载应用版本...')
    const version = await invoke('get_app_version') as string
    appVersion.value = version
    console.log('应用版本加载完成:', version)
  } catch (error) {
    console.error('加载应用版本失败:', error)
    appVersion.value = '未知版本'
  }
}

// 组件挂载时加载设置和版本信息
onMounted(async () => {
  await Promise.all([
    loadSettings(),
    loadAppVersion()
  ])
})

</script>

<style scoped>
/* 与原 HTML 版本相同的样式 */
* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

.container {
  width: 100% !important;
  height: 100vh !important;
  background: #ffffff;
  display: flex !important;
  flex-direction: row !important;
  flex-wrap: nowrap !important;
  overflow: hidden !important;
  position: fixed !important;
  top: 0 !important;
  left: 0 !important;
}

.sidebar {
  width: 240px !important;
  min-width: 240px !important;
  max-width: 240px !important;
  height: 100vh !important;
  background: #f8f8f8;
  border-right: 1px solid #e5e5e5;
  display: flex !important;
  flex-direction: column !important;
  flex-shrink: 0 !important;
  flex-basis: 240px !important;
  overflow: hidden !important;
}

.sidebar-header {
  padding: 20px 16px 16px;
  border-bottom: 1px solid #e5e5e5;
  -webkit-app-region: drag;
  user-select: none;
  flex-shrink: 0;
}

.sidebar-header h1 {
  font-size: 22px;
  font-weight: 600;
  color: #000;
  margin: 0;
}

.sidebar-menu {
  flex: 1;
  padding: 8px 0;
  overflow-y: auto;
  overflow-x: hidden;
}

.menu-item {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  cursor: pointer;
  transition: background-color 0.2s;
  border: none;
  background: none;
  width: 100%;
  text-align: left;
  font-size: 15px;
  color: #000;
}

.menu-item:hover {
  background: #e8e8e8;
}

.menu-item.active {
  background: #007aff;
  color: #ffffff;
}

.menu-item-icon {
  width: 20px;
  height: 20px;
  margin-right: 12px;
  font-size: 16px;
}

.content {
  flex: 1 !important;
  display: flex !important;
  flex-direction: column !important;
  min-width: 0 !important;
  height: 100vh !important;
  overflow: hidden !important;
}

.content-header {
  padding: 20px 24px 16px;
  border-bottom: 1px solid #e5e5e5;
  -webkit-app-region: drag;
  user-select: none;
  flex-shrink: 0;
}

.content-header h2 {
  font-size: 20px;
  font-weight: 600;
  color: #000;
  margin: 0;
}

.content-body {
  flex: 1;
  padding: 24px;
  overflow-y: auto;
}

.setting-section {
  margin-bottom: 32px;
}

.setting-section:last-child {
  margin-bottom: 0;
}

.section-title {
  font-size: 13px;
  font-weight: 600;
  color: #6d6d70;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 8px;
}

.setting-group {
  background: #ffffff;
  border-radius: 10px;
  border: 1px solid #e5e5e5;
  overflow: hidden;
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid #e5e5e5;
  min-height: 44px;
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-label {
  font-size: 17px;
  color: #000;
  font-weight: 400;
}

.setting-description {
  font-size: 13px;
  color: #6d6d70;
  margin-top: 2px;
}

.setting-control {
  display: flex;
  align-items: center;
}

.toggle-switch {
  position: relative;
  width: 51px;
  height: 31px;
  background: #e5e5e5;
  border-radius: 16px;
  cursor: pointer;
  transition: background-color 0.3s;
}

.toggle-switch.active {
  background: #34c759;
}

.toggle-switch::after {
  content: '';
  position: absolute;
  top: 2px;
  left: 2px;
  width: 27px;
  height: 27px;
  background: #ffffff;
  border-radius: 50%;
  transition: transform 0.3s;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.toggle-switch.active::after {
  transform: translateX(20px);
}

.setting-control input[type="range"] {
  width: 120px;
  margin-right: 8px;
}

.setting-control select {
  padding: 8px 12px;
  border: 1px solid #e5e5e5;
  border-radius: 8px;
  background: #ffffff;
  color: #000;
  font-size: 17px;
  min-width: 120px;
}

.range-value {
  font-size: 17px;
  color: #007aff;
  font-weight: 500;
  min-width: 40px;
  text-align: right;
}

.content-footer {
  padding: 16px 24px;
  border-top: 1px solid #e5e5e5;
  background: #f8f8f8;
  display: flex;
  gap: 12px;
  justify-content: flex-end;
  flex-shrink: 0;
}

.btn {
  padding: 10px 20px;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 17px;
  font-weight: 500;
  transition: all 0.2s ease;
}

.btn-primary {
  background: #007aff;
  color: white;
}

.btn-primary:hover {
  background: #0056cc;
}

.btn-secondary {
  background: #ffffff;
  color: #007aff;
  border: 1px solid #007aff;
}

.btn-secondary:hover {
  background: #f0f8ff;
}

</style>