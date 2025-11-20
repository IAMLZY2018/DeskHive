<template>
  <div class="container">
    <header :data-tauri-drag-region="!isDragDisabled ? '' : null">
      <div class="header-title" :data-tauri-drag-region="!isDragDisabled ? '' : null">
        <img src="/icons/app-icon.png" alt="DeskHive" class="app-icon">
        DeskHive
      </div>
      <div class="header-right">
        <div class="progress-indicator">{{ completedTasksCount }}/{{ totalTasksCount }}</div>
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

      <!-- 分组列表 -->
      <div v-if="!showEmptyState && !showAllCompletedState" class="groups-container">
        <TodoGroupComponent
          v-for="group in sortedGroups"
          :key="group.id"
          :group="group"
          :todos="getGroupTodos(group.id, false)"
          :is-active="activeGroupId === group.id"
          @select-group="selectGroup(group.id)"
          @toggle-collapse="toggleGroupCollapse(group.id)"
          @show-menu="(event) => showGroupContextMenu(event, group)"
          @toggle-todo="(index) => toggleTodo(group.id, index)"
          @delete-todo="(index) => deleteTodo(group.id, index)"
          @todo-contextmenu="showTodoContextMenu"
        />
        
        <!-- 已完成任务分组 -->
        <div v-if="allCompletedTodos.length > 0" class="completed-group">
          <div class="group-header" @click="toggleCompletedSection">
            <span class="collapse-indicator" :class="{ collapsed: isCompletedCollapsed }">▼</span>
            <span class="group-icon">✅</span>
            <span class="group-name">已完成</span>
            <span class="group-count">{{ completedTasksCount }}</span>
            <button class="clear-completed-btn" @click.stop="clearAllCompletedTodos" title="清除所有已完成任务">
              🗑️
            </button>
          </div>
          <div v-show="!isCompletedCollapsed" class="group-content">
            <TodoList
              :todos="allCompletedTodos"
              :is-completed-list="true"
              @toggle="(index) => toggleCompletedTodo(index)"
              @delete="(index) => deleteCompletedTodo(index)"
              @contextmenu="showTodoContextMenu"
            />
          </div>
        </div>
      </div>
    </div>
    
    <!-- 底部添加任务区域 -->
    <AddTaskMenu
      @add-task="addTask"
      @add-group="showAddGroupDialog"
    />
    
    <!-- 任务右键菜单 -->
    <ContextMenu
      :show="showContextMenu"
      :position="contextMenuPosition"
      :todo="contextMenuTodo"
      @set-deadline="openDeadlineDialog"
      @remove-deadline="removeDeadline"
      @delete-todo="deleteTodoFromContextMenu"
      @edit-todo="openEditDialog"
    />
    
    <!-- 分组右键菜单 -->
    <GroupContextMenu
      :show="showGroupMenu"
      :position="groupMenuPosition"
      :group="contextMenuGroup"
      @rename="showRenameGroupDialog"
      @toggle-collapse="toggleContextGroupCollapse"
      @delete="deleteGroup"
    />
    
    <!-- 分组名称对话框 -->
    <GroupNameDialog
      :show="showGroupDialog"
      :initial-name="groupDialogName"
      :is-edit="isEditingGroup"
      @confirm="handleGroupDialogConfirm"
      @cancel="closeGroupDialog"
    />
    
    <!-- 截止时间设置对话框 -->
    <DeadlineDialog
      :show="showDeadlineDialog"
      :initial-date="deadlineDate"
      :initial-time="deadlineTime"
      @close="closeDeadlineDialog"
      @confirm="handleDeadlineConfirm"
    />
    
    <!-- 编辑任务对话框 -->
    <EditTaskDialog
      :show="showEditDialog"
      :todo="editDialogTodo"
      @confirm="handleEditConfirm"
      @cancel="closeEditDialog"
    />
    
    <!-- Toast 提示 -->
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
import type { Todo, TodoGroup, DateInfo } from './types';
import EmptyState from './components/EmptyState.vue';
import AllCompletedState from './components/AllCompletedState.vue';
import TodoGroupComponent from './components/TodoGroup.vue';
import TodoList from './components/TodoList.vue';
import AddTaskMenu from './components/AddTaskMenu.vue';
import ContextMenu from './components/ContextMenu.vue';
import GroupContextMenu from './components/GroupContextMenu.vue';
import GroupNameDialog from './components/GroupNameDialog.vue';
import DeadlineDialog from './components/DeadlineDialog.vue';
import EditTaskDialog from './components/EditTaskDialog.vue';
import Toast from './components/Toast.vue';

