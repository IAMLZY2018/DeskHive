<template>
  <div class="container">
    <header :data-tauri-drag-region="!isDragDisabled ? '' : null">
      <div class="header-title" :data-tauri-drag-region="!isDragDisabled ? '' : null">
        <img src="/src-tauri/icons/32x32.png" alt="DeskHive" class="app-icon">
        DeskHive
      </div>
      <div class="header-right">
        <div class="progress-indicator">{{ completedTasks }}/{{ totalTasks }}</div>
        <button class="settings-btn" @click="openSettings">⚙️</button>
      </div>
    </header>

    <div class="todo-container">
      <!-- 空状态显示日期信息 -->
      <EmptyState 
        v-if="showEmptyState && dateInfo" 
        :date-info="dateInfo"
      />

      <!-- 全部任务完成状态 -->
      <AllCompletedState v-if="showAllCompletedState" />

      <!-- 待完成任务列表 -->
      <div v-if="!showEmptyState && !showAllCompletedState" class="todo-section">
        <h3 class="section-title">待完成</h3>
        <TodoList
          :todos="sortedPendingTodos"
          @toggle="toggleTodo"
          @delete="deleteTodo"
          @contextmenu="showContextMenuFor"
        />
      </div>
      
      <!-- 已完成任务列表 -->
      <div v-if="completedTodos.length > 0" class="todo-section completed-section">
        <h3 class="section-title">已完成</h3>
        <TodoList
          :todos="completedTodos"
          :is-completed-list="true"
          @toggle="toggleCompletedTodo"
          @delete="deleteCompletedTodo"
          @contextmenu="showContextMenuFor"
        />
      </div>
    </div>
    
    <AddTask @add-task="addTask" />
    
    <!-- 右键菜单 -->
    <ContextMenu
      :show="showContextMenu"
      :position="contextMenuPosition"
      :todo="contextMenuTodo"
      @set-deadline="openDeadlineDialog"
      @remove-deadline="removeDeadline"
    />
    
    <!-- 截止时间设置对话框 -->
    <DeadlineDialog
      :show="showDeadlineDialog"
      :initial-date="deadlineDate"
      :initial-time="deadlineTime"
      @close="closeDeadlineDialog"
      @confirm="setDeadline"
    />
    
    <!-- Toast 内部提示 -->
    <Toast
      :show="showToast"
      :message="toastMessage"
      :type="toastType"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import type { Todo, DateInfo } from './types';
import EmptyState from './components/EmptyState.vue';
import AllCompletedState from './components/AllCompletedState.vue';
import TodoList from './components/TodoList.vue';
import AddTask from './components/AddTask.vue';
import ContextMenu from './components/ContextMenu.vue';
import DeadlineDialog from './components/DeadlineDialog.vue';
import Toast from './components/Toast.vue';

const pendingTodos = ref<Todo[]>([]);
const completedTodos = ref<Todo[]>([]);
const dateInfo = ref<DateInfo | null>(null);

const totalTasks = computed(() => pendingTodos.value.length + completedTodos.value.length);
const completedTasks = computed(() => completedTodos.value.length);

// 计算排序后的待办任务
const sortedPendingTodos = computed(() => {
  return [...pendingTodos.value].sort((a, b) => {
    // 如果两个任务都没有截止时间，保持原有顺序
    if (!a.deadline && !b.deadline) {
      return 0;
    }
    
    // 如果只有任务a没有截止时间，将a排在前面
    if (!a.deadline) {
      return -1;
    }
    
    // 如果只有任务b没有截止时间，将b排在前面
    if (!b.deadline) {
      return 1;
    }
    
    // 如果两个任务都有截止时间，按照截止时间排序（快到期的在前面）
    return a.deadline - b.deadline;
  });
});

// 计算是否显示空状态（没有任何任务）
const showEmptyState = computed(() => pendingTodos.value.length === 0 && completedTodos.value.length === 0);
// 计算是否显示全部完成状态（只有已完成任务，没有待完成任务）
const showAllCompletedState = computed(() => pendingTodos.value.length === 0 && completedTodos.value.length > 0);

// 拖动设置状态
const isDragDisabled = ref(false);

// 右键菜单状态
const showContextMenu = ref(false);
const contextMenuPosition = ref({ x: 0, y: 0 });
const contextMenuTodo = ref<Todo | null>(null);

// 截止时间设置对话框状态
const showDeadlineDialog = ref(false);
const deadlineDate = ref('');
const deadlineTime = ref('');
const dialogTodo = ref<Todo | null>(null);

// 内部提示弹窗状态
const showToast = ref(false);
const toastMessage = ref('');
const toastType = ref<'error' | 'success' | 'warning'>('error');

// 倒计时更新定时器
const countdownTimer = ref<number | null>(null);

// 阻止浏览器默认右键菜单
function preventDefaultContextMenu(event: MouseEvent) {
  // 检查事件目标是否是todo项（通过类名判断）
  const target = event.target as HTMLElement;
  if (!target.closest('.todo-item')) {
    event.preventDefault();
  }
}

