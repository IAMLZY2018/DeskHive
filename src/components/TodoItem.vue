<template>
  <div 
    :class="['todo-item', { completed: props.todo.completed }]"
    @dblclick="togglePriority"
    @contextmenu.prevent="showContextMenu"
  >
    <span 
      class="todo-dot" 
      :style="{ backgroundColor: getPriorityColor() }"
    ></span>
    <span class="todo-text">{{ props.todo.text }}</span>
    
    <!-- 悬浮操作按钮区域 -->
    <div class="action-buttons">
      <Tooltip 
        v-if="!props.todo.completed"
        text="完成"
        :delay="500"
      >
        <button 
          class="action-btn complete-btn" 
          @click.stop="toggleTodo"
        >
          <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M20 6L9 17L4 12" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </button>
      </Tooltip>
      <Tooltip 
        v-else
        text="撤销"
        :delay="500"
      >
        <button 
          class="action-btn uncomplete-btn" 
          @click.stop="toggleTodo"
        >
          <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M3 12C3 16.9706 7.02944 21 12 21C16.9706 21 21 16.9706 21 12C21 7.02944 16.9706 3 12 3C9.5 3 7.25 4.1 5.75 5.85" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
            <path d="M3 3V8H8" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </button>
      </Tooltip>
      <Tooltip 
        text="拖动排序"
        :delay="500"
      >
        <button 
          class="action-btn drag-btn drag-handle" 
          @click.stop
          @mousedown.stop
        >
          <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M8 6H16M8 12H16M8 18H16" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"/>
          </svg>
        </button>
      </Tooltip>
    </div>
    
    <!-- 时间指示器 - 放到最右侧 -->
    <Tooltip 
      v-if="props.todo.completed"
      :text="getCompletedDaysTooltip()"
      :delay="300"
    >
      <div class="completed-indicator">
        耗时{{ calculateDaysCreated(props.todo.createdAt) }}天
      </div>
    </Tooltip>
    <Tooltip 
      v-else-if="props.todo.deadline"
      :text="getCountdownTooltip()"
      :delay="300"
    >
      <div 
        class="countdown-indicator" 
        :class="{ 'overdue': isOverdue(props.todo.deadline) }"
      >
        {{ getCountdownText(props.todo.deadline) }}
      </div>
    </Tooltip>
    <Tooltip 
      v-else-if="calculateDaysCreated(props.todo.createdAt) >= 1"
      :text="getDaysIndicatorTooltip()"
      :delay="300"
    >
      <div class="days-indicator">
        {{ calculateDaysCreated(props.todo.createdAt) }}天
      </div>
    </Tooltip>
  </div>
</template>

<script setup lang="ts">
import Tooltip from './Tooltip.vue';
import type { Todo } from '../types';

interface Props {
  todo: Todo;
  index: number;
  isCompletedList?: boolean;
  priorityColor?: string;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  toggle: [index: number];
  delete: [index: number];
  contextmenu: [event: MouseEvent, todo: Todo];
  edit: [todo: Todo];
  togglePriority: [todo: Todo];
}>();

function calculateDaysCreated(timestamp: number): number {
  const now = Date.now();
  const createdTime = timestamp * 1000;
  const diffMs = now - createdTime;
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));
  return diffDays;
}

function isOverdue(deadline: number): boolean {
  const now = Math.floor(Date.now() / 1000);
  return deadline < now;
}

