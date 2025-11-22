# DeskHive

一个占用极低的桌面TODO插件，随时随地不忘事！

![image-20250915124406296](other/pic/deskhive.png)

交流与建议：

<img src="other/pic/wx.png" alt="image-20250907030349657" style="zoom:25%;" />

[中文](#中文) | [English](#english)

## 中文

DeskHive 是一个基于 Vue 3 和 Tauri 构建的轻量级、跨平台的桌面待办事项管理工具，旨在提供无需云端服务、数据本地存储、界面简洁高效的待办事项管理体验。

### 技术栈

- **前端**: Vue 3 + TypeScript + Vite
- **后端**: Rust (Tauri 框架)
- **构建工具**: Cargo (Rust) + npm (Node.js)
- **UI框架**: 自定义CSS响应式设计
- **数据存储**: 本地文件系统

### 功能特性

- 本地数据存储，无网络依赖
- 跨平台桌面应用（支持 Windows）
- 响应式 UI，支持拖拽、弹窗提示等交互
- 待办事项管理（添加、删除、标记完成任务）
- 任务分类与状态管理
- 截止时间跟踪与可视化指示器
- 系统托盘集成与快捷操作
- 窗口控制（透明度、位置管理）
- 设置管理（自动启动、窗口位置等）

### 使用说明

#### 📝 任务管理

- **创建任务**：在底部输入框输入任务内容，按回车键或点击"+"按钮
- **完成任务**：悬停任务后点击右侧的"✓"按钮，任务会移至已完成分组
- **取消完成**：在已完成分组中点击"↶"按钮恢复任务
- **删除任务**：双击任务项快速删除
- **编辑任务**：右键任务 → 选择"✏️ 编辑任务"修改内容
- **拖动排序**：悬停任务后点击"☰"按钮拖动调整顺序

#### 📁 分组管理

- **创建分组**：点击底部"📁"按钮，输入分组名称
- **重命名分组**：右键分组标题 → 选择"✏️ 重命名分组"
- **删除分组**：右键分组标题 → 选择"🗑️ 删除分组"（任务会移至未分组）
- **折叠/展开**：点击分组标题左侧的"▼"图标
- **调整顺序**：悬停分组标题，点击"▲▼"按钮上下移动分组

#### 🔄 任务拖动

- **分组内拖动**：点住任务的"☰"按钮，在同一分组内上下拖动调整顺序
- **跨分组拖动**：
  - 拖动到目标分组的任务列表中，插入到指定位置
  - 拖动到目标分组的标题上，任务会添加到该分组末尾
- **拖动提示**：拖动时目标区域会显示蓝色高亮

#### ⏰ 时间管理

- **设置截止时间**：右键任务 → "📅 设置截止时间" → 选择日期和时间
- **移除截止时间**：右键任务 → "🗑️ 移除截止时间"
- **时间指示器**：
  - 🟢 **绿色**：距离截止时间充足（悬停显示截止日期）
  - 🔴 **红色**：已超过截止时间（悬停显示已超时）
  - 🟡 **黄色**：任务已创建多天（悬停显示创建天数）

#### ✅ 已完成任务

- **查看已完成**：点击底部"已完成"分组展开查看
- **清空已完成**：点击已完成分组右侧的垃圾桶图标清除所有已完成任务
- **恢复任务**：点击已完成任务的"↶"按钮恢复到未完成状态

#### ⚙️ 其他功能

- **任务统计**：顶部显示"已完成/总任务"数量
- **右键菜单**：右键任务或分组可快速访问更多操作
- **设置面板**：点击右上角"⚙️"按钮打开设置

### 环境要求

- Node.js（版本 ^20.19.0 或 >=22.12.0）
- npm 或 pnpm
- Rust 工具链（rustup, cargo）

### 开发环境搭建

1. 克隆仓库：
   ```bash
   git clone <repository-url>
   cd DeskHive
   ```

2. 安装依赖：
   ```bash
   npm install
   ```

3. 运行开发环境：
   ```bash
   npm run dev
   ```

### 生产环境构建

1. 构建应用程序：
   ```bash
   npm run tauri build
   ```

2. 构建产物位置：
   - Windows: `src-tauri/target/release/bundle/nsis/` (`.exe`)
   - macOS: `src-tauri/target/release/bundle/macos/` (`.app`)
   - Linux: `src-tauri/target/release/bundle/appimage/` (`.AppImage`)

## English

DeskHive is a lightweight, cross-platform desktop to-do management tool built with Vue 3 and Tauri, designed to provide a local task management experience without cloud dependencies.

### Technology Stack

- **Frontend**: Vue 3 + TypeScript + Vite
- **Backend**: Rust (Tauri Framework)
- **Build Tool**: Cargo (Rust) + npm (Node.js)
- **UI Framework**: Custom CSS with responsive design
- **Data Storage**: Local file system

### Features

- Local data storage with no network dependencies
- Cross-platform support (Windows)
- Responsive UI with drag-and-drop and popup notifications
- Task management (add, delete, mark as complete)
- Task categorization and status management
- Deadline tracking with visual indicators
- System tray integration and quick actions
- Window control (transparency, position management)
- Settings management (auto-start, window position, etc.)

### User Guide

#### 📝 Task Management

- **Create Task**: Enter task content in bottom input, press Enter or click "+" button
- **Complete Task**: Hover over task and click "✓" button on the right, task moves to completed group
- **Uncomplete Task**: Click "↶" button in completed group to restore task
- **Delete Task**: Double-click task item for quick deletion
- **Edit Task**: Right-click task → select "✏️ Edit Task" to modify content
- **Drag to Reorder**: Hover over task and drag "☰" button to adjust order

#### 📁 Group Management

- **Create Group**: Click "📁" button at bottom, enter group name
- **Rename Group**: Right-click group header → select "✏️ Rename Group"
- **Delete Group**: Right-click group header → select "🗑️ Delete Group" (tasks move to ungrouped)
- **Collapse/Expand**: Click "▼" icon on the left of group header
- **Adjust Order**: Hover over group header, click "▲▼" buttons to move group up/down

#### 🔄 Task Dragging

- **Drag Within Group**: Hold task's "☰" button and drag up/down within the same group
- **Drag Between Groups**:
  - Drag to target group's task list to insert at specific position
  - Drag to target group's header to add task at the end
- **Drag Indicator**: Target area shows blue highlight during dragging

#### ⏰ Time Management

- **Set Deadline**: Right-click task → "📅 Set Deadline" → select date and time
- **Remove Deadline**: Right-click task → "🗑️ Remove Deadline"
- **Time Indicators**:
  - 🟢 **Green**: Sufficient time until deadline (hover to see deadline date)
  - 🔴 **Red**: Past deadline (hover to see overdue status)
  - 🟡 **Yellow**: Task created multiple days ago (hover to see days created)

#### ✅ Completed Tasks

- **View Completed**: Click "Completed" group at bottom to expand
- **Clear Completed**: Click trash icon on the right of completed group to clear all
- **Restore Task**: Click "↶" button on completed task to restore to active state

#### ⚙️ Other Features

- **Task Statistics**: Top bar shows "Completed/Total" task count
- **Context Menu**: Right-click tasks or groups for quick access to more actions
- **Settings Panel**: Click "⚙️" button in top-right corner to open settings

### Prerequisites

- Node.js (version ^20.19.0 or >=22.12.0)
- npm or pnpm
- Rust toolchain (rustup, cargo)

### Development Setup

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd DeskHive
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run in development mode:
   ```bash
   npm run dev
   ```

### Building for Production

1. Build the application:
   ```bash
   npm run tauri build
   ```

2. The built application will be located in:
   - Windows: `src-tauri/target/release/bundle/nsis/` (`.exe`)
   - macOS: `src-tauri/target/release/bundle/macos/` (`.app`)
   - Linux: `src-tauri/target/release/bundle/appimage/` (`.AppImage`)

