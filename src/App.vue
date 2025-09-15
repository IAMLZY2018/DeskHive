<template>
  <div class="container">
    <header :data-tauri-drag-region="!isDragDisabled ? '' : null">
      <div class="header-title" :data-tauri-drag-region="!isDragDisabled ? '' : null">
        <img src="/icons/app-icon.png" alt="DeskHive" class="app-icon">
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
        <h3 class="section-title collapsible" @click="toggleCompletedSection">
          <span class="collapse-indicator" :class="{ collapsed: isCompletedSectionCollapsed }">
            ▼
          </span>
          已完成
          <span class="completed-count">{{ completedTasks }}</span>
          <button 
            class="clear-completed-btn" 
            @click.stop="clearAllCompletedTodos"
            title="清除所有已完成任务"
          >
            🗑️
          </button>
        </h3>
        <TodoList
          v-show="!isCompletedSectionCollapsed"
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
      @delete-todo="deleteTodoFromContextMenu"
    />
    
    <!-- 截止时间设置对话框 -->
    <DeadlineDialog
      :show="showDeadlineDialog"
      :initial-date="deadlineDate"
      :initial-time="deadlineTime"
      @close="closeDeadlineDialog"
      @confirm="handleDeadlineConfirm"
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

// 添加已完成部分的折叠状态
const isCompletedSectionCollapsed = ref(true);

const totalTasks = computed(() => pendingTodos.value.length + completedTodos.value.length);
const completedTasks = computed(() => completedTodos.value.length);

// 添加切换已完成部分折叠状态的函数
function toggleCompletedSection() {
  isCompletedSectionCollapsed.value = !isCompletedSectionCollapsed.value;
}

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

// 保存数据到本地文件
async function saveTodoData() {
  try {
    // 转换为后端格式（使用下划线命名）
    const pendingTodosForBackend = pendingTodos.value.map(todo => ({
      id: todo.id, // 添加ID字段
      text: todo.text,
      completed: todo.completed,
      created_at: todo.createdAt,
      deadline: todo.deadline || null // 处理可选的deadline字段
    }));
    const completedTodosForBackend = completedTodos.value.map(todo => ({
      id: todo.id, // 添加ID字段
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
      pending_todos: { id?: string; text: string; completed: boolean; created_at: number; deadline?: number }[],
      completed_todos: { id?: string; text: string; completed: boolean; created_at: number; deadline?: number }[]
    };
    // 转换数据格式（后端使用下划线命名，前端使用驼峰命名）
    pendingTodos.value = data.pending_todos.map(todo => ({
      id: todo.id || generateUniqueId(), // 如果没有ID则生成新的
      text: todo.text,
      completed: todo.completed,
      createdAt: todo.created_at,
      deadline: todo.deadline // 处理可选的deadline字段
    }));
    completedTodos.value = data.completed_todos.map(todo => ({
      id: todo.id || generateUniqueId(), // 如果没有ID则生成新的
      text: todo.text,
      completed: todo.completed,
      createdAt: todo.created_at,
      deadline: todo.deadline // 处理可选的deadline字段
    }));
    console.log('数据加载成功');
  } catch (error) {
    console.error('加载数据失败:', error);
    // 如果加载失败，使用空数据
    pendingTodos.value = [];
    completedTodos.value = [];
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
    id: generateUniqueId(), // 添加唯一ID
    text: text,
    completed: false,
    createdAt: now
  });
  // 保存数据
  saveTodoData();
}

// 生成唯一ID的函数
function generateUniqueId(): string {
  // 生成真正的UUID v4
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
    const r = Math.random() * 16 | 0;
    const v = c == 'x' ? r : (r & 0x3 | 0x8);
    return v.toString(16);
  });
}

