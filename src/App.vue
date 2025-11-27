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
        <!-- 未分组的任务 - 直接显示 -->
        <div v-if="getGroupTodos('default', false).length > 0">
          <TodoList
            :todos="getGroupTodos('default', false)"
            :show-border="false"
            @toggle="(index) => toggleTodo('default', index)"
            @delete="(index) => deleteTodo('default', index)"
            @contextmenu="showTodoContextMenu"
            @reorder="(newOrder) => handleTodoReorder('default', newOrder)"
            @drag-start="(todo) => handleDragStart(todo)"
            @drag-end="handleDragEnd"
            @change="(event) => handleTodoChange('default', event)"
          />
        </div>
        
        <!-- 其他分组 -->
        <TransitionGroup name="group-list" tag="div" class="active-groups">
          <TodoGroupComponent
            v-for="group in sortedGroupsWithoutDefault"
            :key="group.id"
            :group="group"
            :todos="getGroupTodos(group.id, false)"
            @toggleCollapse="toggleGroupCollapse(group.id)"
            @showMenu="(event) => showGroupContextMenu(event, group)"
            @toggle-todo="(index: number) => toggleTodo(group.id, index)"
            @delete-todo="(index: number) => deleteTodo(group.id, index)"
            @todo-contextmenu="showTodoContextMenu"
            @reorder="(newOrder) => handleTodoReorder(group.id, newOrder)"
            @drag-start="(todo) => handleDragStart(todo)"
            @drag-end="handleDragEnd"
            @move-up="moveGroupUp(group.id)"
            @move-down="moveGroupDown(group.id)"
            @change="(event) => handleTodoChange(group.id, event)"
            @drop-on-header="handleDropOnGroupHeader(group.id)"
          />
        </TransitionGroup>
        
        <!-- 已完成任务分组 - 固定在底部 -->
        <div v-if="allCompletedTodos.length > 0" class="completed-group-wrapper">
          <div class="completed-group">
            <div class="group-header" @click="toggleCompletedSection">
              <span class="collapse-indicator" :class="{ collapsed: isCompletedCollapsed }">▼</span>
              <svg class="group-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <circle cx="12" cy="12" r="10" fill="#4CAF50"/>
                <path d="M7 12L10.5 15.5L17 9" stroke="white" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
              <span class="group-name">已完成</span>
              <span class="group-count">{{ completedTasksCount }}</span>
              <button class="clear-completed-btn" @click.stop="clearAllCompletedTodos" title="清除所有已完成任务">
                <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M19 7L18.1327 19.1425C18.0579 20.1891 17.187 21 16.1378 21H7.86224C6.81296 21 5.94208 20.1891 5.86732 19.1425L5 7M10 11V17M14 11V17M15 7V4C15 3.44772 14.5523 3 14 3H10C9.44772 3 9 3.44772 9 4V7M4 7H20" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
              </button>
            </div>
            <div v-show="!isCompletedCollapsed" class="group-content">
              <TodoList
                :todos="allCompletedTodos"
                :is-completed-list="true"
                :show-border="true"
                @toggle="(index) => toggleCompletedTodo(index)"
                @delete="(index) => deleteCompletedTodo(index)"
                @contextmenu="showTodoContextMenu"
              />
            </div>
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

// 拖动状态
const draggedTodo = ref<Todo | null>(null);
const dragSourceGroupId = ref<string | null>(null);

// 计算属性
const sortedGroups = computed(() => {
  return [...groups.value].sort((a, b) => a.order - b.order);
});

const sortedGroupsWithoutDefault = computed(() => {
  return sortedGroups.value.filter(g => g.id !== 'default');
});

const allCompletedTodos = computed(() => {
  return todos.value.filter(t => t.completed).sort((a, b) => a.order - b.order);
});

const totalTasksCount = computed(() => todos.value.length);
const completedTasksCount = computed(() => todos.value.filter(t => t.completed).length);

