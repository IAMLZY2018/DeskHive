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
            <!-- 添加主题切换按钮 -->
            <div class="setting-item">
              <div>
                <div class="setting-label">主题模式</div>
                <div class="setting-description">切换日间或夜间主题</div>
              </div>
              <div class="setting-control">
                <div 
                  class="theme-toggle-switch" 
                  :class="{ 'theme-dark': settings.theme === 'dark' }"
                  @click="toggleTheme"
                >
                  <div class="theme-toggle-slider"></div>
                  <span class="theme-label light-label">☀️</span>
                  <span class="theme-label dark-label">🌙</span>
                </div>
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

        <!-- 使用说明页面 -->
        <div v-if="activeSection === 'help'" class="setting-section">
          <div class="section-title">使用说明</div>
          <div class="setting-group">
            <div class="setting-item">
              <div class="help-content">
                <h3>📝 任务管理</h3>
                <p>• 创建任务：底部输入框输入内容，按回车或点击"+"</p>
                <p>• 完成任务：悬停任务后点击"✓"按钮（时间指示器左侧）</p>
                <p>• 取消完成：在已完成分组中点击"↶"按钮恢复</p>
                <p>• 删除任务：双击任务项快速删除</p>
                <p>• 编辑任务：右键任务 → "✏️ 编辑任务"</p>
                <p>• 拖动排序：悬停任务后点击"☰"按钮拖动调整顺序</p>
                
                <h3>📁 分组管理</h3>
                <p>• 快速创建：底部输入框输入 "/分组名" 按回车</p>
                <p>• 对话框创建：右键点击底部"+"按钮 → "📁 新建分组"</p>
                <p>• 重命名分组：右键分组标题 → "✏️ 重命名分组"</p>
                <p>• 删除分组：右键分组标题 → "🗑️ 删除分组"</p>
                <p>• 折叠/展开：点击分组标题左侧"▼"图标</p>
                <p>• 调整顺序：悬停分组标题，点击"▲▼"按钮上下移动</p>
                
                <h3>🔄 任务拖动</h3>
                <p>• 分组内拖动：点住"☰"按钮在同一分组内上下拖动</p>
                <p>• 跨分组拖动：拖到目标分组任务列表插入指定位置</p>
                <p>• 拖到分组标题：任务会添加到该分组末尾</p>
                <p>• 拖动提示：目标区域显示蓝色高亮</p>
                
                <h3>⏰ 时间管理</h3>
                <p>• 设置截止时间：右键任务 → "📅 设置截止时间"</p>
                <p>• 移除截止时间：右键任务 → "🗑️ 移除截止时间"</p>
                <p>• 时间显示位置：任务条最右侧（拖动按钮右边）</p>
                <p>• 🟢 绿色指示器：距离截止时间充足（悬停显示日期）</p>
                <p>• 🔴 红色指示器：已超过截止时间（悬停显示已超时）</p>
                <p>• 🟡 黄色指示器：任务已创建多天（悬停显示天数）</p>
                
                <h3>✅ 已完成任务</h3>
                <p>• 查看已完成：点击底部"已完成"分组展开</p>
                <p>• 清空已完成：点击已完成分组右侧垃圾桶图标</p>
                <p>• 恢复任务：点击已完成任务的"↶"按钮</p>
                
                <h3>⚙️ 窗口设置</h3>
                <p>• 移动窗口：拖拽窗口顶部（需启用拖动功能）</p>
                <p>• 调整透明度：外观设置 → 主窗口透明度</p>
                <p>• 切换主题：外观设置 → 主题模式（日间/夜间）</p>
                <p>• 开机自启：行为设置 → 开机自启动</p>
                
                <h3>🖱️ 系统托盘</h3>
                <p>• 左键点击：显示/隐藏主窗口</p>
                <p>• 右键菜单：快速添加任务、退出应用</p>
              </div>
            </div>
          </div>
        </div>

        <!-- 联系作者页面 -->
        <div v-if="activeSection === 'contact'" class="setting-section">
          <div class="section-title">联系方式</div>
          <div class="setting-group">
            <div class="setting-item contact-item">
              <div class="contact-content">
                <!-- 第一行：Logo 和跳转按钮 -->
                <div class="contact-row logo-row">
                  <img src="/mypic/feijimiao.png" alt="作者Logo" class="contact-logo" />
                  <button class="blog-btn" @click="openBlog">
                    🌐 点击访问
                  </button>
                </div>
                
                <!-- 第二行：提示文字 -->
                <div class="contact-row text-row">
                  <p class="contact-text">💼 软件定制开发，联系作者</p>
                </div>
                
                <!-- 第三行：两个二维码并排 -->
                <div class="contact-row qrcode-row">
                  <div class="qrcode-item">
                    <h3>📱 微信公众号</h3>
                    <img src="/mypic/gzh.png" alt="公众号二维码" class="contact-qrcode" />
                    <p class="qrcode-tip">扫码关注公众号</p>
                  </div>
                  
                  <div class="qrcode-item">
                    <h3>💬 微信联系</h3>
                    <img src="/mypic/Snipaste_2025-11-23_01-09-52.png" alt="微信二维码" class="contact-qrcode" />
                    <p class="qrcode-tip">扫码添加微信</p>
                  </div>
                </div>
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
          </div>

          <div class="section-title" style="margin-top: 24px;">版本更新</div>
          <div class="setting-group">
            <div class="setting-item">
              <div class="update-content">
                <h3>v0.3.0 (2025-11-22)</h3>
                <h4>✨ 新增功能</h4>
                <p>• 任务拖动排序：支持分组内和跨分组拖动任务</p>
                <p>• 分组排序：可通过上下箭头调整分组顺序</p>
                <p>• 时间提示：悬停时间指示器显示详细信息</p>
                <p>• 完成动画：优化任务完成时的视觉效果</p>
                <p>• 分组动画：添加分组移动的平滑过渡效果</p>
                
                <h4>🎨 界面优化</h4>
                <p>• 重新设计时间指示器样式和颜色</p>
                <p>• 优化已完成分组的高度和间距</p>
                <p>• 改进任务完成状态的视觉反馈</p>
                <p>• 添加自定义 Tooltip 组件</p>
                <p>• 优化分组标题和操作按钮布局</p>
                
                <h4>🐛 问题修复</h4>
                <p>• 修复跨组拖动任务的问题</p>
                <p>• 修复提示框被分组遮挡的问题</p>
                <p>• 移除按截止时间自动排序的逻辑</p>
                <p>• 修复分组内拖动回到原位的问题</p>
                <p>• 优化拖动时的视觉反馈</p>
                
                <h4>📝 文档更新</h4>
                <p>• 更新 README 使用说明</p>
                <p>• 完善设置页面的使用指南</p>
                <p>• 添加详细的功能说明和操作指引</p>
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
  theme: string
}