function getCountdownText(deadline: number): string {
  const now = Math.floor(Date.now() / 1000);
  const diff = deadline - now;
  
  if (diff <= 0) {
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

function toggleTodo() {
  emit('toggle', props.index);
}

function deleteTodo() {
  emit('delete', props.index);
}

function editTodo() {
  emit('edit', props.todo);
}

function togglePriority() {
  emit('togglePriority', props.todo);
}

function getPriorityColor(): string {
  // 优先级 1 = 自定义颜色，0 = 灰色
  return props.todo.priority === 1 ? (props.priorityColor || '#FF9800') : '#d2dbd6';
}

function showContextMenu(event: MouseEvent) {
  emit('contextmenu', event, props.todo);
}

// function formatCreatedDate(timestamp: number): string {
//   const date = new Date(timestamp * 1000);
//   const year = date.getFullYear();
//   const month = String(date.getMonth() + 1).padStart(2, '0');
//   const day = String(date.getDate()).padStart(2, '0');
//   return `${year}-${month}-${day}`;
// }

function formatDeadlineDate(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');
  return `${year}-${month}-${day}`;
}

function getDaysIndicatorTooltip(): string {
  const days = calculateDaysCreated(props.todo.createdAt);
  return `已创建 ${days} 天`;
}

function getCompletedDaysTooltip(): string {
  const days = calculateDaysCreated(props.todo.createdAt);
  return `从创建到完成耗时 ${days} 天`;
}

function getCountdownTooltip(): string {
  if (!props.todo.deadline) return '';
  const deadlineDate = formatDeadlineDate(props.todo.deadline);
  const now = Math.floor(Date.now() / 1000);
  const isOverdueNow = props.todo.deadline < now;
  
  if (isOverdueNow) {
    return `已超时 ${deadlineDate}`;
  } else {
    return `截止 ${deadlineDate}`;
  }
}
</script>

<style scoped>
.todo-item {
  background: rgba(255, 255, 255, 0.9);
  padding: 5px 9px;
  margin-bottom: 3.5px;
  border-radius: 7px;
  display: flex;
  align-items: center;
  gap: 7px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
  border: 1px solid rgba(229, 231, 235, 0.15);
  backdrop-filter: blur(10px);
  min-height: 30px;
  cursor: default;
  position: relative;
  width: 100%;
}

.todo-item:last-child {
  margin-bottom: 0;
}

.todo-item:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
  border-color: rgba(229, 231, 235, 0.2);
}

/* 悬浮操作按钮区域 */
.action-buttons {
  position: absolute;
  right: 6px;
  top: 50%;
  transform: translateY(-50%);
  display: flex;
  gap: 3px;
  opacity: 0;
  pointer-events: none;
  transition: opacity 0.2s ease;
  z-index: 10;
}

.todo-item:hover .action-buttons {
  opacity: 1;
  pointer-events: auto;
}

.action-btn {
  width: 20px;
  height: 20px;
  border: none;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s ease;
  background: rgba(255, 255, 255, 0.95);
  color: #666;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.08);
  backdrop-filter: blur(10px);
  padding: 0;
}

.action-btn svg {
  width: 12px;
  height: 12px;
}

.action-btn:hover {
  transform: scale(1.1);
  box-shadow: 0 3px 10px rgba(0, 0, 0, 0.15);
}

.action-btn:active {
  transform: scale(0.95);
}

.complete-btn:hover {
  background: #6EE748;
  color: white;
}

.complete-btn:active {
  animation: complete-pulse 0.4s ease;
}

.uncomplete-btn:hover {
  background: #FF9800;
  color: white;
}

@keyframes complete-pulse {
  0% {
    transform: scale(0.95);
  }
  50% {
    transform: scale(1.15);
    box-shadow: 0 0 0 6px rgba(110, 231, 72, 0.2);
  }
  100% {
    transform: scale(1);
  }
}

.drag-btn {
  cursor: grab;
}

.drag-btn:hover {
  background: #007aff;
  color: white;
}

.drag-btn:active {
  cursor: grabbing;
}

.todo-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
  transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}

.todo-item.completed .todo-dot {
  opacity: 0.4;
  transform: scale(0.8);
}

.todo-text {
  font-size: 0.8rem;
  word-break: break-word;
  flex: 1;
  line-height: 1.2;
  font-weight: 500;
  color: #333;
  user-select: none;
}

.todo-item.completed {
  opacity: 0.7;
  transform: scale(0.98);
  background: rgba(245, 245, 245, 0.8);
}

.todo-item.completed .todo-text {
  text-decoration: line-through;
  color: #888;
}