function toggleTodo(index: number) {
  // 通过ID找到原始任务，而不是使用排序后的索引
  const sortedTodo = sortedPendingTodos.value[index];
  const originalIndex = pendingTodos.value.findIndex(todo => todo.id === sortedTodo.id);
  
  if (originalIndex === -1) {
    console.error('未找到对应的任务');
    return;
  }
  
  const todo = pendingTodos.value[originalIndex];
  // 为了更好的动画效果，我们先在已完成列表中添加任务，然后再从待完成列表中移除
  completedTodos.value.push({
    id: todo.id, // 保持原有的ID
    text: todo.text,
    completed: true,
    createdAt: todo.createdAt, // 保持原有的创建时间
    deadline: todo.deadline // 保持原有的截止时间
  });
  
  // 使用nextTick确保DOM更新后再移除待完成任务
  setTimeout(() => {
    pendingTodos.value.splice(originalIndex, 1);
    // 保存数据
    saveTodoData();
  }, 10);
}

function toggleCompletedTodo(index: number) {
  const todo = completedTodos.value[index];
  // 为了更好的动画效果，我们先在待完成列表中添加任务，然后再从已完成列表中移除
  pendingTodos.value.push({
    id: todo.id, // 保持原有的ID
    text: todo.text,
    completed: false,
    createdAt: todo.createdAt, // 保持原有的创建时间
    deadline: todo.deadline // 保持原有的截止时间
  });
  
  // 使用setTimeout确保DOM更新后再移除已完成任务
  setTimeout(() => {
    completedTodos.value.splice(index, 1);
    // 保存数据
    saveTodoData();
  }, 10);
}

function deleteTodo(index: number) {
  pendingTodos.value.splice(index, 1);
  // 保存数据
  saveTodoData();
}