// 格式化时间
function formatDateTime(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false
  });
}

// 获取当前日期信息
async function loadDateInfo() {
  try {
    const data = await invoke('get_current_date') as DateInfo;
    dateInfo.value = data;
    console.log('日期信息加载成功:', data);
  } catch (error) {
    console.error('加载日期信息失败:', error);
    // 如果加载失败，使用默认日期信息
    dateInfo.value = {
      solar_date: '2024年8月28日',
      lunar_date: '甲辰年七月廿五',
      weekday: '星期三',
      lunar_year: '甲辰年',
      lunar_month: '七月',
      lunar_day: '廿五'
    };
  }
}

// 显示 Toast 提示
function showToastMessage(message: string, type: 'error' | 'success' | 'warning' = 'error') {
  toastMessage.value = message;
  toastType.value = type;
  showToast.value = true;
  
  // 3秒后自动隐藏
  setTimeout(() => {
    showToast.value = false;
  }, 3000);
}

// 计算创建天数
function calculateDaysCreated(timestamp: number): number {
  const now = Date.now();
  const createdTime = timestamp * 1000; // 转换为毫秒
  const diffMs = now - createdTime;
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));
  return diffDays;
}

// 判断是否已过期
function isOverdue(deadline: number): boolean {
  const now = Math.floor(Date.now() / 1000); // 当前时间戳（秒）
  return deadline < now;
}

// 判断任务是否创建于前一天
function isCreatedYesterday(createdAt: number): boolean {
  // 获取当前日期的开始时间戳（今天00:00:00）
  const today = new Date();
  today.setHours(0, 0, 0, 0);
  const todayTimestamp = Math.floor(today.getTime() / 1000);
  
  // 获取任务创建日期的开始时间戳
  const createdDate = new Date(createdAt * 1000);
  createdDate.setHours(0, 0, 0, 0);
  const createdTimestamp = Math.floor(createdDate.getTime() / 1000);
  
  // 计算两个日期相差的天数
  const diffDays = Math.floor((todayTimestamp - createdTimestamp) / (24 * 60 * 60));
  
  // 如果相差1天，则说明是昨天创建的任务
  return diffDays === 1;
}

// 清除前一天完成的任务
function clearYesterdayCompletedTodos() {
  // 过滤掉昨天完成的任务
  const filteredTodos = completedTodos.value.filter(todo => {
    // 保留今天及以前完成的任务
    return !isCreatedYesterday(todo.createdAt);
  });
  
  // 如果有任务被清除，则更新列表并保存数据
  if (filteredTodos.length !== completedTodos.value.length) {
    completedTodos.value = filteredTodos;
    saveTodoData();
    console.log('已清除前一天完成的任务');
  }
}

// 计算倒计时文本（精确到分钟）
function getCountdownText(deadline: number): string {
  const now = Math.floor(Date.now() / 1000); // 当前时间戳（秒）
  const diff = deadline - now; // 剩余秒数
  
  if (diff <= 0) {
    // 已过期
    const overdueDiff = Math.abs(diff);
    if (overdueDiff < 60) {
      return '已超时';
    } else if (overdueDiff < 3600) {
      const minutes = Math.floor(overdueDiff / 60);
      return `${minutes}分钟`;
    } else if (overdueDiff < 86400) {
      const hours = Math.floor(overdueDiff / 3600);
      const minutes = Math.floor((overdueDiff % 3600) / 60);
      return minutes > 0 ? `${hours}时${minutes}分` : `${hours}时`;
    } else {
      const days = Math.floor(overdueDiff / 86400);
      const hours = Math.floor((overdueDiff % 86400) / 3600);
      return hours > 0 ? `${days}天${hours}时` : `${days}天`;
    }
  }
  
  // 未过期，显示剩余时间（精确到分钟）
  if (diff < 60) {
    return '即将到期';
  } else if (diff < 3600) {
    const minutes = Math.floor(diff / 60);
    return `${minutes}分钟`;
  } else if (diff < 86400) {
    const hours = Math.floor(diff / 3600);
    const minutes = Math.floor((diff % 3600) / 60);
    return minutes > 0 ? `${hours}时${minutes}分` : `${hours}时`;
  } else {
    const days = Math.floor(diff / 86400);
    const hours = Math.floor((diff % 86400) / 3600);
    return hours > 0 ? `${days}天${hours}时` : `${days}天`;
  }
}

// 启动倒计时更新定时器
function startCountdownTimer() {
  if (countdownTimer.value) {
    clearInterval(countdownTimer.value);
  }
  
  // 每分钟更新一次
  countdownTimer.value = window.setInterval(() => {
    // 触发组件重新渲染，让倒计时更新
    // 通过修改一个小的响应式变量来触发重新渲染
    // 这里不需要额外变量，直接让Vue检测到时间变化即可
  }, 60000); // 60秒 = 1分钟
}

