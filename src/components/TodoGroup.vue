<template>
  <div class="todo-group">
    <div 
      class="group-header"
      :class="{ 'drag-over': isDragOver }"
      @click="handleHeaderClick"
      @contextmenu.prevent="showGroupMenu"
      @dragenter.prevent="handleDragEnter"
      @dragover.prevent="handleDragOver"
      @dragleave.prevent="handleDragLeave"
      @drop.prevent="handleDrop"
    >
      <span class="collapse-indicator" :class="{ collapsed: group.collapsed }">
        ▼
      </span>
      <span class="group-name">{{ group.name }}</span>
      
      <!-- 操作按钮区域 - 鼠标悬停时显示 -->
      <div class="group-actions">
        <!-- 排序按钮 -->
        <div class="sort-buttons">
          <button class="sort-btn" @click.stop="moveUp" title="上移">▲</button>
          <button class="sort-btn" @click.stop="moveDown" title="下移">▼</button>
        </div>
        
        <button class="group-menu-btn" @click.stop="showGroupMenu">⋮</button>
      </div>
      
      <!-- 任务数量 - 放到最后 -->
      <span class="group-count">{{ todoCount }}</span>
    </div>
    
    <div v-show="!group.collapsed" class="group-content">
      <TodoList
        :todos="todos"
        :show-border="true"
        @toggle="(index) => emit('toggle-todo', index)"
        @delete="(index) => emit('delete-todo', index)"
        @contextmenu="(event, todo) => emit('todo-contextmenu', event, todo)"
        @edit="(todo) => emit('edit-todo', todo)"
        @reorder="(newOrder) => emit('reorder', newOrder)"
        @drag-start="(todo) => emit('drag-start', todo)"
        @drag-end="emit('drag-end')"
        @change="(event) => emit('change', event)"
      />
      <div v-if="todos.length === 0" class="empty-group-hint">
        拖动任务到这里或点击下方添加新任务
      </div>
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
}

const props = defineProps<Props>();

const emit = defineEmits<{
  'toggleCollapse': [];
  'showMenu': [event: MouseEvent];
  'toggle-todo': [index: number];
  'delete-todo': [index: number];
  'todo-contextmenu': [event: MouseEvent, todo: Todo];
  'edit-todo': [todo: Todo];
  'reorder': [newOrder: Todo[]];
  'drag-start': [todo: Todo];
  'drag-end': [];
  'move-up': [];
  'move-down': [];
  'change': [event: any];
  'drop-on-header': [];
}>();

const todoCount = computed(() => props.todos.filter(t => !t.completed).length);
const isDragOver = ref(false);

function handleHeaderClick() {
  // 点击分组标题时，展开/折叠分组
  emit('toggleCollapse');
}

function showGroupMenu(event: MouseEvent) {
  emit('showMenu', event);
}

function moveUp() {
  console.log('TodoGroup moveUp clicked:', props.group.name);
  emit('move-up');
}

function moveDown() {
  console.log('TodoGroup moveDown clicked:', props.group.name);
  emit('move-down');
}

function handleDragEnter(_event: DragEvent) {
  isDragOver.value = true;
}

function handleDragOver(event: DragEvent) {
  event.preventDefault();
  isDragOver.value = true;
}

function handleDragLeave(event: DragEvent) {
  // 检查是否真的离开了元素
  const rect = (event.currentTarget as HTMLElement).getBoundingClientRect();
  const x = event.clientX;
  const y = event.clientY;
  
  if (x < rect.left || x >= rect.right || y < rect.top || y >= rect.bottom) {
    isDragOver.value = false;
  }
}

function handleDrop(_event: DragEvent) {
  isDragOver.value = false;
  console.log('Drop on group header:', props.group.name);
  emit('drop-on-header');
}
</script>

<style scoped>
.todo-group {
  margin-bottom: clamp(6px, 1.5vh, 8px);
  width: 100%;
  position: relative;
  z-index: 1;
}