// 数据状态
const todos = ref<Todo[]>([]);
const groups = ref<TodoGroup[]>([]);
const dateInfo = ref<DateInfo | null>(null);
const activeGroupId = ref<string>('default');
const isCompletedCollapsed = ref(true);
const isDragDisabled = ref(false);

// 右键菜单状态
const showContextMenu = ref(false);
const contextMenuPosition = ref({ x: 0, y: 0 });
const contextMenuTodo = ref<Todo | null>(null);

const showGroupMenu = ref(false);
const groupMenuPosition = ref({ x: 0, y: 0 });
const contextMenuGroup = ref<TodoGroup | null>(null);

// 对话框状态
const showGroupDialog = ref(false);
const groupDialogName = ref('');
const isEditingGroup = ref(false);
const editingGroupId = ref<string | null>(null);

const showDeadlineDialog = ref(false);
const deadlineDate = ref('');
const deadlineTime = ref('');
const dialogTodo = ref<Todo | null>(null);

const showEditDialog = ref(false);
const editDialogTodo = ref<Todo | null>(null);

// Toast 状态
const showToast = ref(false);
const toastMessage = ref('');
const toastType = ref<'error' | 'success' | 'warning'>('error');

// 定时器
const countdownTimer = ref<number | null>(null);

// 计算属性
const sortedGroups = computed(() => {
  return [...groups.value].sort((a, b) => a.order - b.order);
});

const allCompletedTodos = computed(() => {
  return todos.value.filter(t => t.completed).sort((a, b) => a.order - b.order);
});

const totalTasksCount = computed(() => todos.value.length);
const completedTasksCount = computed(() => todos.value.filter(t => t.completed).length);

const showEmptyState = computed(() => todos.value.length === 0);
const showAllCompletedState = computed(() => {
  return todos.value.length > 0 && todos.value.every(t => t.completed);
});

// 获取分组的任务
function getGroupTodos(groupId: string, completed: boolean) {
  return todos.value
    .filter(t => t.groupId === groupId && t.completed === completed)
    .sort((a, b) => {
      // 优先按截止时间排序
      if (a.deadline && b.deadline) {
        return a.deadline - b.deadline;
      }
      if (a.deadline) return -1;
      if (b.deadline) return 1;
      // 其次按 order 排序
      return a.order - b.order;
    });
}

// 生成唯一ID
function generateUniqueId(): string {
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
    const r = Math.random() * 16 | 0;
    const v = c == 'x' ? r : (r & 0x3 | 0x8);
    return v.toString(16);
  });
}

// 初始化默认分组
function initializeDefaultGroup() {
  if (groups.value.length === 0) {
    groups.value.push({
      id: 'default',
      name: '未分组',
      order: 0,
      collapsed: false
    });
  }
}

// 选择分组
function selectGroup(groupId: string) {
  activeGroupId.value = groupId;
}

// 切换分组折叠状态
function toggleGroupCollapse(groupId: string) {
  const group = groups.value.find(g => g.id === groupId);
  if (group) {
    group.collapsed = !group.collapsed;
    saveGroupData();
  }
}

// 切换已完成部分折叠状态
function toggleCompletedSection() {
  isCompletedCollapsed.value = !isCompletedCollapsed.value;
}

// 添加任务
function addTask(text: string) {
  const now = Math.floor(Date.now() / 1000);
  const maxOrder = Math.max(0, ...todos.value.filter(t => t.groupId === activeGroupId.value).map(t => t.order));
  
  todos.value.push({
    id: generateUniqueId(),
    text: text,
    completed: false,
    createdAt: now,
    order: maxOrder + 1,
    groupId: activeGroupId.value
  });
  
  saveTodoData();
}