// 保存数据到本地文件
async function saveTodoData() {
  try {
    // 转换为后端格式（使用下划线命名）
    const pendingTodosForBackend = pendingTodos.value.map(todo => ({
      text: todo.text,
      completed: todo.completed,
      created_at: todo.createdAt,
      deadline: todo.deadline || null // 处理可选的deadline字段
    }));
    const completedTodosForBackend = completedTodos.value.map(todo => ({
      text: todo.text,
      completed: todo.completed,
      created_at: todo.createdAt,
      deadline: todo.deadline || null // 处理可选的deadline字段
    }));
    
    await invoke('save_todo_data', {
      pendingTodos: pendingTodosForBackend,
      completedTodos: completedTodosForBackend
    });
    console.log('数据保存成功');
  } catch (error) {
    console.error('保存数据失败:', error);
  }
}

// 从本地文件加载数据
async function loadTodoData() {
  try {
    const data = await invoke('load_todo_data') as {
      pending_todos: { text: string; completed: boolean; created_at: number; deadline?: number }[],
      completed_todos: { text: string; completed: boolean; created_at: number; deadline?: number }[]
    };
    // 转换数据格式（后端使用下划线命名，前端使用驼峰命名）
    pendingTodos.value = data.pending_todos.map(todo => ({
      text: todo.text,
      completed: todo.completed,
      createdAt: todo.created_at,
      deadline: todo.deadline // 处理可选的deadline字段
    }));
    completedTodos.value = data.completed_todos.map(todo => ({
      text: todo.text,
      completed: todo.completed,
      createdAt: todo.created_at,
      deadline: todo.deadline // 处理可选的deadline字段
    }));
    console.log('数据加载成功');
  } catch (error) {
    console.error('加载数据失败:', error);
    // 如果加载失败，使用默认数据
    const now = Math.floor(Date.now() / 1000); // 当前时间戳（秒）
    pendingTodos.value = [
      { text: '学习SpringBoot3.5', completed: false, createdAt: now - 3600 },
      { text: '测试部署到服务器', completed: false, createdAt: now - 1800 }
    ];
    completedTodos.value = [
      { text: '完成UI设计', completed: true, createdAt: now - 7200 }
    ];
  }
}

async function openSettings() {
  try {
    await invoke('open_settings_window');
  } catch (error) {
    console.error('打开设置窗口失败:', error);
  }
}

function addTask(text: string) {
  const now = Math.floor(Date.now() / 1000); // 当前时间戳（秒）
  pendingTodos.value.push({
    text: text,
    completed: false,
    createdAt: now
  });
  // 保存数据
  saveTodoData();
}

function toggleTodo(index: number) {
  const todo = pendingTodos.value[index];
  pendingTodos.value.splice(index, 1);
  completedTodos.value.push({
    text: todo.text,
    completed: true,
    createdAt: todo.createdAt, // 保持原有的创建时间
    deadline: todo.deadline // 保持原有的截止时间
  });
  // 保存数据
  saveTodoData();
}

function toggleCompletedTodo(index: number) {
  const todo = completedTodos.value[index];
  completedTodos.value.splice(index, 1);
  pendingTodos.value.push({
    text: todo.text,
    completed: false,
    createdAt: todo.createdAt, // 保持原有的创建时间
    deadline: todo.deadline // 保持原有的截止时间
  });
  // 保存数据
  saveTodoData();
}

function deleteTodo(index: number) {
  pendingTodos.value.splice(index, 1);
  // 保存数据
  saveTodoData();
}

// 显示右键菜单
function showContextMenuFor(event: MouseEvent, todo: Todo) {
  event.preventDefault();
  contextMenuTodo.value = todo;
  contextMenuPosition.value = { x: event.clientX, y: event.clientY };
  showContextMenu.value = true;
  
  // 监听点击事件以隐藏菜单
  document.addEventListener('click', hideContextMenu);
}

// 隐藏右键菜单
function hideContextMenu() {
  showContextMenu.value = false;
  contextMenuTodo.value = null;
  document.removeEventListener('click', hideContextMenu);
}

// 打开截止时间设置对话框
function openDeadlineDialog() {
  if (!contextMenuTodo.value) return;
  
  dialogTodo.value = contextMenuTodo.value;
  
  // 如果已有截止时间，预填表单
  if (contextMenuTodo.value.deadline) {
    const deadlineDateTime = new Date(contextMenuTodo.value.deadline * 1000);
    deadlineDate.value = deadlineDateTime.toISOString().split('T')[0];
    deadlineTime.value = deadlineDateTime.toTimeString().slice(0, 5);
  } else {
    // 默认设置为今天晚上6点
    const now = new Date();
    deadlineDate.value = now.toISOString().split('T')[0];
    deadlineTime.value = '18:00';
  }
  
  hideContextMenu();
  showDeadlineDialog.value = true;
}

// 关闭截止时间设置对话框
function closeDeadlineDialog() {
  showDeadlineDialog.value = false;
  dialogTodo.value = null;
  deadlineDate.value = '';
  deadlineTime.value = '';
}