const showEmptyState = computed(() => todos.value.length === 0);
const showAllCompletedState = computed(() => {
  // 只有当所有任务都完成，且所有分组（除了default）都被删除时才显示
  const hasActiveTodos = todos.value.some(t => !t.completed);
  const hasNonDefaultGroups = sortedGroupsWithoutDefault.value.length > 0;
  
  return todos.value.length > 0 && !hasActiveTodos && !hasNonDefaultGroups;
});

// 获取分组的任务
function getGroupTodos(groupId: string, completed: boolean) {
  return todos.value
    .filter(t => t.groupId === groupId && t.completed === completed)
    .sort((a, b) => a.order - b.order);
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

// 选择分组函数已移除，不再需要

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
  // 检查是否是创建分组的命令（以 / 开头）
  if (text.startsWith('/')) {
    const groupName = text.slice(1).trim();
    if (groupName) {
      // 直接创建分组，不弹窗
      const maxOrder = Math.max(0, ...groups.value.map(g => g.order));
      groups.value.push({
        id: generateUniqueId(),
        name: groupName,
        order: maxOrder + 1,
        collapsed: false
      });
      saveGroupData();
      showToastMessage('分组已创建', 'success');
    }
    return;
  }
  
  const now = Math.floor(Date.now() / 1000);
  // 新任务始终添加到未分组（default）
  const maxOrder = Math.max(0, ...todos.value.filter(t => t.groupId === 'default').map(t => t.order));
  
  todos.value.push({
    id: generateUniqueId(),
    text: text,
    completed: false,
    createdAt: now,
    order: maxOrder + 1,
    groupId: 'default'
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

// 上移分组
function moveGroupUp(groupId: string) {
  console.log('moveGroupUp called:', groupId);
  const index = sortedGroups.value.findIndex(g => g.id === groupId);
  console.log('Current index:', index, 'Total groups:', sortedGroups.value.length);
  
  if (index <= 0) {
    console.log('Already at top');
    return; // 已经在最上面
  }
  
  // 交换 order
  const currentGroup = sortedGroups.value[index];
  const prevGroup = sortedGroups.value[index - 1];
  
  console.log('Swapping:', currentGroup.name, 'with', prevGroup.name);
  
  const tempOrder = currentGroup.order;
  currentGroup.order = prevGroup.order;
  prevGroup.order = tempOrder;
  
  console.log('New order:', sortedGroups.value.map(g => `${g.name}(${g.order})`));
  
  saveGroupData();
}

// 下移分组
function moveGroupDown(groupId: string) {
  console.log('moveGroupDown called:', groupId);
  const index = sortedGroups.value.findIndex(g => g.id === groupId);
  console.log('Current index:', index, 'Total groups:', sortedGroups.value.length);
  
  if (index < 0 || index >= sortedGroups.value.length - 1) {
    console.log('Already at bottom');
    return; // 已经在最下面
  }
  
  // 交换 order
  const currentGroup = sortedGroups.value[index];
  const nextGroup = sortedGroups.value[index + 1];
  
  console.log('Swapping:', currentGroup.name, 'with', nextGroup.name);
  
  const tempOrder = currentGroup.order;
  currentGroup.order = nextGroup.order;
  nextGroup.order = tempOrder;
  
  console.log('New order:', sortedGroups.value.map(g => `${g.name}(${g.order})`));
  
  saveGroupData();
}

// 处理拖动开始
function handleDragStart(todo: Todo) {
  console.log('拖动开始:', todo.text, '来自分组:', todo.groupId);
  draggedTodo.value = todo;
  dragSourceGroupId.value = todo.groupId;
}

// 处理拖动结束
function handleDragEnd() {
  console.log('拖动结束');
  draggedTodo.value = null;
  dragSourceGroupId.value = null;
}

// 处理拖放到分组标题
function handleDropOnGroupHeader(targetGroupId: string) {
  console.log('拖放到分组标题:', targetGroupId);
  
  if (!draggedTodo.value || !dragSourceGroupId.value) {
    console.log('没有拖动的任务');
    return;
  }
  
  // 如果拖到同一个分组，不做处理
  if (dragSourceGroupId.value === targetGroupId) {
    console.log('拖到同一个分组，忽略');
    return;
  }
  
  const todoIndex = todos.value.findIndex(t => t.id === draggedTodo.value!.id);
  if (todoIndex === -1) {
    console.log('找不到任务');
    return;
  }
  
  // 获取目标分组的最大 order
  const targetGroupTodos = getGroupTodos(targetGroupId, false);
  const maxOrder = targetGroupTodos.length > 0 
    ? Math.max(...targetGroupTodos.map(t => t.order))
    : -1;
  
  // 更新任务的分组和顺序（放到末尾）
  todos.value[todoIndex].groupId = targetGroupId;
  todos.value[todoIndex].order = maxOrder + 1;
  
  console.log(`任务 "${draggedTodo.value.text}" 从分组 "${dragSourceGroupId.value}" 移动到分组 "${targetGroupId}" 的末尾`);
  
  // 重新计算源分组的 order
  const sourceGroupTodos = getGroupTodos(dragSourceGroupId.value, false);
  sourceGroupTodos.forEach((t, index) => {
    const idx = todos.value.findIndex(item => item.id === t.id);
    if (idx !== -1) {
      todos.value[idx].order = index;
    }
  });
  
  // 保存到后端
  saveTodoData();
  
  // 清除拖动状态
  draggedTodo.value = null;
  dragSourceGroupId.value = null;
}

// 处理任务重新排序
function handleTodoReorder(groupId: string, newOrder: Todo[]) {
  console.log('任务重新排序:', groupId, newOrder.map(t => t.text));
  
  // 更新任务的 order 字段和 groupId
  newOrder.forEach((todo, index) => {
    const todoIndex = todos.value.findIndex(t => t.id === todo.id);
    if (todoIndex !== -1) {
      todos.value[todoIndex].order = index;
      todos.value[todoIndex].groupId = groupId;
    }
  });
  
  // 保存到后端
  saveTodoData();
}

// 处理任务跨分组拖拽
function handleTodoChange(groupId: string, event: any) {
  console.log('任务拖拽变化:', groupId, event);
  
  // 当任务被添加到这个分组时
  if (event.added) {
    const todo = event.added.element;
    // const newIndex = event.added.newIndex; // 暂时不需要使用
    const todoIndex = todos.value.findIndex(t => t.id === todo.id);
    
    if (todoIndex !== -1) {
      // 更新任务的分组ID
      todos.value[todoIndex].groupId = groupId;
      console.log(`任务 "${todo.text}" 从分组 "${dragSourceGroupId.value}" 移动到分组 "${groupId}"`);
      
      // 重新计算目标分组所有任务的 order
      const targetGroupTodos = getGroupTodos(groupId, false);
      targetGroupTodos.forEach((t, index) => {
        const idx = todos.value.findIndex(item => item.id === t.id);
        if (idx !== -1) {
          todos.value[idx].order = index;
        }
      });
      
      // 如果有源分组，重新计算源分组的 order
      if (dragSourceGroupId.value && dragSourceGroupId.value !== groupId) {
        const sourceGroupTodos = getGroupTodos(dragSourceGroupId.value, false);
        sourceGroupTodos.forEach((t, index) => {
          const idx = todos.value.findIndex(item => item.id === t.id);
          if (idx !== -1) {
            todos.value[idx].order = index;
          }
        });
      }
      
      // 保存到后端
      saveTodoData();
    }
  }
  
  // 当任务在同一分组内移动时
  if (event.moved) {
    console.log('任务在分组内移动:', groupId, event.moved);
    // 不需要在这里处理，reorder 事件会处理
  }
  
  // 当任务从这个分组移除时
  if (event.removed) {
    console.log('任务从分组移除:', groupId);
    // 重新计算该分组剩余任务的 order
    const groupTodos = getGroupTodos(groupId, false);
    groupTodos.forEach((todo, index) => {
      const todoIndex = todos.value.findIndex(t => t.id === todo.id);
      if (todoIndex !== -1) {
        todos.value[todoIndex].order = index;
      }
    });
    
    // 保存到后端
    saveTodoData();
  }
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
    
    todos.value = data.todos.map((todo, index) => ({
      id: todo.id,
      text: todo.text,
      completed: todo.completed,
      createdAt: todo.created_at,
      deadline: todo.deadline,
      order: todo.order ?? index, // 如果没有order，使用索引
      groupId: todo.group_id || 'default' // 如果没有groupId，使用default
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
  display: none;
}

.todo-container {
  -ms-overflow-style: none;  /* IE and Edge */
  scrollbar-width: none;  /* Firefox */
}

.groups-container {
  padding: clamp(8px, 2vh, 12px);
  display: flex;
  flex-direction: column;
  min-height: 100%;
}



.active-groups {
  flex: 0 0 auto;
  margin-bottom: clamp(8px, 2vh, 12px);
  position: relative;
}

/* 分组列表动画 */
.group-list-move {
  transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}

.group-list-enter-active,
.group-list-leave-active {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.group-list-enter-from {
  opacity: 0;
  transform: translateY(-20px);
}

.group-list-leave-to {
  opacity: 0;
  transform: translateY(20px);
}

.group-list-leave-active {
  position: absolute;
  width: 100%;
}

.completed-group-wrapper {
  margin-top: auto;
  padding-top: 10px;
}

.completed-group {
  border-top: 1px dashed rgba(229, 231, 235, 0.2);
  padding-top: 10px;
}

.completed-group .group-header {
  display: flex;
  align-items: center;
  padding: clamp(4px, 1vh, 6px) clamp(8px, 2vw, 10px);
  background: rgba(255, 255, 255, 0.5);
  border-radius: clamp(8px, 1.5vw, 10px);
  cursor: pointer;
  transition: all 0.3s ease;
  user-select: none;
  gap: clamp(6px, 1.5vw, 8px);
  border: 1px solid rgba(229, 231, 235, 0.2);
  backdrop-filter: blur(5px);
  min-height: clamp(28px, 4vh, 32px);
}

.completed-group .group-header:hover {
  background: rgba(255, 255, 255, 0.7);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.collapse-indicator {
  font-size: clamp(0.6rem, 1.5vw, 0.7rem);
  transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  color: #94a3b8;
  flex-shrink: 0;
}

.collapse-indicator.collapsed {
  transform: rotate(-90deg);
}

.group-icon {
  width: clamp(16px, 3.5vw, 20px);
  height: clamp(16px, 3.5vw, 20px);
  flex-shrink: 0;
}

.group-name {
  flex: 1;
  font-size: clamp(0.75rem, 2vw, 0.85rem);
  font-weight: 600;
  color: #475569;
}

.group-count {
  background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
  color: white;
  border-radius: 10px;
  padding: clamp(1px, 0.3vh, 2px) clamp(6px, 1.5vw, 8px);
  font-size: clamp(0.6rem, 1.5vw, 0.7rem);
  font-weight: bold;
  min-width: clamp(18px, 4vw, 22px);
  text-align: center;
  flex-shrink: 0;
  box-shadow: 0 1px 3px rgba(245, 158, 11, 0.2);
  margin-left: auto;
}

.clear-completed-btn {
  background: transparent;
  border: none;
  cursor: pointer;
  color: #94a3b8;
  padding: clamp(2px, 0.5vh, 4px);
  border-radius: 4px;
  transition: all 0.2s ease;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  width: clamp(20px, 4vw, 24px);
  height: clamp(20px, 4vw, 24px);
}

.clear-completed-btn svg {
  width: clamp(14px, 3vw, 16px);
  height: clamp(14px, 3vw, 16px);
}

.clear-completed-btn:hover {
  background: rgba(244, 67, 54, 0.1);
  color: #f44336;
  transform: scale(1.1);
}

.group-content {
  padding: clamp(4px, 1vh, 6px) clamp(4px, 1vw, 6px);
  animation: slideDown 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  transform-origin: top;
}

@keyframes slideDown {
  from {
    opacity: 0;
    transform: scaleY(0.95);
  }
  to {
    opacity: 1;
    transform: scaleY(1);
  }
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

body.dark-theme .group-icon circle {
  fill: #4CAF50;
}

body.dark-theme .group-icon path {
  stroke: white;
}




</style>