.days-indicator {
  background: #FFE082;
  border-radius: 12px;
  padding: 3px 7px;
  font-size: 0.65rem;
  font-weight: bold;
  color: #333;
  margin-left: auto;
  margin-right: 4px;
  flex-shrink: 0;
  box-shadow: 0 2px 6px rgba(255, 224, 130, 0.4);
  border: 1px solid rgba(255, 224, 130, 0.6);
  user-select: none;
  white-space: nowrap;
  min-width: fit-content;
  transition: all 0.3s ease;
  cursor: help;
}

.todo-item:hover .days-indicator {
  opacity: 0;
  pointer-events: none;
}

.countdown-indicator {
  background: #6EE748;
  color: #000;
  border-radius: 12px;
  padding: 3px 7px;
  font-size: 0.65rem;
  font-weight: bold;
  margin-left: auto;
  margin-right: 4px;
  border: 1px solid rgba(110, 231, 72, 0.6);
  box-shadow: 0 2px 8px rgba(110, 231, 72, 0.3);
  backdrop-filter: blur(3px);
  flex-shrink: 0;
  white-space: nowrap;
  min-width: fit-content;
  transition: all 0.3s ease;
  user-select: none;
  cursor: help;
}

.todo-item:hover .countdown-indicator {
  opacity: 0;
  pointer-events: none;
}

.countdown-indicator.overdue {
  background: #F44336;
  color: white;
  border-color: rgba(244, 67, 54, 0.6);
  box-shadow: 0 2px 8px rgba(244, 67, 54, 0.3);
}

.countdown-indicator.overdue:hover {
  box-shadow: 0 3px 12px rgba(244, 67, 54, 0.5);
}

.countdown-indicator.completed {
  background: #9E9E9E;
  border-color: rgba(158, 158, 158, 0.6);
  box-shadow: 0 2px 8px rgba(158, 158, 158, 0.3);
  opacity: 0.8;
}

.countdown-indicator.completed.overdue {
  background: #9E9E9E;
  border-color: rgba(158, 158, 158, 0.6);
  box-shadow: 0 2px 8px rgba(158, 158, 158, 0.3);
  opacity: 0.8;
}

.completed-indicator {
  background: #9E9E9E;
  color: white;
  border-radius: 12px;
  padding: 3px 7px;
  font-size: 0.65rem;
  font-weight: bold;
  margin-left: auto;
  margin-right: 4px;
  border: 1px solid rgba(158, 158, 158, 0.6);
  box-shadow: 0 2px 8px rgba(158, 158, 158, 0.3);
  backdrop-filter: blur(3px);
  flex-shrink: 0;
  white-space: nowrap;
  min-width: fit-content;
  transition: all 0.3s ease;
  user-select: none;
  cursor: help;
  opacity: 0.8;
}

.todo-item:hover .completed-indicator {
  opacity: 0;
  pointer-events: none;
}

/* 夜间主题 */
body.dark-theme .todo-item {
  background: rgba(42, 45, 52, 0.6);
  border: none;
  color: #e8eaed;
  box-shadow: none;
}

body.dark-theme .todo-item:hover {
  background: rgba(48, 52, 60, 0.7);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
}

body.dark-theme .todo-item.completed {
  background: rgba(35, 38, 44, 0.4);
}

body.dark-theme .todo-text {
  color: #e8eaed;
}

body.dark-theme .todo-item.completed .todo-text {
  color: #9ca3af;
}

body.dark-theme .action-btn {
  background: rgba(60, 65, 75, 0.9);
  color: #9ca3af;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.2);
}

body.dark-theme .action-btn:hover {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}

body.dark-theme .complete-btn:hover {
  background: #6EE748;
  color: #000;
}

body.dark-theme .uncomplete-btn:hover {
  background: #FF9800;
  color: #fff;
}

body.dark-theme .drag-btn:hover {
  background: #007aff;
  color: #fff;
}

body.dark-theme .completed-indicator {
  background: #5f6368;
  border-color: rgba(95, 99, 104, 0.4);
  box-shadow: none;
}
</style>