// 设置截止时间
async function setDeadline() {
  if (!dialogTodo.value || !deadlineDate.value || !deadlineTime.value) {
    showToastMessage('请选择日期和时间', 'warning');
    return;
  }
  
  // 合并日期和时间，转换为Unix时间戳
  const deadlineDateTime = new Date(`${deadlineDate.value}T${deadlineTime.value}`);
  const deadlineTimestamp = Math.floor(deadlineDateTime.getTime() / 1000);
  
  // 检查时间是否在未来
  const now = Math.floor(Date.now() / 1000);
  if (deadlineTimestamp <= now) {
    showToastMessage('截止时间必须在未来', 'error');
    return;
  }
  
  try {
    // 调用后端命令设置截止时间
    await invoke('set_todo_deadline', {
      todoText: dialogTodo.value.text,
      isCompleted: dialogTodo.value.completed,
      deadline: deadlineTimestamp
    });
    
    // 更新本地数据
    const targetList = dialogTodo.value.completed ? completedTodos.value : pendingTodos.value;
    const todoIndex = targetList.findIndex(t => t.text === dialogTodo.value!.text);
    if (todoIndex !== -1) {
      targetList[todoIndex].deadline = deadlineTimestamp;
    }
    
    console.log('截止时间设置成功');
    showToastMessage('截止时间设置成功', 'success');
    closeDeadlineDialog();
  } catch (error) {
    console.error('设置截止时间失败:', error);
    showToastMessage('设置失败，请重试', 'error');
  }
}

// 移除截止时间
async function removeDeadline() {
  if (!contextMenuTodo.value) return;
  
  console.log('准备移除截止时间:', {
    text: contextMenuTodo.value.text,
    completed: contextMenuTodo.value.completed,
    hasDeadline: !!contextMenuTodo.value.deadline
  });
  
  try {
    // 调用后端命令移除截止时间（后端会直接保存数据到文件）
    await invoke('set_todo_deadline', {
      todoText: contextMenuTodo.value.text,
      isCompleted: contextMenuTodo.value.completed,
      deadline: null
    });
    
    // 更新本地数据以保持UI同步
    let found = false;
    const lists = [
      { list: pendingTodos.value },
      { list: completedTodos.value }
    ];
    
    // 遍历所有列表查找匹配的todo项
    for (const { list } of lists) {
      // 首先尝试通过文本和完成状态匹配
      const todoIndex = list.findIndex(t => 
        t.text === contextMenuTodo.value!.text && 
        t.completed === contextMenuTodo.value!.completed
      );
      
      if (todoIndex !== -1) {
        console.log('通过文本和完成状态找到todo项，更新前的deadline:', list[todoIndex].deadline);
        list[todoIndex].deadline = undefined;
        console.log('更新后的deadline:', list[todoIndex].deadline);
        found = true;
        break;
      }
      
      // 如果没有找到，尝试通过文本和创建时间匹配（更精确的匹配方式）
      const todoIndexByTime = list.findIndex(t => 
        t.text === contextMenuTodo.value!.text && 
        t.createdAt === contextMenuTodo.value!.createdAt
      );
      
      if (todoIndexByTime !== -1) {
        console.log('通过文本和创建时间找到todo项，更新前的deadline:', list[todoIndexByTime].deadline);
        list[todoIndexByTime].deadline = undefined;
        console.log('更新后的deadline:', list[todoIndexByTime].deadline);
        found = true;
        break;
      }
    }
    
    if (!found) {
      console.warn('在本地数据中未找到对应的todo项，重新加载数据');
      // 如果本地数据不一致，重新加载数据以确保同步
      await loadTodoData();
    }
    
    console.log('截止时间移除成功');
    showToastMessage('截止时间移除成功', 'success');
    hideContextMenu();
  } catch (error) {
    console.error('移除截止时间失败:', error);
    console.error('错误详情:', {
      message: (error as any)?.message || String(error),
      todoText: contextMenuTodo.value?.text,
      isCompleted: contextMenuTodo.value?.completed
    });
    
    // 即使前端报错，也重新加载数据确保同步
    try {
      await loadTodoData();
      hideContextMenu();
      showToastMessage('截止时间移除成功', 'success');
    } catch (reloadError) {
      console.error('重新加载数据失败:', reloadError);
      showToastMessage('移除失败，请重试', 'error');
    }
  }
}

function deleteCompletedTodo(index: number) {
  completedTodos.value.splice(index, 1);
  // 保存数据
  saveTodoData();
}

// 加载应用设置
async function loadAppSettings() {
  try {
    const settings = await invoke('load_app_settings') as {
      opacity: number,
      disable_drag: boolean,
      auto_show: boolean,
      minimize_to_tray: boolean,
      hotkey: string
    };
    isDragDisabled.value = settings.disable_drag;
    console.log('应用设置加载成功，拖动禁用状态:', isDragDisabled.value);
  } catch (error) {
    console.error('加载应用设置失败:', error);
  }
}