// 切换任务完成状态
function toggleTodo(groupId: string, index: number) {
  const groupTodos = getGroupTodos(groupId, false);
  const todo = groupTodos[index];
  const todoIndex = todos.value.findIndex(t => t.id === todo.id);
  
  if (todoIndex !== -1) {
    todos.value[todoIndex].completed = true;
    saveTodoData();
  }
}

// 切换已完成任务状态
function toggleCompletedTodo(index: number) {
  const todo = allCompletedTodos.value[index];
  const todoIndex = todos.value.findIndex(t => t.id === todo.id);
  
  if (todoIndex !== -1) {
    todos.value[todoIndex].completed = false;
    saveTodoData();
  }
}

// 删除任务
function deleteTodo(groupId: string, index: number) {
  const groupTodos = getGroupTodos(groupId, false);
  const todo = groupTodos[index];
  const todoIndex = todos.value.findIndex(t => t.id === todo.id);
  
  if (todoIndex !== -1) {
    todos.value.splice(todoIndex, 1);
    saveTodoData();
  }
}

// 删除已完成任务
function deleteCompletedTodo(index: number) {
  const todo = allCompletedTodos.value[index];
  const todoIndex = todos.value.findIndex(t => t.id === todo.id);
  
  if (todoIndex !== -1) {
    todos.value.splice(todoIndex, 1);
    saveTodoData();
  }
}

// 清除所有已完成任务
function clearAllCompletedTodos() {
  todos.value = todos.value.filter(t => !t.completed);
  saveTodoData();
  showToastMessage('已清除所有已完成任务', 'success');
}

// 显示任务右键菜单
function showTodoContextMenu(event: MouseEvent, todo: Todo) {
  event.preventDefault();
  contextMenuTodo.value = todo;
  contextMenuPosition.value = { x: event.clientX, y: event.clientY };
  showContextMenu.value = true;
  document.addEventListener('click', hideContextMenu);
}

// 隐藏任务右键菜单
function hideContextMenu() {
  showContextMenu.value = false;
  contextMenuTodo.value = null;
  document.removeEventListener('click', hideContextMenu);
}

// 从右键菜单删除任务
function deleteTodoFromContextMenu() {
  if (!contextMenuTodo.value) return;
  
  const todoIndex = todos.value.findIndex(t => t.id === contextMenuTodo.value!.id);
  if (todoIndex !== -1) {
    todos.value.splice(todoIndex, 1);
    saveTodoData();
    showToastMessage('任务已删除', 'success');
  }
  
  hideContextMenu();
}

// 显示分组右键菜单
function showGroupContextMenu(event: MouseEvent, group: TodoGroup) {
  event.preventDefault();
  contextMenuGroup.value = group;
  groupMenuPosition.value = { x: event.clientX, y: event.clientY };
  showGroupMenu.value = true;
  document.addEventListener('click', hideGroupMenu);
}

// 隐藏分组右键菜单
function hideGroupMenu() {
  showGroupMenu.value = false;
  contextMenuGroup.value = null;
  document.removeEventListener('click', hideGroupMenu);
}

// 切换右键菜单中的分组折叠状态
function toggleContextGroupCollapse() {
  if (contextMenuGroup.value) {
    toggleGroupCollapse(contextMenuGroup.value.id);
    hideGroupMenu();
  }
}

// 显示添加分组对话框
function showAddGroupDialog() {
  groupDialogName.value = '';
  isEditingGroup.value = false;
  editingGroupId.value = null;
  showGroupDialog.value = true;
}

// 显示重命名分组对话框
function showRenameGroupDialog() {
  if (!contextMenuGroup.value) return;
  
  groupDialogName.value = contextMenuGroup.value.name;
  isEditingGroup.value = true;
  editingGroupId.value = contextMenuGroup.value.id;
  showGroupDialog.value = true;
  hideGroupMenu();
}