.group-header {
  display: flex;
  align-items: center;
  padding: clamp(4px, 1vh, 6px) clamp(8px, 2vw, 10px);
  background: rgba(248, 250, 252, 0.6);
  border-radius: clamp(8px, 1.5vw, 10px);
  cursor: pointer;
  transition: background 0.3s ease, box-shadow 0.3s ease, border-color 0.3s ease;
  user-select: none;
  gap: clamp(6px, 1.5vw, 8px);
  border: 1px solid rgba(226, 232, 240, 0.5);
  backdrop-filter: blur(5px);
  min-height: clamp(28px, 4vh, 32px);
  width: 100%;
}

.group-header:hover {
  background: rgba(241, 245, 249, 0.8);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
}

.group-header.drag-over {
  background: rgba(59, 130, 246, 0.1);
  border-color: rgba(59, 130, 246, 0.5);
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.2);
  transform: scale(1.02);
}

.sort-buttons {
  display: flex;
  flex-direction: column;
  gap: 1px;
  flex-shrink: 0;
}

.sort-btn {
  background: transparent;
  border: none;
  cursor: pointer;
  font-size: clamp(0.5rem, 1.2vw, 0.6rem);
  color: #94a3b8;
  padding: 0;
  line-height: 1;
  transition: all 0.2s ease;
  width: 14px;
  height: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.sort-btn:hover {
  color: #475569;
  transform: scale(1.2);
}

.collapse-indicator {
  font-size: clamp(0.6rem, 1.5vw, 0.7rem);
  transition: transform 0.3s ease;
  color: #94a3b8;
  flex-shrink: 0;
}

.collapse-indicator.collapsed {
  transform: rotate(-90deg);
}

.group-name {
  flex: 1;
  font-size: clamp(0.75rem, 2vw, 0.85rem);
  font-weight: 600;
  color: #475569;
}

.group-actions {
  display: flex;
  align-items: center;
  gap: clamp(4px, 1vw, 6px);
  opacity: 0;
  transition: opacity 0.3s ease;
  flex-shrink: 0;
}

.group-header:hover .group-actions {
  opacity: 1;
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

.group-menu-btn {
  background: transparent;
  border: none;
  cursor: pointer;
  font-size: clamp(0.85rem, 2vw, 0.95rem);
  color: #94a3b8;
  padding: clamp(1px, 0.3vh, 2px) clamp(4px, 1vw, 6px);
  border-radius: 4px;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.group-menu-btn:hover {
  background: rgba(148, 163, 184, 0.1);
  color: #475569;
}

.group-content {
  padding: clamp(4px, 1vh, 6px) clamp(4px, 1vw, 6px);
  width: 100%;
  overflow: visible;
  animation: slideDown 0.3s ease;
  position: relative;
  min-height: 0;
}

@keyframes slideDown {
  from {
    opacity: 0;
    max-height: 0;
  }
  to {
    opacity: 1;
    max-height: 1000px;
  }
}

/* 夜间主题 */
body.dark-theme .group-header {
  background: rgba(37, 38, 39, 0.5);
  border-color: rgba(231, 233, 237, 0.2);
}

body.dark-theme .group-header:hover {
  background: rgba(37, 38, 39, 0.7);
}

body.dark-theme .group-header.drag-over {
  background: rgba(0, 122, 255, 0.15);
  border-color: rgba(0, 122, 255, 0.5);
  box-shadow: 0 4px 12px rgba(0, 122, 255, 0.2);
}

body.dark-theme .sort-btn {
  color: #aaa;
}

body.dark-theme .sort-btn:hover {
  color: #e7e9ed;
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

.empty-group-hint {
  text-align: center;
  padding: clamp(12px, 2.5vh, 16px) clamp(8px, 2vw, 12px);
  color: #bbb;
  font-size: clamp(0.65rem, 1.6vw, 0.75rem);
  pointer-events: none;
  user-select: none;
  white-space: nowrap;
  min-height: clamp(40px, 8vh, 50px);
  display: flex;
  align-items: center;
  justify-content: center;
}

body.dark-theme .empty-group-hint {
  color: #aaa;
  background: rgba(255, 255, 255, 0.02);
  border-color: rgba(255, 255, 255, 0.1);
}
</style>