// 组件挂载时加载数据
onMounted(async () => {
  console.log('前端渲染完成');
  console.log('Vue 组件已挂载');
  console.log('待办事项数量:', pendingTodos.value.length);
  
  // 添加全局右键菜单阻止事件监听器
  document.addEventListener('contextmenu', preventDefaultContextMenu);
  
  // 加载todo数据、应用设置和日期信息
  await loadTodoData();
  
  // 清除前一天完成的任务
  clearYesterdayCompletedTodos();
  
  await loadAppSettings();
  await loadDateInfo();
  
  // 启动倒计时更新定时器
  startCountdownTimer();
  
  // 监听拖动设置变化
  const currentWindow = getCurrentWindow();
  await currentWindow.listen('drag-setting-changed', (event) => {
    isDragDisabled.value = event.payload as boolean;
    console.log('拖动设置已更新:', isDragDisabled.value);
  });
});

// 组件卸载时清理定时器和事件监听器
onUnmounted(() => {
  if (countdownTimer.value) {
    clearInterval(countdownTimer.value);
    countdownTimer.value = null;
  }
  
  // 移除全局右键菜单阻止事件监听器
  document.removeEventListener('contextmenu', preventDefaultContextMenu);
});
</script>

<style>
* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}
html, body, #app {
  width: 100%;
  height: 100%;
  overflow: hidden;
}
body {
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  background: #e8e4f3;
  color: #333;
}
#app {
  display: flex;
  justify-content: center;
  align-items: center;
}
.container {
  width: 100%;
  height: 100%;
  background: #ffffff;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border-radius: 0;
}
header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: clamp(6px, 1.2vh, 8px) clamp(10px, 2.5vw, 14px);
  font-size: clamp(0.85rem, 2.2vw, 0.95rem);
  background: rgba(255, 255, 255, 0.6);
  border-bottom: 1px solid rgba(104, 58, 183, 0.1);
  color: #333;
  font-weight: 600;
  backdrop-filter: blur(10px);
  min-height: clamp(30px, 5vh, 36px);
}
.header-title {
  flex: 1;
  text-align: center;
  cursor: default;
  user-select: none;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}
.app-icon {
  width: 20px;
  height: 20px;
  flex-shrink: 0;
}
.header-right {
  display: flex;
  align-items: center;
  gap: clamp(6px, 1.5vw, 10px);
}
.progress-indicator {
  font-size: clamp(0.7rem, 1.8vw, 0.8rem);
  color: #333;
  font-weight: 600;
  background: rgba(255, 255, 255, 0.8);
  padding: clamp(2px, 0.5vh, 4px) clamp(6px, 1.5vw, 8px);
  border-radius: clamp(8px, 1.5vw, 12px);
  border: 1px solid rgba(255, 255, 255, 0.4);
  min-width: clamp(28px, 6vw, 35px);
  text-align: center;
  backdrop-filter: blur(5px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}
.settings-btn {
  width: clamp(24px, 4.5vw, 28px);
  height: clamp(24px, 4.5vw, 28px);
  border: none;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.8);
  color: #333;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: clamp(0.65rem, 1.8vw, 0.8rem);
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  flex-shrink: 0;
  backdrop-filter: blur(5px);
  border: 1px solid rgba(255, 255, 255, 0.4);
}
.settings-btn:hover {
  transform: rotate(90deg) scale(1.08);
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.15);
  background: rgba(255, 255, 255, 0.95);
}
.todo-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow-y: auto;
  background: transparent;
  min-height: 0;
}

.todo-container::-webkit-scrollbar {
  width: 5px;
}

.todo-container::-webkit-scrollbar-track {
  background: rgba(104, 58, 183, 0.1);
  border-radius: 3px;
}

.todo-container::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.8);
  border-radius: 3px;
  border: 1px solid rgba(255, 255, 255, 0.4);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
}

.todo-section {
  margin-bottom: 10px;
  padding: clamp(8px, 2vh, 12px);
}

.section-title {
  font-size: clamp(0.8rem, 2.2vw, 0.9rem);
  color: #555;
  margin-bottom: 8px;
  font-weight: 600;
  padding-left: 5px;
}

.completed-section {
  margin-top: auto;
  border-top: 1px dashed rgba(104, 58, 183, 0.2);
  padding-top: 10px;
}

.todo-list {
  min-height: 0;
}
.todo-item {
  background: rgba(255, 255, 255, 0.7);
  padding: clamp(6px, 1.5vh, 10px) clamp(8px, 2vw, 12px);
  margin-bottom: clamp(4px, 1vh, 8px);
  border-radius: clamp(8px, 1.5vw, 12px);
  display: flex;
  align-items: center;
  box-shadow: 0 4px 16px rgba(104, 58, 183, 0.1);
  transition: all 0.3s ease;
  border: 1px solid rgba(104, 58, 183, 0.08);
  backdrop-filter: blur(10px);
  min-height: clamp(30px, 5vh, 36px);
  cursor: pointer;
  position: relative;
}
.todo-item:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(104, 58, 183, 0.15);
  border-color: rgba(104, 58, 183, 0.2);
}