type SectionKey = 'appearance' | 'behavior' | 'help' | 'contact' | 'about'

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
  help: { name: '使用说明', icon: '📖' },
  contact: { name: '联系作者', icon: '✉️' },
  about: { name: '关于', icon: 'ℹ️' }
}

const settings = reactive<AppSettings>({
  opacity: 1.0,
  disable_drag: false,
  auto_start: false,
  theme: 'light'
})

// 透明度的计算属性，确保始终为数字类型
const opacityValue = computed({
  get: () => settings.opacity,
  set: (value: string | number) => {
    settings.opacity = typeof value === 'string' ? parseFloat(value) : value
  }
})

// 主题切换函数
function toggleTheme() {
  settings.theme = settings.theme === 'light' ? 'dark' : 'light'
  // 应用主题到当前页面
  document.body.className = settings.theme === 'dark' ? 'dark-theme' : ''
  
  // 实时通知主窗口切换主题以实现预览效果
  invoke('emit_theme_changed', { theme: settings.theme })
}

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
      auto_start: Boolean(settings.auto_start),
      theme: settings.theme
    }
    
    console.log('转换后的设置数据:', settingsToSave)
    
    // 调用 Tauri 命令保存设置
    await invoke('save_app_settings', { settings: settingsToSave })
    console.log('设置保存成功')
    
    // 通知主窗口主题已更改
    if (settingsToSave.theme) {
      await invoke('emit_theme_changed', { theme: settingsToSave.theme })
    }
    
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
    
    // 应用主题到当前页面
    document.body.className = settings.theme === 'dark' ? 'dark-theme' : ''
    
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