// 关闭分组对话框
function closeGroupDialog() {
  showGroupDialog.value = false;
  groupDialogName.value = '';
  isEditingGroup.value = false;
  editingGroupId.value = null;
}

// 处理分组对话框确认
function handleGroupDialogConfirm(name: string) {
  if (isEditingGroup.value && editingGroupId.value) {
    // 重命名分组
    const group = groups.value.find(g => g.id === editingGroupId.value);
    if (group) {
      group.name = name;
      saveGroupData();
      showToastMessage('分组已重命名', 'success');
    }
  } else {
    // 新建分组
    const maxOrder = Math.max(0, ...groups.value.map(g => g.order));
    groups.value.push({
      id: generateUniqueId(),
      name: name,
      order: maxOrder + 1,
      collapsed: false
    });
    saveGroupData();
    showToastMessage('分组已创建', 'success');
  }
  
  closeGroupDialog();
}

// 删除分组
function deleteGroup() {
  if (!contextMenuGroup.value || contextMenuGroup.value.id === 'default') {
    hideGroupMenu();
    return;
  }
  
  const groupId = contextMenuGroup.value.id;
  
  // 将分组中的任务移动到默认分组
  todos.value.forEach(todo => {
    if (todo.groupId === groupId) {
      todo.groupId = 'default';
    }
  });
  
  // 删除分组
  const groupIndex = groups.value.findIndex(g => g.id === groupId);
  if (groupIndex !== -1) {
    groups.value.splice(groupIndex, 1);
    saveGroupData();
    saveTodoData();
    showToastMessage('分组已删除', 'success');
  }
  
  hideGroupMenu();
}

// 打开编辑任务对话框
function openEditDialog() {
  if (!contextMenuTodo.value) return;
  
  editDialogTodo.value = contextMenuTodo.value;
  hideContextMenu();
  showEditDialog.value = true;
}

// 关闭编辑任务对话框
function closeEditDialog() {
  showEditDialog.value = false;
  editDialogTodo.value = null;
}

// 处理编辑确认
async function handleEditConfirm(newText: string) {
  if (!editDialogTodo.value || !newText.trim()) {
    closeEditDialog();
    return;
  }
  
  const todoIndex = todos.value.findIndex(t => t.id === editDialogTodo.value!.id);
  if (todoIndex !== -1) {
    todos.value[todoIndex].text = newText.trim();
    saveTodoData();
  }
  
  closeEditDialog();
}