.todo-item:hover::after {
  content: '双击删除';
  position: absolute;
  right: 10px;
  font-size: 0.7rem;
  color: #888;
  opacity: 0.7;
}
.todo-checkbox {
  width: clamp(14px, 2.5vw, 18px);
  height: clamp(14px, 2.5vw, 18px);
  border: 1px solid rgba(255, 255, 255, 0.4);
  border-radius: 50%;
  margin-right: clamp(6px, 1.5vw, 10px);
  cursor: pointer;
  transition: all 0.3s ease;
  flex-shrink: 0;
  position: relative;
  background: rgba(255, 255, 255, 0.6);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  backdrop-filter: blur(5px);
}
.todo-checkbox.completed {
  background: rgba(255, 255, 255, 0.8);
  border-color: rgba(76, 175, 80, 0.8);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  backdrop-filter: blur(5px);
}
.todo-checkbox.completed::after {
  content: '✓';
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  color: #333;
  font-size: clamp(8px, 1.5vw, 10px);
  font-weight: bold;
}
.todo-item span {
  font-size: clamp(0.75rem, 2vw, 0.85rem);
  word-break: break-word;
  flex: 1;
  line-height: 1.2;
  font-weight: 500;
  color: #333;
  user-select: none;
}
.todo-item.completed span {
  text-decoration: line-through;
  color: #888;
}
.add-task {
  display: flex;
  padding: clamp(6px, 1.5vh, 10px);
  border-top: 1px solid rgba(104, 58, 183, 0.1);
  background: rgba(255, 255, 255, 0.6);
  gap: clamp(6px, 1.2vw, 10px);
  backdrop-filter: blur(10px);
  min-height: clamp(34px, 6vh, 42px);
  align-items: center;
}
.add-task input {
  flex: 1;
  padding: clamp(6px, 1.5vh, 8px) clamp(8px, 2vw, 10px);
  border: 1px solid rgba(104, 58, 183, 0.2);
  border-radius: clamp(8px, 1.5vw, 12px);
  outline: none;
  background: rgba(255, 255, 255, 0.8);
  color: #333;
  font-size: clamp(0.75rem, 2vw, 0.85rem);
  transition: all 0.3s ease;
  height: clamp(28px, 4.5vh, 32px);
}
.add-task input:focus {
  border-color: #8e7cc3;
  box-shadow: 0 0 8px rgba(104, 58, 183, 0.2);
  background: rgba(255, 255, 255, 0.95);
}
.add-task input::placeholder {
  color: rgba(51, 51, 51, 0.5);
}
.add-task button {
  width: clamp(28px, 4.5vh, 32px);
  height: clamp(28px, 4.5vh, 32px);
  background: rgba(255, 255, 255, 0.8);
  border: none;
  border-radius: 50%;
  cursor: pointer;
  color: #333;
  font-weight: 700;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  transition: all 0.3s ease;
  font-size: clamp(0.7rem, 2vw, 0.85rem);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  backdrop-filter: blur(5px);
  border: 1px solid rgba(255, 255, 255, 0.4);
}
.add-task button:hover {
  transform: translateY(-1px);
  box-shadow: 0 6px 16px rgba(104, 58, 183, 0.4);
}
.add-task button:active {
  transform: translateY(0);
}
.add-task button:hover {
  background: rgba(255, 255, 255, 0.95);
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.15);
  transform: translateY(-2px);
}

/* Todo列表过渡动画 */
.todo-list-enter-active,
.todo-list-leave-active {
  transition: all 0.5s ease;
}
.todo-list-enter-from {
  opacity: 0;
  transform: translateY(30px);
}
.todo-list-leave-to {
  opacity: 0;
  transform: translateX(100px);
}
.todo-list-move {
  transition: transform 0.5s ease;
}

/* 右键菜单样式 */
.context-menu {
  position: fixed;
  background: rgba(255, 255, 255, 0.95);
  border: 1px solid rgba(104, 58, 183, 0.2);
  border-radius: 8px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
  backdrop-filter: blur(10px);
  z-index: 1000;
  min-width: 200px;
  padding: 8px;
}

.context-menu-item {
  padding: 8px 12px;
  font-size: 0.85rem;
  color: #333;
}

.context-menu-label {
  font-weight: 600;
  color: #555;
  margin-bottom: 4px;
}

.context-menu-value {
  font-family: 'Courier New', monospace;
  background: rgba(104, 58, 183, 0.1);
  padding: 4px 8px;
  border-radius: 4px;
  border: 1px solid rgba(104, 58, 183, 0.2);
  font-size: 0.8rem;
}

.status-value {
  display: flex;
  align-items: center;
  gap: 6px;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  display: inline-block;
  flex-shrink: 0;
}

.status-dot.completed {
  background-color: #4CAF50; /* 绿色 */
  box-shadow: 0 0 4px rgba(76, 175, 80, 0.5);
}

