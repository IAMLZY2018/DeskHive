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
          <span class="menu-item-icon" v-html="section.icon"></span>
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
                  <span class="theme-label light-label">
                    <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                      <circle cx="12" cy="12" r="5" fill="currentColor"/>
                      <path d="M12 1V3M12 21V23M4.22 4.22L5.64 5.64M18.36 18.36L19.78 19.78M1 12H3M21 12H23M4.22 19.78L5.64 18.36M18.36 5.64L19.78 4.22" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
                    </svg>
                  </span>
                  <span class="theme-label dark-label">
                    <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                      <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" fill="currentColor"/>
                    </svg>
                  </span>
                </div>
              </div>
            </div>
            <!-- 高优先级颜色 -->
            <div class="setting-item">
              <div>
                <div class="setting-label">高优先级颜色</div>
                <div class="setting-description">双击任务标记为高优先级时的圆点颜色</div>
              </div>
              <div class="setting-control">
                <div class="color-picker-wrapper">
                  <input 
                    type="color" 
                    v-model="settings.priority_color" 
                    class="color-picker"
                    @input="applyPriorityColorPreview"
                  >
                  <span class="color-value">{{ settings.priority_color }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- 行为设置 -->
        <div v-if="activeSection === 'behavior'" class="setting-section">
          <div class="section-title">窗口行为</div>
          <div class="setting-group">
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
            <div class="setting-item">
              <div>
                <div class="setting-label">窗口层级</div>
                <div class="setting-description">选择窗口显示在顶层还是桌面层</div>
              </div>
              <div class="setting-control">
                <div class="radio-group">
                  <label class="radio-option">
                    <input 
                      type="radio" 
                      value="always_on_top" 
                      v-model="settings.window_level"
                      @change="applyWindowLevel"
                    >
                    <span class="radio-label">置于顶层</span>
                  </label>
                  <label class="radio-option">
                    <input 
                      type="radio" 
                      value="always_on_bottom" 
                      v-model="settings.window_level"
                      @change="applyWindowLevel"
                    >
                    <span class="radio-label">置于桌面</span>
                  </label>
                </div>
              </div>
            </div>
          </div>

          <div class="section-title" style="margin-top: 24px;">启动行为</div>
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

        <!-- 使用设置 -->
        <div v-if="activeSection === 'tasks'" class="setting-section">
          <div class="section-title">时间轴设置</div>
          <div class="setting-group">
            <div class="setting-item">
              <div>
                <div class="setting-label">时间轴截止时间优先</div>
                <div class="setting-description">开启后，有截止时间的任务在时间轴上按截止时间排序；关闭则所有任务按创建时间排序</div>
              </div>
              <div class="setting-control">
                <div 
                  class="toggle-switch" 
                  :class="{ active: settings.timeline_deadline_priority }" 
                  @click="settings.timeline_deadline_priority = !settings.timeline_deadline_priority"
                ></div>
              </div>
            </div>
          </div>

          <div class="section-title" style="margin-top: 24px;">倒计时提醒</div>
          <div class="setting-group">
            <div class="setting-item">
              <div>
                <div class="setting-label">启用倒计时通知</div>
                <div class="setting-description">在任务截止前指定时间发送系统通知提醒</div>
              </div>
              <div class="setting-control">
                <div 
                  class="toggle-switch" 
                  :class="{ active: settings.enable_deadline_notification }" 
                  @click="settings.enable_deadline_notification = !settings.enable_deadline_notification"
                ></div>
              </div>
            </div>
            <div v-if="settings.enable_deadline_notification" class="setting-item">
              <div>
                <div class="setting-label">提前提醒时间</div>
                <div class="setting-description">在截止时间前多少分钟发送通知</div>
              </div>
              <div class="setting-control">
                <input 
                  type="number" 
                  v-model.number="settings.notification_minutes_before" 
                  min="1" 
                  max="1440"
                  class="number-input"
                >
                <span class="input-unit">分钟</span>
              </div>
            </div>

          </div>
        </div>

        <!-- 使用说明页面 -->
        <div v-if="activeSection === 'help'" class="setting-section">
          <div class="section-title">快速上手</div>
          <div class="setting-group">
            <div class="setting-item">
              <div class="help-content">
                <h3>📝 任务操作</h3>
                <p>• 新建任务：底部输入框输入内容后按回车键</p>
                <p>• 完成任务：鼠标悬停任务，点击左侧"✓"按钮</p>
                <p>• 撤销完成：在已完成分组中，悬停任务点击"↻"按钮</p>
                <p>• 编辑任务：右键任务选择"编辑任务"，或双击任务文字</p>
                <p>• 删除任务：右键任务选择"删除任务"</p>
                <p>• 标记优先级：双击任务快速标记/取消高优先级（橙色圆点）</p>
                <p>• 拖动排序：鼠标悬停任务，点住"☰"图标拖动调整顺序</p>
                <p>• 查看详情：右键任务查看创建时间、截止时间、优先级等信息</p>
                
                <h3>⏰ 截止时间</h3>
                <p>• 设置截止：右键任务选择"设置截止时间"</p>
                <p>• 快捷操作：对话框中使用 Tab 键切换输入框，Enter 键确认</p>
                <p>• 默认时间：首次设置默认为当前时间 1 小时后</p>
                <p>• 修改时间：右键任务选择"修改截止时间"，保留原有时间</p>
                <p>• 移除截止：右键任务选择"移除截止时间"</p>
                <p>• 倒计时显示：任务右侧显示剩余时间（绿色/黄色/红色）</p>
                <p>• 到期提醒：在"使用设置"中启用倒计时通知功能</p>
                
                <h3>📁 分组管理</h3>
                <p>• 快速创建：输入框输入"/分组名"后回车（如：/工作）</p>
                <p>• 菜单创建：点击底部"+"按钮，选择"新建分组"</p>
                <p>• 重命名分组：右键分组标题选择"重命名分组"</p>
                <p>• 删除分组：右键分组标题选择"删除分组"（任务会移到未分组）</p>
                <p>• 折叠/展开：点击分组标题左侧"▼"图标</p>
                <p>• 调整顺序：鼠标悬停分组标题，点击"▲▼"按钮上下移动</p>
                <p>• 未分组：新建任务默认添加到未分组，可拖动到其他分组</p>
                
                <h3>🔄 拖动功能</h3>
                <p>• 同组排序：拖动任务到目标位置释放，调整组内顺序</p>
                <p>• 跨组移动：拖动任务到其他分组的任务列表中</p>
                <p>• 快速移动：拖动任务到分组标题上，自动添加到该组末尾</p>
                <p>• 拖动提示：拖动时分组会高亮显示，表示可以放置</p>
                <p>• 禁止拖动：在"行为"设置中可禁止拖动窗口，方便调整位置</p>
                
                <h3>🎨 视图切换</h3>
                <p>• 列表视图：默认视图，按分组显示任务</p>
                <p>• 时间轴视图：点击标题栏"时间轴"图标切换</p>
                <p>• 时间轴排序：在"使用设置"中可选择按截止时间或创建时间排序</p>
                <p>• 时间轴操作：支持完成、删除、查看详情、标记优先级</p>
                
                <h3>✅ 已完成任务</h3>
                <p>• 查看已完成：点击底部"已完成"分组展开查看</p>
                <p>• 撤销完成：悬停已完成任务，点击"↻"按钮恢复为待办</p>
                <p>• 批量清理：点击"已完成"分组右侧垃圾桶图标清空所有</p>
                <p>• 自动清理：右键任务选择"移除完成7天前"，清理旧任务</p>
                <p>• 耗时显示：已完成任务显示从创建到完成的耗时天数</p>
                
                <h3>🔔 通知提醒</h3>
                <p>• 启用通知：在"使用设置"中开启"启用倒计时通知"</p>
                <p>• 提前时间：设置提前多少分钟提醒（默认 30 分钟）</p>
                <p>• 通知内容：显示剩余时间、任务内容、截止时间、优先级</p>
                <p>• 通知位置：Windows 系统右下角通知中心</p>
                <p>• 提示音：通知时播放系统提示音</p>
                <p>• 权限设置：如无通知，请检查 Windows 通知权限</p>
                
                <h3>⚙️ 外观设置</h3>
                <p>• 透明度：调整主窗口透明度（50%-100%），设置窗口保持不透明</p>
                <p>• 主题模式：切换日间/夜间主题，夜间模式更护眼</p>
                <p>• 优先级颜色：自定义高优先级任务的圆点颜色</p>
                
                <h3>🎯 行为设置</h3>
                <p>• 禁止拖动：开启后无法拖动窗口，防止误操作</p>
                <p>• 窗口层级：选择"置于顶层"或"置于桌面"</p>
                <p>• 开机自启：系统启动时自动运行应用</p>
                
                <h3>💡 实用技巧</h3>
                <p>• 托盘图标：左键点击快速显示/隐藏窗口</p>
                <p>• 托盘菜单：右键托盘图标访问快捷功能</p>
                <p>• 重置位置：托盘菜单选择"重置窗口位置"，自动关闭禁止拖动</p>
                <p>• 窗口位置：拖动窗口后自动记住位置，下次启动恢复</p>
                <p>• 关闭窗口：点击关闭按钮会最小化到托盘，不会退出程序</p>
                <p>• 完全退出：右键托盘图标选择"退出"</p>
                <p>• 数据保存：所有数据自动保存到文档目录，重装不丢失</p>
                <p>• 设置同步：所有设置修改即时生效，无需手动保存</p>
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
                    <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                      <path d="M14 3V5H17.59L7.76 14.83L9.17 16.24L19 6.41V10H21V3M19 19H5V5H12V3H5C3.89 3 3 3.9 3 5V19C3 20.1 3.89 21 5 21H19C20.1 21 21 20.1 21 19V12H19V19Z" fill="currentColor"/>
                    </svg>
                    给作者留言
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
            <div class="setting-item">
              <div>
                <div class="setting-label">检查更新</div>
                <div class="setting-description">访问官网查看最新版本</div>
              </div>
              <div class="setting-control">
                <button class="check-update-btn" @click="checkUpdate">
                  <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M12 2C6.48 2 2 6.48 2 12C2 17.52 6.48 22 12 22C17.52 22 22 17.52 22 12C22 6.48 17.52 2 12 2M12 20C7.59 20 4 16.41 4 12C4 7.59 7.59 4 12 4C16.41 4 20 7.59 20 12C20 16.41 16.41 20 12 20M16.59 7.58L10 14.17L7.41 11.59L6 13L10 17L18 9L16.59 7.58Z" fill="currentColor"/>
                  </svg>
                  检查新版本
                </button>
              </div>
            </div>
          </div>

          <div class="section-title" style="margin-top: 24px;">版本更新</div>
          <div class="setting-group">
            <div class="setting-item">
              <div class="update-content">
                <h3>v1.0.0 (2025-12-11)</h3>
                <h4>✨ 新增功能</h4>
                <p>• 时间轴视图：全新的任务时间树展示方式，按时间线查看任务</p>
                <p>• 截止时间通知：支持定时系统通知，可自定义提前提醒时间</p>
                <p>• 通知设置：可配置是否启用截止时间通知和提前提醒分钟数</p>
                <p>• 窗口焦点优化：托盘图标和设置按钮点击时强制获取焦点</p>
                <p>• 已完成任务优化：右键菜单简化，移除不必要的编辑选项</p>
                
                <h4>🎨 界面优化</h4>
                <p>• 页面样式全面优化，更美观简洁</p>
                <p>• 修复高分辨率屏幕显示异常问题</p>
                <p>• 优化 Tooltip 提示框位置算法，智能避免遮挡</p>
                <p>• 改进"置于桌面"模式下的窗口显示逻辑</p>
                <p>• 优化设置窗口的焦点获取体验</p>
                
                <h4>🐛 问题修复</h4>
                <p>• 修复托盘点击需要两次才能显示的问题</p>
                <p>• 修复"置于桌面"模式下窗口无法获取焦点的问题</p>
                <p>• 修复设置窗口已打开时点击无反应的问题</p>
                <p>• 修复第一个任务的提示框被遮挡的问题</p>
                <p>• 优化窗口显示和隐藏的稳定性</p>
                
                <h4>🔧 功能调整</h4>
                <p>• 托盘菜单"显示/隐藏"改为"显示"，统一为获取焦点</p>
                <p>• 已完成任务右键菜单移除"编辑"、"设置截止时间"等选项</p>
                <p>• 使用 Windows API 强制获取窗口焦点，提升用户体验</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="content-footer">
        <button class="btn btn-primary" @click="closeWindow">
          关闭
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'

interface AppSettings {
  opacity: number
  disable_drag: boolean
  auto_start: boolean
  silent_start: boolean
  theme: string
  priority_color: string
  window_level: string
  timeline_deadline_priority: boolean
  enable_deadline_notification: boolean
  notification_minutes_before: number
}

type SectionKey = 'appearance' | 'behavior' | 'tasks' | 'help' | 'contact' | 'about'

interface Section {
  name: string
  icon: string
}

const currentWindow = getCurrentWindow()

const activeSection = ref<SectionKey>('appearance')
const originalOpacity = ref(0.95)
const appVersion = ref('...')

const sections: Record<SectionKey, Section> = {
  appearance: { 
    name: '外观', 
    icon: '<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12 2C11.5 2 11 2.19 10.59 2.59L2.59 10.59C1.8 11.37 1.8 12.63 2.59 13.41L10.59 21.41C11.37 22.2 12.63 22.2 13.41 21.41L21.41 13.41C22.2 12.63 22.2 11.37 21.41 10.59L13.41 2.59C13 2.19 12.5 2 12 2M12 4L20 12L12 20L4 12L12 4M12 6C9.79 6 8 7.79 8 10C8 12.21 9.79 14 12 14C14.21 14 16 12.21 16 10C16 7.79 14.21 6 12 6Z" fill="currentColor"/></svg>' 
  },
  behavior: { 
    name: '行为', 
    icon: '<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M13 2.03V2.05L13 4.05C17.39 4.59 20.5 8.58 19.96 12.97C19.5 16.61 16.64 19.5 13 19.93V21.93C18.5 21.38 22.5 16.5 21.95 11C21.5 6.25 17.73 2.5 13 2.03M11 2.06C9.05 2.25 7.19 3 5.67 4.26L7.1 5.74C8.22 4.84 9.57 4.26 11 4.06V2.06M4.26 5.67C3 7.19 2.25 9.04 2.05 11H4.05C4.24 9.58 4.8 8.23 5.69 7.1L4.26 5.67M2.06 13C2.26 14.96 3.03 16.81 4.27 18.33L5.69 16.9C4.81 15.77 4.24 14.42 4.06 13H2.06M7.1 18.37L5.67 19.74C7.18 21 9.04 21.79 11 22V20C9.58 19.82 8.23 19.25 7.1 18.37M12.5 7V12.25L17 14.92L16.25 16.15L11 13V7H12.5Z" fill="currentColor"/></svg>' 
  },
  tasks: { 
    name: '使用设置', 
    icon: '<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M19 3H14.82C14.4 1.84 13.3 1 12 1C10.7 1 9.6 1.84 9.18 3H5C3.9 3 3 3.9 3 5V19C3 20.1 3.9 21 5 21H19C20.1 21 21 20.1 21 19V5C21 3.9 20.1 3 19 3M12 3C12.55 3 13 3.45 13 4C13 4.55 12.55 5 12 5C11.45 5 11 4.55 11 4C11 3.45 11.45 3 12 3M7 7H17V9H7V7M7 11H17V13H7V11M7 15H14V17H7V15Z" fill="currentColor"/></svg>' 
  },
  help: { 
    name: '使用说明', 
    icon: '<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M19 2H14.82C14.4 0.84 13.3 0 12 0C10.7 0 9.6 0.84 9.18 2H5C3.9 2 3 2.9 3 4V18C3 19.1 3.9 20 5 20H9.11C9.56 21.19 10.69 22 12 22C13.31 22 14.44 21.19 14.89 20H19C20.1 20 21 19.1 21 18V4C21 2.9 20.1 2 19 2M12 2C12.55 2 13 2.45 13 3C13 3.55 12.55 4 12 4C11.45 4 11 3.55 11 3C11 2.45 11.45 2 12 2M12 20C11.45 20 11 19.55 11 19C11 18.45 11.45 18 12 18C12.55 18 13 18.45 13 19C13 19.55 12.55 20 12 20M19 18H14.82C14.4 16.84 13.3 16 12 16C10.7 16 9.6 16.84 9.18 18H5V4H9.18C9.6 5.16 10.7 6 12 6C13.3 6 14.4 5.16 14.82 4H19V18M12 9C10.9 9 10 9.9 10 11C10 12.1 10.9 13 12 13C13.1 13 14 12.1 14 11C14 9.9 13.1 9 12 9Z" fill="currentColor"/></svg>' 
  },
  contact: { 
    name: '联系作者', 
    icon: '<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M20 4H4C2.9 4 2.01 4.9 2.01 6L2 18C2 19.1 2.9 20 4 20H20C21.1 20 22 19.1 22 18V6C22 4.9 21.1 4 20 4M20 8L12 13L4 8V6L12 11L20 6V8Z" fill="currentColor"/></svg>' 
  },
  about: { 
    name: '关于', 
    icon: '<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M11 7H13V9H11V7M11 11H13V17H11V11M12 2C6.48 2 2 6.48 2 12C2 17.52 6.48 22 12 22C17.52 22 22 17.52 22 12C22 6.48 17.52 2 12 2M12 20C7.59 20 4 16.41 4 12C4 7.59 7.59 4 12 4C16.41 4 20 7.59 20 12C20 16.41 16.41 20 12 20Z" fill="currentColor"/></svg>' 
  }
}

const settings = reactive<AppSettings>({
  opacity: 1.0,
  disable_drag: false,
  auto_start: false,
  silent_start: false,
  theme: 'light',
  priority_color: '#FF9800',
  window_level: 'always_on_bottom',
  timeline_deadline_priority: true,
  enable_deadline_notification: false,
  notification_minutes_before: 30
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

// 实时预览高优先级颜色
async function applyPriorityColorPreview() {
  try {
    // 通知主窗口更新高优先级颜色
    await invoke('emit_priority_color_changed', { color: settings.priority_color })
  } catch (error) {
    console.error('应用高优先级颜色预览失败:', error)
  }
}

// 实时应用窗口层级设置
async function applyWindowLevel() {
  try {
    // 临时保存并应用窗口层级设置以实现预览
    const tempSettings = {
      opacity: settings.opacity,
      disable_drag: settings.disable_drag,
      auto_start: settings.auto_start,
      theme: settings.theme,
      priority_color: settings.priority_color,
      window_level: settings.window_level
    }
    await invoke('save_app_settings', { settings: tempSettings })
  } catch (error) {
    console.error('应用窗口层级失败:', error)
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

// 保存设置（即时保存，不关闭窗口）
async function saveSettingsImmediately() {
  try {
    // 确保数据类型正确
    const settingsToSave = {
      opacity: typeof settings.opacity === 'string' ? parseFloat(settings.opacity) : settings.opacity,
      disable_drag: Boolean(settings.disable_drag),
      auto_start: Boolean(settings.auto_start),
      silent_start: Boolean(settings.silent_start),
      theme: settings.theme,
      priority_color: settings.priority_color,
      window_level: settings.window_level,
      timeline_deadline_priority: Boolean(settings.timeline_deadline_priority),
      enable_deadline_notification: Boolean(settings.enable_deadline_notification),
      notification_minutes_before: typeof settings.notification_minutes_before === 'string' 
        ? parseInt(settings.notification_minutes_before) 
        : settings.notification_minutes_before
    }
    
    // 调用 Tauri 命令保存设置
    await invoke('save_app_settings', { settings: settingsToSave })
    console.log('设置已自动保存')
    
    // 通知主窗口主题已更改
    if (settingsToSave.theme) {
      await invoke('emit_theme_changed', { theme: settingsToSave.theme })
    }
  } catch (error) {
    console.error('保存设置失败:', error)
  }
}

// 监听设置变化，自动保存
let saveTimeout: number | null = null
watch(settings, () => {
  // 防抖：延迟500ms保存，避免频繁保存
  if (saveTimeout) {
    clearTimeout(saveTimeout)
  }
  saveTimeout = window.setTimeout(() => {
    saveSettingsImmediately()
  }, 500)
}, { deep: true })

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

// 检查新版本
function checkUpdate() {
  window.open('https://www.feijimiao.cn/deskhive', '_blank')
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
  background: #fafafa;
  display: flex !important;
  flex-direction: row !important;
  flex-wrap: nowrap !important;
  overflow: hidden !important;
  position: fixed !important;
  top: 0 !important;
  left: 0 !important;
  border-radius: 12px;
}

.sidebar {
  width: 240px !important;
  min-width: 240px !important;
  max-width: 240px !important;
  height: 100vh !important;
  background: linear-gradient(180deg, #ffffff 0%, #f8f9fa 100%);
  border-right: 1px solid #e8eaed;
  display: flex !important;
  flex-direction: column !important;
  flex-shrink: 0 !important;
  flex-basis: 240px !important;
  overflow: hidden !important;
  box-shadow: 2px 0 8px rgba(0, 0, 0, 0.02);
}

.sidebar-header {
  padding: 16px 16px 16px;
  border-bottom: 1px solid #e8eaed;
  -webkit-app-region: drag;
  user-select: none;
  flex-shrink: 0;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  height: 60px;
  box-sizing: border-box;
  display: flex;
  align-items: center;
}

.sidebar-header h1 {
  font-size: 20px;
  font-weight: 700;
  color: #1a1a1a;
  margin: 0;
  letter-spacing: -0.3px;
  line-height: 1;
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
  padding: 10px 16px;
  margin: 3px 10px;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  border: none;
  background: transparent;
  width: calc(100% - 20px);
  text-align: left;
  font-size: 14px;
  color: #5f6368;
  border-radius: 8px;
  font-weight: 500;
}

.menu-item:hover {
  background: rgba(0, 122, 255, 0.08);
  color: #007aff;
  transform: translateX(2px);
}

.menu-item.active {
  background: #007aff;
  color: #ffffff;
  box-shadow: 0 2px 8px rgba(0, 122, 255, 0.25);
  transform: translateX(0);
}

.menu-item-icon {
  width: 18px;
  height: 18px;
  margin-right: 10px;
  font-size: 15px;
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
  padding: 16px 24px 16px;
  border-bottom: 1px solid #e8eaed;
  -webkit-app-region: drag;
  user-select: none;
  flex-shrink: 0;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  height: 60px;
  box-sizing: border-box;
  display: flex;
  align-items: center;
}

.content-header h2 {
  font-size: 22px;
  font-weight: 700;
  color: #202124;
  margin: 0;
  letter-spacing: -0.3px;
  line-height: 1;
}

.content-body {
  flex: 1;
  padding: 20px 24px;
  overflow-y: auto;
  background: #fafafa;
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
  margin-bottom: 20px;
}

.setting-section:last-child {
  margin-bottom: 0;
}

.section-title {
  font-size: 11px;
  font-weight: 700;
  color: #8e8e93;
  text-transform: uppercase;
  letter-spacing: 0.8px;
  margin-bottom: 8px;
}

.setting-group {
  background: #ffffff;
  border-radius: 12px;
  border: 1px solid #e8eaed;
  overflow: hidden;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
  transition: box-shadow 0.3s ease;
}

.setting-group:hover {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 14px 16px;
  border-bottom: 1px solid #f0f1f3;
  min-height: 50px;
  transition: background-color 0.2s ease;
}

.setting-item:hover {
  background: #fafbfc;
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-label {
  font-size: 14px;
  color: #202124;
  font-weight: 600;
  margin-bottom: 2px;
}

.setting-description {
  font-size: 12px;
  color: #5f6368;
  margin-top: 2px;
  line-height: 1.4;
}

.setting-control {
  display: flex;
  align-items: center;
}

.toggle-switch {
  position: relative;
  width: 51px;
  height: 31px;
  background: #e0e0e0;
  border-radius: 16px;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.toggle-switch.active {
  background: #34c759;
  box-shadow: 0 2px 6px rgba(52, 199, 89, 0.3);
}

.toggle-switch::after {
  content: '';
  position: absolute;
  top: 3px;
  left: 3px;
  width: 25px;
  height: 25px;
  background: #ffffff;
  border-radius: 50%;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.15);
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
  pointer-events: none;
  display: flex;
  align-items: center;
  justify-content: center;
}

.theme-label svg {
  width: 14px;
  height: 14px;
}

.light-label {
  left: 7px;
  color: #fbbf24;
}

.dark-label {
  right: 7px;
  color: #60a5fa;
}

.theme-toggle-switch.theme-dark .light-label {
  color: rgba(251, 191, 36, 0.4);
}

.theme-toggle-switch.theme-dark .dark-label {
  color: #60a5fa;
}

.range-value {
  font-size: 17px;
  color: #007aff;
  font-weight: 500;
  min-width: 40px;
  text-align: right;
}

.color-picker-wrapper {
  display: flex;
  align-items: center;
  gap: 10px;
}

.color-picker {
  width: 50px;
  height: 32px;
  border: 2px solid #e5e5e5;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.color-picker:hover {
  border-color: #007aff;
  transform: scale(1.05);
}

.color-value {
  font-size: 14px;
  color: #5f6368;
  font-weight: 500;
  font-family: 'Courier New', monospace;
}

.radio-group {
  display: flex;
  gap: 12px;
  align-items: center;
}

.radio-option {
  display: flex;
  align-items: center;
  cursor: pointer;
  user-select: none;
}

.radio-option input[type="radio"] {
  width: 18px;
  height: 18px;
  margin: 0;
  cursor: pointer;
  accent-color: #007aff;
}

.radio-label {
  margin-left: 6px;
  font-size: 14px;
  color: #202124;
  font-weight: 500;
}

.number-input {
  width: 80px;
  padding: 6px 10px;
  border: 1px solid #e5e5e5;
  border-radius: 8px;
  background: #ffffff;
  color: #202124;
  font-size: 14px;
  font-weight: 500;
  text-align: center;
  transition: all 0.2s ease;
}

.number-input:focus {
  outline: none;
  border-color: #007aff;
  box-shadow: 0 0 0 3px rgba(0, 122, 255, 0.1);
}

.input-unit {
  margin-left: 8px;
  font-size: 14px;
  color: #5f6368;
  font-weight: 500;
}

.test-btn {
  padding: 6px 16px;
  background: #007aff;
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.test-btn:hover {
  background: #0051d5;
  transform: translateY(-1px);
}

.test-btn:active {
  transform: translateY(0);
}

.content-footer {
  padding: 14px 24px;
  border-top: 1px solid #e8eaed;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  display: flex;
  gap: 10px;
  justify-content: flex-end;
  flex-shrink: 0;
}

.btn {
  padding: 9px 22px;
  border: none;
  border-radius: 10px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 600;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  letter-spacing: 0.2px;
}

.btn-primary {
  background: #007aff;
  color: white;
  box-shadow: 0 2px 8px rgba(0, 122, 255, 0.25);
}

.btn-primary:hover {
  background: #0051d5;
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 122, 255, 0.3);
}

.btn-primary:active {
  transform: translateY(0);
}

.btn-secondary {
  background: #ffffff;
  color: #007aff;
  border: 2px solid #007aff;
}

.btn-secondary:hover {
  background: rgba(0, 122, 255, 0.08);
  transform: translateY(-1px);
}

/* 夜间主题下的设置页面样式 */
body.dark-theme {
  background: #0a0a0a;
  color: #e0e0e0;
}

body.dark-theme .container {
  background: #0a0a0a;
  border-radius: 12px;
}

body.dark-theme .sidebar {
  background: linear-gradient(180deg, #0f0f0f 0%, #0a0a0a 100%);
  border-right: 1px solid rgba(255, 255, 255, 0.08);
  box-shadow: 2px 0 12px rgba(0, 0, 0, 0.5);
}

body.dark-theme .sidebar-header {
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  background: rgba(15, 15, 15, 0.9);
}

body.dark-theme .sidebar-header h1 {
  color: #e0e0e0;
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
  color: #808080;
}

body.dark-theme .menu-item:hover {
  background: rgba(0, 122, 255, 0.15);
  color: #0a84ff;
}

body.dark-theme .menu-item.active {
  background: #0a84ff;
  color: #ffffff;
  box-shadow: 0 2px 8px rgba(10, 132, 255, 0.4);
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
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  background: rgba(15, 15, 15, 0.9);
}

body.dark-theme .content-header h2 {
  color: #e0e0e0;
}

body.dark-theme .content-body {
  background: #0a0a0a;
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
  color: #8e8e93;
}

body.dark-theme .setting-group {
  background: #141414;
  border: 1px solid rgba(255, 255, 255, 0.08);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.4);
}

body.dark-theme .setting-group:hover {
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.5);
}

body.dark-theme .setting-item {
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

body.dark-theme .setting-item:hover {
  background: #1a1a1a;
}

body.dark-theme .setting-label {
  color: #e0e0e0;
}

body.dark-theme .setting-description {
  color: #808080;
}

body.dark-theme .setting-control {
  display: flex;
  align-items: center;
}

body.dark-theme .toggle-switch {
  background: #202020;
}

body.dark-theme .toggle-switch.active {
  background: #30d158;
  box-shadow: 0 2px 6px rgba(48, 209, 88, 0.5);
}

body.dark-theme .setting-control input[type="range"] {
  width: 120px;
  margin-right: 8px;
}

body.dark-theme .setting-control select {
  padding: 8px 12px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  background: #1a1a1a;
  color: #e0e0e0;
  font-size: 17px;
  min-width: 120px;
}

body.dark-theme .theme-toggle-switch {
  position: relative;
  width: 60px;
  height: 30px;
  background: #1a1a1a;
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

body.dark-theme .light-label {
  color: #fbbf24;
}

body.dark-theme .dark-label {
  color: #60a5fa;
}

body.dark-theme .theme-toggle-switch.theme-dark .light-label {
  color: rgba(251, 191, 36, 0.4);
}

body.dark-theme .theme-toggle-switch.theme-dark .dark-label {
  color: #60a5fa;
}

body.dark-theme .range-value {
  font-size: 17px;
  color: #007aff;
  font-weight: 500;
  min-width: 40px;
  text-align: right;
}

body.dark-theme .color-picker {
  border-color: rgba(255, 255, 255, 0.1);
}

body.dark-theme .color-picker:hover {
  border-color: #0a84ff;
}

body.dark-theme .color-value {
  color: #808080;
}

body.dark-theme .radio-option input[type="radio"] {
  accent-color: #0a84ff;
}

body.dark-theme .radio-label {
  color: #e0e0e0;
}

body.dark-theme .number-input {
  border-color: rgba(255, 255, 255, 0.1);
  background: #1a1a1a;
  color: #e0e0e0;
}

body.dark-theme .number-input:focus {
  border-color: #0a84ff;
  box-shadow: 0 0 0 3px rgba(10, 132, 255, 0.2);
}

body.dark-theme .input-unit {
  color: #808080;
}

body.dark-theme .test-btn {
  background: #0a84ff;
}

body.dark-theme .test-btn:hover {
  background: #0077ed;
}

body.dark-theme .content-footer {
  border-top: 1px solid rgba(255, 255, 255, 0.08);
  background: rgba(15, 15, 15, 0.98);
}

body.dark-theme .btn-primary {
  background: #0a84ff;
  box-shadow: 0 2px 8px rgba(10, 132, 255, 0.4);
}

body.dark-theme .btn-primary:hover {
  background: #0077ed;
  box-shadow: 0 4px 12px rgba(10, 132, 255, 0.5);
}

body.dark-theme .btn-secondary {
  background: #141414;
  color: #0a84ff;
  border: 2px solid #0a84ff;
}

body.dark-theme .btn-secondary:hover {
  background: rgba(10, 132, 255, 0.15);
}

/* 使用说明内容样式 */
.help-content {
  padding: 12px 14px;
  line-height: 1.5;
}

.help-content h3 {
  margin: 12px 0 6px 0;
  font-size: 14px;
  font-weight: 600;
  color: #000;
}

body.dark-theme .help-content h3 {
  color: #e7e9ed;
}

.help-content p {
  margin: 3px 0;
  font-size: 13px;
  color: #333;
}

body.dark-theme .help-content p {
  color: #a0a6aa;
}

/* 更新说明内容样式 */
.update-content {
  padding: 12px 14px;
  line-height: 1.5;
}

.update-content h3 {
  margin: 0 0 12px 0;
  font-size: 16px;
  font-weight: 600;
  color: #007aff;
}

body.dark-theme .update-content h3 {
  color: #007aff;
}

.update-content h4 {
  margin: 10px 0 6px 0;
  font-size: 14px;
  font-weight: 600;
  color: #000;
}

body.dark-theme .update-content h4 {
  color: #e7e9ed;
}

.update-content p {
  margin: 3px 0;
  font-size: 13px;
  color: #333;
  padding-left: 6px;
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
  padding: 20px 16px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 24px;
}

.contact-row {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
}

/* 第一行：Logo 和按钮 */
.logo-row {
  gap: 16px;
}

.contact-logo {
  width: 80px;
  height: 80px;
  border-radius: 40px;
  object-fit: cover;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

body.dark-theme .contact-logo {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

.blog-btn {
  padding: 9px 20px;
  background: #007aff;
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 2px 8px rgba(0, 122, 255, 0.3);
  display: flex;
  align-items: center;
  gap: 6px;
}

.blog-btn svg {
  width: 18px;
  height: 18px;
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

/* 检查更新按钮 */
.check-update-btn {
  padding: 6px 16px;
  background: #34c759;
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 2px 6px rgba(52, 199, 89, 0.3);
  display: flex;
  align-items: center;
  gap: 6px;
}

.check-update-btn svg {
  width: 16px;
  height: 16px;
}

.check-update-btn:hover {
  background: #28a745;
  transform: translateY(-1px);
  box-shadow: 0 4px 10px rgba(52, 199, 89, 0.4);
}

.check-update-btn:active {
  transform: translateY(0);
}

body.dark-theme .check-update-btn {
  background: #34c759;
  box-shadow: 0 2px 6px rgba(52, 199, 89, 0.4);
}

body.dark-theme .check-update-btn:hover {
  background: #28a745;
  box-shadow: 0 4px 10px rgba(52, 199, 89, 0.5);
}

/* 第二行：二维码并排 */
.qrcode-row {
  gap: 32px;
  flex-wrap: wrap;
}

.qrcode-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
}

.qrcode-item h3 {
  margin: 0 0 12px 0;
  font-size: 14px;
  font-weight: 600;
  color: #000;
}

body.dark-theme .qrcode-item h3 {
  color: #e7e9ed;
}

.contact-qrcode {
  width: 150px;
  height: 150px;
  border-radius: 10px;
  object-fit: cover;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
  border: 1px solid #e5e5e5;
}

body.dark-theme .contact-qrcode {
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.3);
  border: 1px solid #444b4f;
}

.qrcode-tip {
  margin: 8px 0 0 0;
  font-size: 12px;
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
  font-size: 14px;
  font-weight: 500;
  color: #007aff;
  margin: 0;
  padding: 8px 18px;
  background: rgba(0, 122, 255, 0.1);
  border-radius: 8px;
  border: 1px solid rgba(0, 122, 255, 0.2);
}

body.dark-theme .contact-text {
  color: #0a84ff;
  background: rgba(10, 132, 255, 0.15);
  border: 1px solid rgba(10, 132, 255, 0.3);
}
</style>