// 打开截止时间设置对话框
function openDeadlineDialog() {
  if (!contextMenuTodo.value) return;
  
  dialogTodo.value = contextMenuTodo.value;
  
  if (contextMenuTodo.value.deadline) {
    const deadlineDateTime = new Date(contextMenuTodo.value.deadline * 1000);
    deadlineDate.value = deadlineDateTime.toISOString().split('T')[0];
    deadlineTime.value = deadlineDateTime.toTimeString().slice(0, 5);
  } else {
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

// 处理截止时间确认
function handleDeadlineConfirm(date: string, time: string) {
  deadlineDate.value = date;
  deadlineTime.value = time;
  setDeadline();
}

// 设置截止时间
function setDeadline() {
  if (!dialogTodo.value || !deadlineDate.value || !deadlineTime.value) {
    showToastMessage('请选择日期和时间', 'warning');
    return;
  }
  
  const deadlineDateTime = new Date(`${deadlineDate.value}T${deadlineTime.value}`);
  const deadlineTimestamp = Math.floor(deadlineDateTime.getTime() / 1000);
  
  const now = Math.floor(Date.now() / 1000);
  if (deadlineTimestamp <= now - 60) {
    showToastMessage('截止时间必须在未来', 'error');
    return;
  }
  
  const todoIndex = todos.value.findIndex(t => t.id === dialogTodo.value!.id);
  if (todoIndex !== -1) {
    todos.value[todoIndex].deadline = deadlineTimestamp;
    saveTodoData();
    showToastMessage('截止时间设置成功', 'success');
  }
  
  closeDeadlineDialog();
}

// 移除截止时间
function removeDeadline() {
  if (!contextMenuTodo.value) return;
  
  const todoIndex = todos.value.findIndex(t => t.id === contextMenuTodo.value!.id);
  if (todoIndex !== -1) {
    todos.value[todoIndex].deadline = undefined;
    saveTodoData();
    showToastMessage('截止时间移除成功', 'success');
  }
  
  hideContextMenu();
}

// 显示 Toast 提示
function showToastMessage(message: string, type: 'error' | 'success' | 'warning' = 'error') {
  toastMessage.value = message;
  toastType.value = type;
  showToast.value = true;
  
  setTimeout(() => {
    showToast.value = false;
  }, 3000);
}

// 保存任务数据
async function saveTodoData() {
  try {
    const todosForBackend = todos.value.map(todo => ({
      id: todo.id,
      text: todo.text,
      completed: todo.completed,
      created_at: todo.createdAt,
      deadline: todo.deadline || null,
      order: todo.order,
      group_id: todo.groupId
    }));
    
    await invoke('save_todo_data_with_groups', {
      todos: todosForBackend
    });
    console.log('任务数据保存成功');
  } catch (error) {
    console.error('保存任务数据失败:', error);
  }
}

// 保存分组数据
async function saveGroupData() {
  try {
    const groupsForBackend = groups.value.map(group => ({
      id: group.id,
      name: group.name,
      order: group.order,
      collapsed: group.collapsed
    }));
    
    await invoke('save_group_data', {
      groups: groupsForBackend
    });
    console.log('分组数据保存成功');
  } catch (error) {
    console.error('保存分组数据失败:', error);
  }
}

// 加载任务数据
async function loadTodoData() {
  try {
    const data = await invoke('load_todo_data_with_groups') as {
      todos: { id: string; text: string; completed: boolean; created_at: number; deadline?: number; order: number; group_id: string }[]
    };
    
    todos.value = data.todos.map(todo => ({
      id: todo.id,
      text: todo.text,
      completed: todo.completed,
      createdAt: todo.created_at,
      deadline: todo.deadline,
      order: todo.order,
      groupId: todo.group_id
    }));
    
    console.log('任务数据加载成功');
  } catch (error) {
    console.error('加载任务数据失败:', error);
    todos.value = [];
  }
}

// 加载分组数据
async function loadGroupData() {
  try {
    const data = await invoke('load_group_data') as {
      groups: { id: string; name: string; order: number; collapsed: boolean }[]
    };
    
    groups.value = data.groups;
    console.log('分组数据加载成功');
  } catch (error) {
    console.error('加载分组数据失败:', error);
    groups.value = [];
  }
  
  initializeDefaultGroup();
}

// 加载日期信息
async function loadDateInfo() {
  try {
    const data = await invoke('get_current_date') as DateInfo;
    dateInfo.value = data;
  } catch (error) {
    console.error('加载日期信息失败:', error);
  }
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
    document.body.className = settings.theme === 'dark' ? 'dark-theme' : '';
  } catch (error) {
    console.error('加载应用设置失败:', error);
  }
}

// 打开设置窗口
async function openSettings() {
  try {
    await invoke('open_settings_window');
  } catch (error) {
    console.error('打开设置窗口失败:', error);
  }
}

// 监听主题变化
async function listenThemeChange() {
  const currentWindow = getCurrentWindow();
  await currentWindow.listen('theme-changed', (event) => {
    const theme = event.payload as string;
    document.body.className = theme === 'dark' ? 'dark-theme' : '';
  });
}

// 启动倒计时更新定时器
function startCountdownTimer() {
  if (countdownTimer.value) {
    clearInterval(countdownTimer.value);
  }
  
  countdownTimer.value = window.setInterval(() => {
    todos.value = [...todos.value];
  }, 60000);
}

// 阻止浏览器默认右键菜单
function preventDefaultContextMenu(event: MouseEvent) {
  const target = event.target as HTMLElement;
  if (!target.closest('.todo-item') && !target.closest('.group-header')) {
    event.preventDefault();
  }
}

// 组件挂载
onMounted(async () => {
  document.addEventListener('contextmenu', preventDefaultContextMenu);
  
  await loadGroupData();
  await loadTodoData();
  await loadAppSettings();
  await loadDateInfo();
  await listenThemeChange();
  
  startCountdownTimer();
  
  const currentWindow = getCurrentWindow();
  await currentWindow.listen('drag-setting-changed', (event) => {
    isDragDisabled.value = event.payload as boolean;
  });
});

// 组件卸载
onUnmounted(() => {
  if (countdownTimer.value) {
    clearInterval(countdownTimer.value);
  }
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

.groups-container {
  padding: clamp(8px, 2vh, 12px);
}

.completed-group {
  margin-top: 10px;
  border-top: 1px dashed rgba(229, 231, 235, 0.2);
  padding-top: 10px;
}

.completed-group .group-header {
  display: flex;
  align-items: center;
  padding: clamp(8px, 2vh, 10px) clamp(10px, 2.5vw, 14px);
  background: rgba(255, 255, 255, 0.5);
  border-radius: clamp(8px, 1.5vw, 12px);
  cursor: pointer;
  transition: all 0.3s ease;
  user-select: none;
  gap: 8px;
  border: 1px solid rgba(229, 231, 235, 0.2);
  backdrop-filter: blur(5px);
}

.completed-group .group-header:hover {
  background: rgba(255, 255, 255, 0.7);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.collapse-indicator {
  font-size: 0.7rem;
  transition: transform 0.3s ease;
  color: #666;
}

.collapse-indicator.collapsed {
  transform: rotate(-90deg);
}

.group-icon {
  font-size: 1rem;
}

.group-name {
  flex: 1;
  font-size: clamp(0.85rem, 2.2vw, 0.95rem);
  font-weight: 600;
  color: #333;
}

.group-count {
  background: #4CAF50;
  color: white;
  border-radius: 12px;
  padding: 2px 8px;
  font-size: 0.7rem;
  font-weight: bold;
  min-width: 20px;
  text-align: center;
}

.clear-completed-btn {
  background: transparent;
  border: none;
  cursor: pointer;
  font-size: 0.9rem;
  color: #888;
  padding: 2px 6px;
  border-radius: 4px;
  transition: all 0.2s ease;
}

.clear-completed-btn:hover {
  background: rgba(244, 67, 54, 0.1);
  color: #f44336;
}

.group-content {
  padding: clamp(8px, 2vh, 12px) clamp(4px, 1vw, 8px);
}

/* 夜间主题 */
body.dark-theme header {
  background: rgba(24, 26, 27, 0.8);
  border-bottom: 1px solid rgba(231, 233, 237, 0.2);
  color: #e7e9ed;
}

body.dark-theme .progress-indicator {
  background: rgba(24, 26, 27, 0.9);
  color: #e7e9ed;
  border: 1px solid rgba(231, 233, 237, 0.2);
}

body.dark-theme .settings-btn {
  background: rgba(24, 26, 27, 0.9);
  color: #e7e9ed;
  border: 1px solid rgba(231, 233, 237, 0.2);
}

body.dark-theme .completed-group .group-header {
  background: rgba(37, 38, 39, 0.5);
  border-color: rgba(231, 233, 237, 0.2);
}

body.dark-theme .completed-group .group-header:hover {
  background: rgba(37, 38, 39, 0.7);
}

body.dark-theme .group-name {
  color: #e7e9ed;
}

body.dark-theme .collapse-indicator {
  color: #aaa;
}

body.dark-theme .clear-completed-btn {
  color: #aaa;
}

body.dark-theme .clear-completed-btn:hover {
  background: rgba(244, 67, 54, 0.2);
  color: #f44336;
}

body.dark-theme .todo-container::-webkit-scrollbar-track {
  background: rgba(231, 233, 237, 0.1);
}

body.dark-theme .todo-container::-webkit-scrollbar-thumb {
  background: #252627;
  border: 1px solid rgba(231, 233, 237, 0.2);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.2);
}
</style>