.status-dot.pending {
  background-color: #F44336; /* 红色 */
  box-shadow: 0 0 4px rgba(244, 67, 54, 0.5);
}

/* 天数指示器样式 */
.days-indicator {
  width: clamp(18px, 3vw, 22px);
  height: clamp(18px, 3vw, 22px);
  background: #FFE082; /* 更淡的黄色 */
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: clamp(0.6rem, 1.5vw, 0.7rem);
  font-weight: bold;
  color: #333;
  margin-right: clamp(6px, 1.5vw, 8px);
  flex-shrink: 0;
  box-shadow: 0 2px 6px rgba(255, 224, 130, 0.4);
  border: 1px solid rgba(255, 224, 130, 0.6);
  user-select: none;
}

/* 倒计时指示器样式 */
.countdown-indicator {
  background: #4CAF50; /* 绿色椭圆 */
  color: white;
  border-radius: 12px;
  padding: clamp(2px, 0.5vh, 4px) clamp(6px, 1.2vw, 8px);
  font-size: clamp(0.6rem, 1.3vw, 0.7rem);
  font-weight: bold;
  margin-right: clamp(6px, 1.5vw, 8px);
  border: 1px solid rgba(76, 175, 80, 0.6);
  box-shadow: 0 2px 8px rgba(76, 175, 80, 0.3);
  backdrop-filter: blur(3px);
  flex-shrink: 0;
  white-space: nowrap;
  min-width: fit-content;
  transition: all 0.3s ease;
  user-select: none;
}

/* 过期倒计时指示器样式 */
.countdown-indicator.overdue {
  background: #F44336; /* 红色 */
  border-color: rgba(244, 67, 54, 0.6);
  box-shadow: 0 2px 8px rgba(244, 67, 54, 0.3);
}

/* 已完成的倒计时指示器 */
.countdown-indicator.completed {
  background: #9E9E9E; /* 灰色 */
  border-color: rgba(158, 158, 158, 0.6);
  box-shadow: 0 2px 8px rgba(158, 158, 158, 0.3);
  opacity: 0.8;
}

/* 已完成且过期的倒计时指示器 */
.countdown-indicator.completed.overdue {
  background: #9E9E9E; /* 灰色 */
  border-color: rgba(158, 158, 158, 0.6);
  box-shadow: 0 2px 8px rgba(158, 158, 158, 0.3);
  opacity: 0.8;
}

/* 右键菜单分割线 */
.context-menu-divider {
  height: 1px;
  background: rgba(104, 58, 183, 0.2);
  margin: 8px 0;
}

/* 右键菜单按钮 */
.context-menu-button {
  padding: 8px 12px;
  font-size: 0.85rem;
  color: #333;
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.2s ease;
  user-select: none;
}

.context-menu-button:hover {
  background: rgba(104, 58, 183, 0.1);
  color: #683ab7;
}

/* 对话框遮罩 */
.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
  backdrop-filter: blur(3px);
}

/* 对话框主体 */
.dialog-box {
  background: rgba(255, 255, 255, 0.95);
  border-radius: 12px;
  padding: 24px;
  box-shadow: 0 12px 32px rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(104, 58, 183, 0.2);
  min-width: 300px;
  max-width: 400px;
}

/* 对话框标题 */
.dialog-title {
  font-size: 1.1rem;
  font-weight: 600;
  color: #333;
  margin-bottom: 20px;
  text-align: center;
}

/* 对话框内容 */
.dialog-content {
  margin-bottom: 24px;
}

/* 输入组 */
.input-group {
  margin-bottom: 16px;
}

.input-group label {
  display: block;
  margin-bottom: 6px;
  font-size: 0.9rem;
  font-weight: 500;
  color: #555;
}

/* 对话框输入框 */
.dialog-input {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid rgba(104, 58, 183, 0.3);
  border-radius: 8px;
  outline: none;
  background: rgba(255, 255, 255, 0.9);
  color: #333;
  font-size: 0.9rem;
  transition: all 0.3s ease;
  box-sizing: border-box;
}

.dialog-input:focus {
  border-color: #683ab7;
  box-shadow: 0 0 8px rgba(104, 58, 183, 0.2);
  background: rgba(255, 255, 255, 1);
}

/* 对话框按钮组 */
.dialog-buttons {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
}

/* 对话框按钮 */
.dialog-btn {
  padding: 10px 20px;
  border: none;
  border-radius: 8px;
  font-size: 0.9rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
  backdrop-filter: blur(5px);
}

.dialog-btn.cancel {
  background: rgba(158, 158, 158, 0.2);
  color: #666;
  border: 1px solid rgba(158, 158, 158, 0.3);
}

.dialog-btn.cancel:hover {
  background: rgba(158, 158, 158, 0.3);
  color: #333;
}

.dialog-btn.confirm {
  background: #683ab7;
  color: white;
  border: 1px solid #683ab7;
}

.dialog-btn.confirm:hover {
  background: #5e35a1;
  box-shadow: 0 4px 12px rgba(104, 58, 183, 0.3);
}

