<template>
  <div class="todo-group">
    <div 
      class="group-header"
      :class="{ active: isActive, 'drag-over': isDragOver }"
      @click="selectGroup"
      @contextmenu.prevent="showGroupMenu"
      @dragover.prevent="handleDragOver"
      @dragleave="handleDragLeave"
      @drop.prevent="handleDrop"
    >
      <span class="collapse-indicator" :class="{ collapsed: group.collapsed }" @click.stop="toggleCollapse">
        ▼
      </span>
      <span class="group-icon">📁</span>
      <span class="group-name">{{ group.name }}</span>
      <span class="group-count">{{ todoCount }}</span>
      <button class="group-menu-btn" @click.stop="showGroupMenu">⋮</button>
    </div>
    
    <div v-show="!group.collapsed" class="group-content">
      <TodoList
        :todos="todos"
        @toggle="(index) => emit('toggle-todo', index)"
        @delete="(index) => emit('delete-todo', index)"
        @contextmenu="(event, todo) => emit('todo-contextmenu', event, todo)"
        @reorder="(newOrder) => emit('reorder', newOrder)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import TodoList from './TodoList.vue';
import type { TodoGroup, Todo } from '../types';

interface Props {
  group: TodoGroup;
  todos: Todo[];
  isActive: boolean;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  'select-group': [];
  'toggle-collapse': [];
  'show-menu': [event: MouseEvent];
  'toggle-todo': [index: number];
  'delete-todo': [index: number];
  'todo-contextmenu': [event: MouseEvent, todo: Todo];
  'drop-to-group': [];
  'reorder': [newOrder: Todo[]];
}>();

const todoCount = computed(() => props.todos.filter(t => !t.completed).length);
const isDragOver = ref(false);

function selectGroup() {
  emit('select-group');
}

function toggleCollapse() {
  emit('toggle-collapse');
}

function showGroupMenu(event: MouseEvent) {
  emit('show-menu', event);
}

// 拖拽悬停在分组标题上
function handleDragOver(event: DragEvent) {
  event.preventDefault();
  isDragOver.value = true;
  if (event.dataTransfer) {
    event.dataTransfer.dropEffect = 'move';
  }
}

// 拖拽离开分组标题
function handleDragLeave() {
  isDragOver.value = false;
}

// 放置到分组标题
function handleDrop(event: DragEvent) {
  event.preventDefault();
  isDragOver.value = false;
  emit('drop-to-group');
}
</script>

<style scoped>
.todo-group {
  margin-bottom: 10px;
}

.group-header {
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

.group-header:hover {
  background: rgba(255, 255, 255, 0.7);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.group-header.active {
  background: rgba(255, 255, 255, 0.8);
  border-color: #007aff;
  box-shadow: 0 4px 12px rgba(0, 122, 255, 0.2);
}

.group-header.drag-over {
  background: rgba(0, 122, 255, 0.1);
  border-color: #007aff;
  border-style: dashed;
  box-shadow: 0 4px 12px rgba(0, 122, 255, 0.3);
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

.group-menu-btn {
  background: transparent;
  border: none;
  cursor: pointer;
  font-size: 1rem;
  color: #888;
  padding: 2px 6px;
  border-radius: 4px;
  transition: all 0.2s ease;
}

.group-menu-btn:hover {
  background: rgba(0, 0, 0, 0.05);
  color: #333;
}

.group-content {
  padding: clamp(8px, 2vh, 12px) clamp(4px, 1vw, 8px);
}

/* 夜间主题 */
body.dark-theme .group-header {
  background: rgba(37, 38, 39, 0.5);
  border-color: rgba(231, 233, 237, 0.2);
}

body.dark-theme .group-header:hover {
  background: rgba(37, 38, 39, 0.7);
}

body.dark-theme .group-header.active {
  background: rgba(37, 38, 39, 0.8);
  border-color: #007aff;
}

body.dark-theme .group-name {
  color: #e7e9ed;
}

body.dark-theme .collapse-indicator {
  color: #aaa;
}

body.dark-theme .group-menu-btn {
  color: #aaa;
}

body.dark-theme .group-menu-btn:hover {
  background: rgba(255, 255, 255, 0.05);
  color: #e7e9ed;
}
</style>