// 打开博客链接
function openBlog() {
  window.open('https://www.feijimiao.cn/contact', '_blank')
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

/* 隐藏滚动条 */
::-webkit-scrollbar {
  width: 0px;
  height: 0px;
  background: transparent;
}

* {
  -ms-overflow-style: none;  /* IE and Edge */
  scrollbar-width: none;  /* Firefox */
}

/* 确保所有滚动条都被隐藏 */
.sidebar-menu,
.content-body {
  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none; /* IE and Edge */
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
  /* 隐藏滚动条 */
  -ms-overflow-style: none;
  scrollbar-width: none;
}

.sidebar-menu::-webkit-scrollbar {
  display: none;
  width: 0;
  height: 0;
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
  /* 隐藏滚动条 */
  -ms-overflow-style: none;
  scrollbar-width: none;
}

.content-body::-webkit-scrollbar {
  display: none;
  width: 0;
  height: 0;
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

/* 添加主题切换按钮样式 */
.theme-toggle-switch {
  position: relative;
  width: 60px;
  height: 30px;
  background: #e5e5e5;
  border-radius: 15px;
  cursor: pointer;
  transition: background-color 0.3s;
  overflow: hidden;
}

.theme-toggle-switch.theme-dark {
  background: #34c759;
}

.theme-toggle-slider {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 26px;
  height: 26px;
  background: #ffffff;
  border-radius: 50%;
  transition: transform 0.3s;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.theme-toggle-switch.theme-dark .theme-toggle-slider {
  transform: translateX(30px);
}

.theme-label {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  font-size: 14px;
  pointer-events: none;
}

.light-label {
  left: 8px;
  color: #f5c442;
}

.dark-label {
  right: 8px;
  color: #4a90e2;
}

.theme-toggle-switch.theme-dark .light-label {
  color: rgba(245, 196, 66, 0.5);
}

.theme-toggle-switch.theme-dark .dark-label {
  color: #4a90e2;
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

/* 夜间主题下的设置页面样式 */
body.dark-theme {
  background: #181a1b;
  color: #e7e9ed;
}

body.dark-theme .container {
  background: #181a1b;
}

body.dark-theme .sidebar {
  background: #252627;
  border-right: 1px solid #444b4f;
}

body.dark-theme .sidebar-header {
  border-bottom: 1px solid #444b4f;
}

body.dark-theme .sidebar-header h1 {
  color: #e7e9ed;
}

body.dark-theme .sidebar-menu {
  padding: 8px 0;
  overflow-y: auto;
  overflow-x: hidden;
  /* 隐藏滚动条 */
  -ms-overflow-style: none;
  scrollbar-width: none;
}

.body.dark-theme .sidebar-menu::-webkit-scrollbar {
  display: none;
  width: 0;
  height: 0;
}

body.dark-theme .menu-item {
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
  color: #e7e9ed;
}

body.dark-theme .menu-item:hover {
  background: #3d4549;
}

body.dark-theme .menu-item.active {
  background: #007aff;
  color: #ffffff;
}

body.dark-theme .menu-item-icon {
  width: 20px;
  height: 20px;
  margin-right: 12px;
  font-size: 16px;
}

body.dark-theme .content {
  flex: 1 !important;
  display: flex !important;
  flex-direction: column !important;
  min-width: 0 !important;
  height: 100vh !important;
  overflow: hidden !important;
}

body.dark-theme .content-header {
  padding: 20px 24px 16px;
  border-bottom: 1px solid #444b4f;
  -webkit-app-region: drag;
  user-select: none;
  flex-shrink: 0;
}

body.dark-theme .content-header h2 {
  font-size: 20px;
  font-weight: 600;
  color: #e7e9ed;
  margin: 0;
}

body.dark-theme .content-body {
  flex: 1;
  padding: 24px;
  overflow-y: auto;
  background: #181a1b;
  /* 隐藏滚动条 */
  -ms-overflow-style: none;
  scrollbar-width: none;
}

.body.dark-theme .content-body::-webkit-scrollbar {
  display: none;
  width: 0;
  height: 0;
}

body.dark-theme .setting-section {
  margin-bottom: 32px;
}

body.dark-theme .setting-section:last-child {
  margin-bottom: 0;
}

body.dark-theme .section-title {
  font-size: 13px;
  font-weight: 600;
  color: #e7e9ed;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 8px;
}

body.dark-theme .setting-group {
  background: #252627;
  border-radius: 10px;
  border: 1px solid #444b4f;
  overflow: hidden;
}

body.dark-theme .setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid #444b4f;
  min-height: 44px;
}

body.dark-theme .setting-item:last-child {
  border-bottom: none;
}

body.dark-theme .setting-label {
  font-size: 17px;
  color: #e7e9ed;
  font-weight: 400;
}

body.dark-theme .setting-description {
  font-size: 13px;
  color: #a0a6aa;
  margin-top: 2px;
}

body.dark-theme .setting-control {
  display: flex;
  align-items: center;
}

body.dark-theme .toggle-switch {
  position: relative;
  width: 51px;
  height: 31px;
  background: #252627;
  border-radius: 16px;
  cursor: pointer;
  transition: background-color 0.3s;
}

body.dark-theme .toggle-switch.active {
  background: #34c759;
}

body.dark-theme .toggle-switch::after {
  content: '';
  position: absolute;
  top: 2px;
  left: 2px;
  width: 27px;
  height: 27px;
  background: #e7e9ed;
  border-radius: 50%;
  transition: transform 0.3s;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

body.dark-theme .toggle-switch.active::after {
  transform: translateX(20px);
}

body.dark-theme .setting-control input[type="range"] {
  width: 120px;
  margin-right: 8px;
}

body.dark-theme .setting-control select {
  padding: 8px 12px;
  border: 1px solid #444b4f;
  border-radius: 8px;
  background: #252627;
  color: #e7e9ed;
  font-size: 17px;
  min-width: 120px;
}

body.dark-theme .theme-toggle-switch {
  position: relative;
  width: 60px;
  height: 30px;
  background: #252627;
  border-radius: 15px;
  cursor: pointer;
  transition: background-color 0.3s;
  overflow: hidden;
}

body.dark-theme .theme-toggle-switch.theme-dark {
  background: #34c759;
}

body.dark-theme .theme-toggle-slider {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 26px;
  height: 26px;
  background: #e7e9ed;
  border-radius: 50%;
  transition: transform 0.3s;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

body.dark-theme .theme-toggle-switch.theme-dark .theme-toggle-slider {
  transform: translateX(30px);
}

body.dark-theme .theme-label {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  font-size: 14px;
  pointer-events: none;
}

body.dark-theme .light-label {
  left: 8px;
  color: #f5c442;
}

body.dark-theme .dark-label {
  right: 8px;
  color: #4a90e2;
}

body.dark-theme .theme-toggle-switch.theme-dark .light-label {
  color: rgba(245, 196, 66, 0.5);
}

body.dark-theme .theme-toggle-switch.theme-dark .dark-label {
  color: #4a90e2;
}

body.dark-theme .range-value {
  font-size: 17px;
  color: #007aff;
  font-weight: 500;
  min-width: 40px;
  text-align: right;
}

body.dark-theme .content-footer {
  padding: 16px 24px;
  border-top: 1px solid #444b4f;
  background: #252627;
  display: flex;
  gap: 12px;
  justify-content: flex-end;
  flex-shrink: 0;
}

body.dark-theme .btn {
  padding: 10px 20px;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 17px;
  font-weight: 500;
  transition: all 0.2s ease;
}

body.dark-theme .btn-primary {
  background: #007aff;
  color: white;
}

body.dark-theme .btn-primary:hover {
  background: #0056cc;
}

body.dark-theme .btn-secondary {
  background: #252627;
  color: #007aff;
  border: 1px solid #007aff;
}

body.dark-theme .btn-secondary:hover {
  background: #3d4549;
}

/* 使用说明内容样式 */
.help-content {
  padding: 16px;
  line-height: 1.6;
}

.help-content h3 {
  margin: 16px 0 8px 0;
  font-size: 16px;
  font-weight: 600;
  color: #000;
}

body.dark-theme .help-content h3 {
  color: #e7e9ed;
}

.help-content p {
  margin: 4px 0;
  font-size: 14px;
  color: #333;
}

body.dark-theme .help-content p {
  color: #a0a6aa;
}

/* 更新说明内容样式 */
.update-content {
  padding: 16px;
  line-height: 1.6;
}

.update-content h3 {
  margin: 0 0 16px 0;
  font-size: 18px;
  font-weight: 600;
  color: #007aff;
}

body.dark-theme .update-content h3 {
  color: #007aff;
}

.update-content h4 {
  margin: 12px 0 8px 0;
  font-size: 15px;
  font-weight: 600;
  color: #000;
}

body.dark-theme .update-content h4 {
  color: #e7e9ed;
}

.update-content p {
  margin: 4px 0;
  font-size: 14px;
  color: #333;
  padding-left: 8px;
}

body.dark-theme .update-content p {
  color: #a0a6aa;
}

/* 联系作者页面样式 */
.contact-item {
  display: block;
  padding: 0;
}

.contact-content {
  padding: 32px 24px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 40px;
}

.contact-row {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
}

/* 第一行：Logo 和按钮 */
.logo-row {
  gap: 24px;
}

.contact-logo {
  width: 100px;
  height: 100px;
  border-radius: 50px;
  object-fit: cover;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

body.dark-theme .contact-logo {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

.blog-btn {
  padding: 12px 24px;
  background: #007aff;
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 2px 8px rgba(0, 122, 255, 0.3);
}

.blog-btn:hover {
  background: #0056cc;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 122, 255, 0.4);
}

.blog-btn:active {
  transform: translateY(0);
}

body.dark-theme .blog-btn {
  background: #007aff;
  box-shadow: 0 2px 8px rgba(0, 122, 255, 0.4);
}

body.dark-theme .blog-btn:hover {
  background: #0056cc;
  box-shadow: 0 4px 12px rgba(0, 122, 255, 0.5);
}

/* 第二行：二维码并排 */
.qrcode-row {
  gap: 48px;
  flex-wrap: wrap;
}

.qrcode-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
}

.qrcode-item h3 {
  margin: 0 0 16px 0;
  font-size: 15px;
  font-weight: 600;
  color: #000;
}

body.dark-theme .qrcode-item h3 {
  color: #e7e9ed;
}

.contact-qrcode {
  width: 180px;
  height: 180px;
  border-radius: 12px;
  object-fit: cover;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  border: 1px solid #e5e5e5;
}

body.dark-theme .contact-qrcode {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  border: 1px solid #444b4f;
}

.qrcode-tip {
  margin: 12px 0 0 0;
  font-size: 13px;
  color: #6d6d70;
}

body.dark-theme .qrcode-tip {
  color: #a0a6aa;
}

/* 第二行：提示文字 */
.text-row {
  justify-content: center;
}

.contact-text {
  font-size: 16px;
  font-weight: 500;
  color: #007aff;
  margin: 0;
  padding: 12px 24px;
  background: rgba(0, 122, 255, 0.1);
  border-radius: 8px;
  border: 1px solid rgba(0, 122, 255, 0.2);
}

body.dark-theme .contact-text {
  color: #4a9eff;
  background: rgba(74, 158, 255, 0.15);
  border: 1px solid rgba(74, 158, 255, 0.3);
}
</style>