/* 空状态日期信息样式 */
.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: clamp(20px, 4vh, 40px);
  text-align: center;
}

.date-info {
  background: rgba(255, 255, 255, 0.9);
  border-radius: clamp(12px, 2.5vw, 16px);
  padding: clamp(16px, 3vh, 24px);
  box-shadow: 0 6px 20px rgba(104, 58, 183, 0.15);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(104, 58, 183, 0.1);
  margin-bottom: clamp(16px, 3vh, 24px);
  min-width: clamp(200px, 40vw, 260px);
}

.solar-date {
  margin-bottom: clamp(12px, 2vh, 16px);
}

.date-main {
  font-size: clamp(1.2rem, 4vw, 1.5rem);
  font-weight: 700;
  color: #333;
  margin-bottom: clamp(4px, 0.8vh, 6px);
  letter-spacing: 1px;
}

.weekday {
  font-size: clamp(0.85rem, 2.2vw, 1rem);
  color: #683ab7;
  font-weight: 600;
}

.lunar-date {
  border-top: 1px dashed rgba(104, 58, 183, 0.2);
  padding-top: clamp(10px, 2vh, 12px);
}

.lunar-main {
  font-size: clamp(1rem, 3vw, 1.2rem);
  font-weight: 600;
  color: #555;
  margin-bottom: 0;
  font-family: 'Microsoft YaHei', '微软雅黑', sans-serif;
}

.welcome-text {
  font-size: clamp(0.9rem, 2.5vw, 1.1rem);
  color: #683ab7;
  font-weight: 600;
  background: rgba(104, 58, 183, 0.1);
  padding: clamp(8px, 1.5vh, 12px) clamp(16px, 3vw, 20px);
  border-radius: clamp(8px, 1.5vw, 12px);
  border: 1px solid rgba(104, 58, 183, 0.2);
  backdrop-filter: blur(5px);
  box-shadow: 0 2px 8px rgba(104, 58, 183, 0.1);
  animation: gentle-pulse 3s ease-in-out infinite;
}

/* 温和的脉动动画 */
@keyframes gentle-pulse {
  0%, 100% {
    transform: scale(1);
    opacity: 0.9;
  }
  50% {
    transform: scale(1.02);
    opacity: 1;
  }
}

/* 全部完成状态样式 */
.all-completed-state {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: clamp(30px, 5vh, 50px);
}

.celebration-message {
  font-size: clamp(1.1rem, 3vw, 1.3rem);
  font-weight: 700;
  color: #4CAF50;
  background: linear-gradient(135deg, rgba(76, 175, 80, 0.1), rgba(76, 175, 80, 0.2));
  padding: clamp(16px, 3vh, 24px) clamp(20px, 4vw, 32px);
  border-radius: clamp(12px, 2.5vw, 16px);
  border: 2px solid rgba(76, 175, 80, 0.3);
  backdrop-filter: blur(10px);
  box-shadow: 0 8px 24px rgba(76, 175, 80, 0.2);
  text-align: center;
  animation: celebration-bounce 0.8s ease-out;
  cursor: pointer;
  transition: all 0.3s ease;
}

.celebration-message:hover {
  animation: celebration-bounce 0.8s ease-out;
  box-shadow: 0 10px 28px rgba(76, 175, 80, 0.3);
  transform: translateY(-2px);
}

/* 庆祝弹跳动画 */
@keyframes celebration-bounce {
  0%, 20%, 50%, 80%, 100% {
    transform: translateY(0) scale(1);
  }
  40% {
    transform: translateY(-8px) scale(1.05);
  }
  60% {
    transform: translateY(-4px) scale(1.02);
  }
}

/* Toast 内部提示样式 */
.toast-notification {
  position: fixed;
  top: 20px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 3000;
  padding: 12px 20px;
  border-radius: 8px;
  backdrop-filter: blur(10px);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);
  font-size: 0.9rem;
  font-weight: 600;
  animation: toast-slide-in 0.3s ease-out;
  max-width: 300px;
  min-width: 200px;
}

.toast-success {
  background: rgba(76, 175, 80, 0.95);
  color: white;
  border: 1px solid rgba(76, 175, 80, 0.8);
}

.toast-error {
  background: rgba(244, 67, 54, 0.95);
  color: white;
  border: 1px solid rgba(244, 67, 54, 0.8);
}

.toast-warning {
  background: rgba(255, 193, 7, 0.95);
  color: #333;
  border: 1px solid rgba(255, 193, 7, 0.8);
}

.toast-content {
  display: flex;
  align-items: center;
  gap: 8px;
}

.toast-icon {
  font-size: 1.1rem;
  flex-shrink: 0;
}

.toast-message {
  flex: 1;
}

/* Toast 动画 */
@keyframes toast-slide-in {
  0% {
    opacity: 0;
    transform: translateX(-50%) translateY(-20px);
  }
  100% {
    opacity: 1;
    transform: translateX(-50%) translateY(0);
  }
}
</style>
