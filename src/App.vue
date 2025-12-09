<template>
  <div class="container">
    <header :data-tauri-drag-region="!isDragDisabled ? '' : null">
      <div class="header-title" :data-tauri-drag-region="!isDragDisabled ? '' : null">
        <img src="/icons/app-icon.svg" alt="DeskHive" class="app-icon">
        DeskHive
      </div>
      <div class="header-right">
        <div class="progress-indicator">{{ completedTasksCount }}/{{ totalTasksCount }}</div>
        <button class="settings-btn" @click="openSettings">
          <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M12 15C13.6569 15 15 13.6569 15 12C15 10.3431 13.6569 9 12 9C10.3431 9 9 10.3431 9 12C9 13.6569 10.3431 15 12 15Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M19.4 15C19.2669 15.3016 19.2272 15.6362 19.286 15.9606C19.3448 16.285 19.4995 16.5843 19.73 16.82L19.79 16.88C19.976 17.0657 20.1235 17.2863 20.2241 17.5291C20.3248 17.7719 20.3766 18.0322 20.3766 18.295C20.3766 18.5578 20.3248 18.8181 20.2241 19.0609C20.1235 19.3037 19.976 19.5243 19.79 19.71C19.6043 19.896 19.3837 20.0435 19.1409 20.1441C18.8981 20.2448 18.6378 20.2966 18.375 20.2966C18.1122 20.2966 17.8519 20.2448 17.6091 20.1441C17.3663 20.0435 17.1457 19.896 16.96 19.71L16.9 19.65C16.6643 19.4195 16.365 19.2648 16.0406 19.206C15.7162 19.1472 15.3816 19.1869 15.08 19.32C14.7842 19.4468 14.532 19.6572 14.3543 19.9255C14.1766 20.1938 14.0813 20.5082 14.08 20.83V21C14.08 21.5304 13.8693 22.0391 13.4942 22.4142C13.1191 22.7893 12.6104 23 12.08 23C11.5496 23 11.0409 22.7893 10.6658 22.4142C10.2907 22.0391 10.08 21.5304 10.08 21V20.91C10.0723 20.579 9.96512 20.258 9.77251 19.9887C9.5799 19.7194 9.31074 19.5143 9 19.4C8.69838 19.2669 8.36381 19.2272 8.03941 19.286C7.71502 19.3448 7.41568 19.4995 7.18 19.73L7.12 19.79C6.93425 19.976 6.71368 20.1235 6.47088 20.2241C6.22808 20.3248 5.96783 20.3766 5.705 20.3766C5.44217 20.3766 5.18192 20.3248 4.93912 20.2241C4.69632 20.1235 4.47575 19.976 4.29 19.79C4.10405 19.6043 3.95653 19.3837 3.85588 19.1409C3.75523 18.8981 3.70343 18.6378 3.70343 18.375C3.70343 18.1122 3.75523 17.8519 3.85588 17.6091C3.95653 17.3663 4.10405 17.1457 4.29 16.96L4.35 16.9C4.58054 16.6643 4.73519 16.365 4.794 16.0406C4.85282 15.7162 4.81312 15.3816 4.68 15.08C4.55324 14.7842 4.34276 14.532 4.07447 14.3543C3.80618 14.1766 3.49179 14.0813 3.17 14.08H3C2.46957 14.08 1.96086 13.8693 1.58579 13.4942C1.21071 13.1191 1 12.6104 1 12.08C1 11.5496 1.21071 11.0409 1.58579 10.6658C1.96086 10.2907 2.46957 10.08 3 10.08H3.09C3.42099 10.0723 3.742 9.96512 4.0113 9.77251C4.28059 9.5799 4.48572 9.31074 4.6 9C4.73312 8.69838 4.77282 8.36381 4.714 8.03941C4.65519 7.71502 4.50054 7.41568 4.27 7.18L4.21 7.12C4.02405 6.93425 3.87653 6.71368 3.77588 6.47088C3.67523 6.22808 3.62343 5.96783 3.62343 5.705C3.62343 5.44217 3.67523 5.18192 3.77588 4.93912C3.87653 4.69632 4.02405 4.47575 4.21 4.29C4.39575 4.10405 4.61632 3.95653 4.85912 3.85588C5.10192 3.75523 5.36217 3.70343 5.625 3.70343C5.88783 3.70343 6.14808 3.75523 6.39088 3.85588C6.63368 3.95653 6.85425 4.10405 7.04 4.29L7.1 4.35C7.33568 4.58054 7.63502 4.73519 7.95941 4.794C8.28381 4.85282 8.61838 4.81312 8.92 4.68H9C9.29577 4.55324 9.54802 4.34276 9.72569 4.07447C9.90337 3.80618 9.99872 3.49179 10 3.17V3C10 2.46957 10.2107 1.96086 10.5858 1.58579C10.9609 1.21071 11.4696 1 12 1C12.5304 1 13.0391 1.21071 13.4142 1.58579C13.7893 1.96086 14 2.46957 14 3V3.09C14.0013 3.41179 14.0966 3.72618 14.2743 3.99447C14.452 4.26276 14.7042 4.47324 15 4.6C15.3016 4.73312 15.6362 4.77282 15.9606 4.714C16.285 4.65519 16.5843 4.50054 16.82 4.27L16.88 4.21C17.0657 4.02405 17.2863 3.87653 17.5291 3.77588C17.7719 3.67523 18.0322 3.62343 18.295 3.62343C18.5578 3.62343 18.8181 3.67523 19.0609 3.77588C19.3037 3.87653 19.5243 4.02405 19.71 4.21C19.896 4.39575 20.0435 4.61632 20.1441 4.85912C20.2448 5.10192 20.2966 5.36217 20.2966 5.625C20.2966 5.88783 20.2448 6.14808 20.1441 6.39088C20.0435 6.63368 19.896 6.85425 19.71 7.04L19.65 7.1C19.4195 7.33568 19.2648 7.63502 19.206 7.95941C19.1472 8.28381 19.1869 8.61838 19.32 8.92V9C19.4468 9.29577 19.6572 9.54802 19.9255 9.72569C20.1938 9.90337 20.5082 9.99872 20.83 10H21C21.5304 10 22.0391 10.2107 22.4142 10.5858C22.7893 10.9609 23 11.4696 23 12C23 12.5304 22.7893 13.0391 22.4142 13.4142C22.0391 13.7893 21.5304 14 21 14H20.91C20.5882 14.0013 20.2738 14.0966 20.0055 14.2743C19.7372 14.452 19.5268 14.7042 19.4 15Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </button>
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
            :priority-color="priorityColor"
            @toggle="(index) => toggleTodo('default', index)"
            @delete="(index) => deleteTodo('default', index)"
            @contextmenu="showTodoContextMenu"
            @edit="handleEditTodo"
            @toggle-priority="handleTogglePriority"
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
            :priority-color="priorityColor"
            @toggleCollapse="toggleGroupCollapse(group.id)"
            @showMenu="(event) => showGroupContextMenu(event, group)"
            @toggle-todo="(index: number) => toggleTodo(group.id, index)"
            @delete-todo="(index: number) => deleteTodo(group.id, index)"
            @todo-contextmenu="showTodoContextMenu"
            @edit-todo="handleEditTodo"
            @toggle-priority="handleTogglePriority"
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
                :priority-color="priorityColor"
                :show-border="true"
                @toggle="(index) => toggleCompletedTodo(index)"
                @delete="(index) => deleteCompletedTodo(index)"
                @contextmenu="showTodoContextMenu"
                @edit="handleEditTodo"
                @toggle-priority="handleTogglePriority"
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
      @remove-old-completed="removeOldCompletedTodos"
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
const priorityColor = ref('#FF9800');

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
    groupId: 'default',
    priority: 0 // 默认优先级为普通
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
    todos.value[todoIndex].completedAt = Math.floor(Date.now() / 1000); // 记录完成时间
    saveTodoData();
  }
}