// 从右键菜单删除任务
function deleteTodoFromContextMenu() {
  if (!contextMenuTodo.value) return;
  
  // 根据任务状态在相应的列表中删除
  if (contextMenuTodo.value.completed) {
    // 在已完成列表中删除
    const index = completedTodos.value.findIndex(todo => todo.id === contextMenuTodo.value!.id);
    if (index !== -1) {
      completedTodos.value.splice(index, 1);
    }
  } else {
    // 在待完成列表中删除
    const index = pendingTodos.value.findIndex(todo => todo.id === contextMenuTodo.value!.id);
    if (index !== -1) {
      pendingTodos.value.splice(index, 1);
    }
  }
  
  // 隐藏菜单
  hideContextMenu();
  
  // 保存数据
  saveTodoData();
  
  // 显示提示
  showToastMessage('任务已删除', 'success');
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
    // 默认设置为1小时后的时间，使用规范的时间计算方法
    const now = new Date();
    const oneHourLater = new Date(now);
    oneHourLater.setHours(now.getHours() + 1);
    
    deadlineDate.value = oneHourLater.toISOString().split('T')[0];
    deadlineTime.value = `${oneHourLater.getHours().toString().padStart(2, '0')}:${oneHourLater.getMinutes().toString().padStart(2, '0')}`;
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

// 处理DeadlineDialog确认事件
function handleDeadlineConfirm(date: string, time: string) {
  console.log('处理DeadlineDialog确认事件:', { date, time });
  deadlineDate.value = date;
  deadlineTime.value = time;
  setDeadline(); // 调用原有的设置截止时间函数
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
  
  // 检查时间是否在未来（增加1分钟的容差，避免时间精度问题）
  const now = Math.floor(Date.now() / 1000);
  if (deadlineTimestamp <= now - 60) { // 允许1分钟内的误差
    showToastMessage('截止时间必须在未来', 'error');
    return;
  }
  
  try {
    // 调用后端命令设置截止时间，使用ID来精确匹配任务
    await invoke('set_todo_deadline', {
      todoId: dialogTodo.value.id, // 使用ID而不是文本
      isCompleted: dialogTodo.value.completed,
      deadline: deadlineTimestamp
    });
    
    // 更新本地数据
    const targetList = dialogTodo.value.completed ? completedTodos.value : pendingTodos.value;
    const todoIndex = targetList.findIndex(t => t.id === dialogTodo.value!.id);
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
    id: contextMenuTodo.value.id,
    text: contextMenuTodo.value.text,
    completed: contextMenuTodo.value.completed,
    hasDeadline: !!contextMenuTodo.value.deadline
  });
  
  try {
    // 调用后端命令移除截止时间（后端会直接保存数据到文件）
    await invoke('set_todo_deadline', {
      todoId: contextMenuTodo.value.id, // 使用ID而不是文本
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
      // 使用ID来精确匹配任务
      const todoIndex = list.findIndex(t => t.id === contextMenuTodo.value!.id);
      
      if (todoIndex !== -1) {
        console.log('通过ID找到todo项，更新前的deadline:', list[todoIndex].deadline);
        list[todoIndex].deadline = undefined;
        console.log('更新后的deadline:', list[todoIndex].deadline);
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
      todoId: contextMenuTodo.value?.id,
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

// 清除所有已完成任务
function clearAllCompletedTodos() {
  if (completedTodos.value.length === 0) return;
  
  completedTodos.value = [];
  // 保存数据
  saveTodoData();
  
  showToastMessage('已清除所有已完成任务', 'success');
}

// 加载应用设置
async function loadAppSettings() {
  try {
    const settings = await invoke('load_app_settings') as {
      opacity: number,
      disable_drag: boolean,
      auto_show: boolean,
      minimize_to_tray: boolean,
      hotkey: string,
      theme: string
    };
    isDragDisabled.value = settings.disable_drag;
    
    // 应用主题设置
    document.body.className = settings.theme === 'dark' ? 'dark-theme' : '';
    
    console.log('应用设置加载成功，拖动禁用状态:', isDragDisabled.value);
  } catch (error) {
    console.error('加载应用设置失败:', error);
  }
}

// 监听主题变化事件
async function listenThemeChange() {
  const currentWindow = getCurrentWindow();
  await currentWindow.listen('theme-changed', (event) => {
    const theme = event.payload as string;
    document.body.className = theme === 'dark' ? 'dark-theme' : '';
    console.log('主题已更新:', theme);
  });
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
  
  // 监听主题变化
  await listenThemeChange();
  
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
  border-bottom: 1px solid rgba(229, 231, 235, 0.2);
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
  border: 1px solid rgba(229, 231, 235, 0.2);
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
  border: 1px solid rgba(229, 231, 235, 0.2);
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
  background: rgba(229, 231, 235, 0.1);
  border-radius: 3px;
}

.todo-container::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.8);
  border-radius: 3px;
  border: 1px solid rgba(229, 231, 235, 0.2);
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

/* 可折叠标题样式 */
.section-title.collapsible {
  cursor: pointer;
  user-select: none;
  display: flex;
  justify-content: flex-start;
  align-items: center;
  gap: 8px;
}

.collapse-indicator {
  font-size: 0.7rem;
  transition: transform 0.3s ease;
}

.collapse-indicator.collapsed {
  transform: rotate(-90deg);
}

/* 已完成数量显示样式 */
.completed-count {
  background: #4CAF50; /* 绿色背景 */
  color: white;
  border-radius: 12px;
  padding: 2px 8px;
  font-size: 0.7rem;
  font-weight: bold;
  min-width: 20px;
  text-align: center;
  margin-left: auto;
}

/* 清除已完成任务按钮 */
.clear-completed-btn {
  background: transparent;
  border: none;
  cursor: pointer;
  font-size: 0.9rem;
  color: #888;
  margin-left: 8px;
  padding: 2px 6px;
  border-radius: 4px;
  transition: all 0.2s ease;
}

.clear-completed-btn:hover {
  background: rgba(244, 67, 54, 0.1);
  color: #f44336;
}

.completed-section {
  margin-top: auto;
  border-top: 1px dashed rgba(229, 231, 235, 0.2);
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
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
  transition: all 0.3s ease;
  border: 1px solid rgba(229, 231, 235, 0.2);
  backdrop-filter: blur(10px);
  min-height: clamp(30px, 5vh, 36px);
  cursor: pointer;
  position: relative;
}
.todo-item:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
  border-color: rgba(229, 231, 235, 0.3);
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
  border-top: 1px solid rgba(229, 231, 235, 0.2);
  background: rgba(255, 255, 255, 0.6);
  gap: clamp(6px, 1.2vw, 10px);
  backdrop-filter: blur(10px);
  min-height: clamp(34px, 6vh, 42px);
  align-items: center;
}
.add-task input {
  flex: 1;
  padding: clamp(6px, 1.5vh, 8px) clamp(8px, 2vw, 10px);
  border: 1px solid rgba(229, 231, 235, 0.3);
  border-radius: clamp(8px, 1.5vw, 12px);
  outline: none;
  background: rgba(255, 255, 255, 0.8);
  color: #333;
  font-size: clamp(0.75rem, 2vw, 0.85rem);
  transition: all 0.3s ease;
  height: clamp(28px, 4.5vh, 32px);
}
.add-task input:focus {
  border-color: #007aff;
  box-shadow: 0 0 8px rgba(0, 122, 255, 0.3);
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
  background: rgba(255, 255, 255, 0.95);
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.15);
  transform: translateY(-2px);
}

.add-task button:active {
  transform: translateY(0);
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
  border: 1px solid rgba(229, 231, 235, 0.3);
  border-radius: 8px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
  backdrop-filter: blur(10px);
  z-index: 1000;
  min-width: 200px;
}

/* 夜间主题样式 */
body.dark-theme header {
  background: rgba(42, 49, 53, 0.8);
  border-bottom: 1px solid rgba(229, 231, 235, 0.2);
  color: #e5e7eb;
}

body.dark-theme .progress-indicator {
  background: rgba(42, 49, 53, 0.9);
  color: #e5e7eb;
  border: 1px solid rgba(229, 231, 235, 0.2);
}

body.dark-theme .settings-btn {
  background: rgba(42, 49, 53, 0.9);
  color: #e5e7eb;
  border: 1px solid rgba(229, 231, 235, 0.2);
}

body.dark-theme .todo-item {
  background: #3d4549;
  border: 1px solid rgba(229, 231, 235, 0.2);
  color: #e5e7eb;
  border-radius: 10px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);
}

body.dark-theme .todo-item:hover {
  border-color: rgba(229, 231, 235, 0.3);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
}

body.dark-theme .todo-checkbox {
  background: rgba(42, 49, 53, 0.8);
  border: 1px solid rgba(229, 231, 235, 0.2);
}

body.dark-theme .todo-checkbox.completed {
  background: rgba(42, 49, 53, 0.9);
  border-color: rgba(76, 175, 80, 0.8);
}

body.dark-theme .todo-checkbox.completed::after {
  color: #e5e7eb;
}

body.dark-theme .todo-item span {
  color: #e5e7eb;
}

body.dark-theme .todo-item.completed span {
  color: #a0a6aa;
}

body.dark-theme .add-task {
  background: rgba(42, 49, 53, 0.8);
  border-top: 1px solid rgba(229, 231, 235, 0.2);
}

body.dark-theme .add-task input {
  background: #3d4549;
  border: 1px solid rgba(229, 231, 235, 0.3);
  color: #e5e7eb;
  border-radius: 10px;
}

body.dark-theme .add-task input::placeholder {
  color: rgba(229, 231, 235, 0.5);
}

body.dark-theme .add-task button {
  background: #3d4549;
  color: #e5e7eb;
  border: 1px solid rgba(229, 231, 235, 0.2);
  border-radius: 50%;
}

body.dark-theme .section-title {
  color: #a0a6aa;
}

body.dark-theme .completed-section {
  border-top: 1px dashed rgba(229, 231, 235, 0.3);
}

body.dark-theme .completed-count {
  background: #4CAF50;
  color: white;
}

body.dark-theme .clear-completed-btn {
  color: #a0a6aa;
}

body.dark-theme .clear-completed-btn:hover {
  background: rgba(244, 67, 54, 0.2);
  color: #f44336;
}

body.dark-theme .context-menu {
  background: rgba(42, 49, 53, 0.95);
  border: 1px solid rgba(229, 231, 235, 0.3);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
}

body.dark-theme .todo-container::-webkit-scrollbar-track {
  background: rgba(229, 231, 235, 0.1);
}

body.dark-theme .todo-container::-webkit-scrollbar-thumb {
  background: #3d4549;
  border: 1px solid rgba(229, 231, 235, 0.2);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.2);
}
</style>