// 切换已完成任务状态
function toggleCompletedTodo(index: number) {
  const todo = allCompletedTodos.value[index];
  const todoIndex = todos.value.findIndex(t => t.id === todo.id);
  
  if (todoIndex !== -1) {
    todos.value[todoIndex].completed = false;
    todos.value[todoIndex].completedAt = undefined; // 清除完成时间
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

// 移除完成7天前的任务
function removeOldCompletedTodos() {
  const now = Math.floor(Date.now() / 1000);
  const sevenDaysAgo = now - (7 * 24 * 60 * 60); // 7天前的时间戳
  
  const beforeCount = todos.value.length;
  todos.value = todos.value.filter(t => {
    // 保留未完成的任务
    if (!t.completed) return true;
    // 保留没有完成时间的任务（兼容旧数据）
    if (!t.completedAt) return true;
    // 保留7天内完成的任务
    return t.completedAt > sevenDaysAgo;
  });
  
  const removedCount = beforeCount - todos.value.length;
  
  if (removedCount > 0) {
    saveTodoData();
    showToastMessage(`已移除 ${removedCount} 个完成7天前的任务`, 'success');
  } else {
    showToastMessage('没有完成7天前的任务', 'warning');
  }
  
  hideContextMenu();
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

// 处理双击编辑任务
function handleEditTodo(todo: Todo) {
  editDialogTodo.value = todo;
  showEditDialog.value = true;
}

// 处理切换优先级
function handleTogglePriority(todo: Todo) {
  const todoIndex = todos.value.findIndex(t => t.id === todo.id);
  if (todoIndex !== -1) {
    // 切换优先级：0 <-> 1
    todos.value[todoIndex].priority = todos.value[todoIndex].priority === 1 ? 0 : 1;
    saveTodoData();
  }
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
      completed_at: todo.completedAt || null,
      deadline: todo.deadline || null,
      order: todo.order,
      group_id: todo.groupId,
      priority: todo.priority || 0
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
      todos: { id: string; text: string; completed: boolean; created_at: number; completed_at?: number; deadline?: number; order: number; group_id: string; priority?: number }[]
    };
    
    todos.value = data.todos.map((todo, index) => ({
      id: todo.id,
      text: todo.text,
      completed: todo.completed,
      createdAt: todo.created_at,
      completedAt: todo.completed_at,
      deadline: todo.deadline,
      order: todo.order ?? index, // 如果没有order，使用索引
      groupId: todo.group_id || 'default', // 如果没有groupId，使用default
      priority: todo.priority ?? 0 // 如果没有priority，默认为0
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
      theme: string,
      priority_color: string,
      window_level: string
    };
    isDragDisabled.value = settings.disable_drag;
    priorityColor.value = settings.priority_color || '#FF9800';
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

// 监听高优先级颜色变化
async function listenPriorityColorChange() {
  const currentWindow = getCurrentWindow();
  await currentWindow.listen('priority-color-changed', (event) => {
    priorityColor.value = event.payload as string;
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
  await listenPriorityColorChange();
  
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
  background: transparent;
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
  border-radius: 12px;
}

header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  font-size: 0.9rem;
  background: rgba(255, 255, 255, 0.6);
  border-bottom: 1px solid rgba(229, 231, 235, 0.2);
  color: #333;
  font-weight: 600;
  backdrop-filter: blur(10px);
  min-height: 36px;
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
  gap: 8px;
}

.progress-indicator {
  font-size: 0.75rem;
  color: #333;
  font-weight: 600;
  background: rgba(255, 255, 255, 0.8);
  padding: 3px 7px;
  border-radius: 10px;
  border: 1px solid rgba(229, 231, 235, 0.2);
  min-width: 32px;
  text-align: center;
  backdrop-filter: blur(5px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.settings-btn {
  width: 26px;
  height: 26px;
  border: none;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.8);
  color: #333;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  flex-shrink: 0;
  backdrop-filter: blur(5px);
  border: 1px solid rgba(229, 231, 235, 0.2);
  padding: 0;
}

.settings-btn svg {
  width: 16px;
  height: 16px;
  transition: transform 0.3s ease;
}

.settings-btn:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.12);
  background: rgba(255, 255, 255, 0.95);
}

.settings-btn:hover svg {
  transform: rotate(90deg);
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
  padding: 10px;
  display: flex;
  flex-direction: column;
  min-height: 100%;
}



.active-groups {
  flex: 0 0 auto;
  margin-bottom: 10px;
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
  padding: 5px 9px;
  background: rgba(255, 255, 255, 0.5);
  border-radius: 9px;
  cursor: pointer;
  transition: all 0.3s ease;
  user-select: none;
  gap: 7px;
  border: 1px solid rgba(229, 231, 235, 0.2);
  backdrop-filter: blur(5px);
  min-height: 30px;
}

.completed-group .group-header:hover {
  background: rgba(255, 255, 255, 0.7);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.collapse-indicator {
  font-size: 0.65rem;
  transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  color: #94a3b8;
  flex-shrink: 0;
}

.collapse-indicator.collapsed {
  transform: rotate(-90deg);
}

.group-icon {
  width: 18px;
  height: 18px;
  flex-shrink: 0;
}

.group-name {
  flex: 1;
  font-size: 0.8rem;
  font-weight: 600;
  color: #475569;
}

.group-count {
  background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
  color: white;
  border-radius: 10px;
  padding: 1.5px 7px;
  font-size: 0.65rem;
  font-weight: bold;
  min-width: 20px;
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
  padding: 3px;
  border-radius: 4px;
  transition: all 0.2s ease;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 22px;
  height: 22px;
}

.clear-completed-btn svg {
  width: 15px;
  height: 15px;
}

.clear-completed-btn:hover {
  background: rgba(244, 67, 54, 0.1);
  color: #f44336;
  transform: scale(1.1);
}

.group-content {
  padding: 5px;
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
body.dark-theme .container {
  background: #1a1d23;
}

body.dark-theme header {
  background: rgba(30, 33, 39, 0.8);
  border-bottom: none;
  color: #e8eaed;
}

body.dark-theme .progress-indicator {
  background: rgba(42, 45, 52, 0.8);
  color: #e8eaed;
  border: none;
}

body.dark-theme .settings-btn {
  background: rgba(42, 45, 52, 0.8);
  color: #e8eaed;
  border: none;
}

body.dark-theme .completed-group {
  border-top: 1px dashed rgba(255, 255, 255, 0.05);
}

body.dark-theme .completed-group .group-header {
  background: rgba(42, 45, 52, 0.5);
  border: none;
}

body.dark-theme .completed-group .group-header:hover {
  background: rgba(48, 52, 60, 0.6);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
}

body.dark-theme .group-name {
  color: #e8eaed;
}

body.dark-theme .collapse-indicator {
  color: #9ca3af;
}

body.dark-theme .clear-completed-btn {
  color: #9ca3af;
}

body.dark-theme .clear-completed-btn:hover {
  background: rgba(244, 67, 54, 0.15);
  color: #f87171;
}

body.dark-theme .group-icon circle {
  fill: #4CAF50;
}

body.dark-theme .group-icon path {
  stroke: white;
}




</style>
