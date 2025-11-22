<template>
  <div 
    :class="['todo-item', { completed: props.todo.completed }]"
    @dblclick="deleteTodo"
    @contextmenu.prevent="showContextMenu"
  >
    <span 
      class="todo-dot" 
      :style="{ backgroundColor: props.todo.color || '#d2dbd6' }"
    ></span>
    <span class="todo-text">{{ props.todo.text }}</span>
    
    <!-- 右侧操作按钮区域 -->
    <div class="action-buttons">
      <button 
        v-if="!props.todo.completed"
        class="action-btn complete-btn" 
        @click.stop="toggleTodo"
        title="完成"
      >
        ✓
      </button>
      <button 
        v-else
        class="action-btn uncomplete-btn" 
        @click.stop="toggleTodo"
        title="取消完成"
      >
        ↶
      </button>
      <button 
        class="action-btn drag-btn drag-handle" 
        @click.stop
        @mousedown.stop
        title="拖动排序"
      >
        ☰
      </button>
    </div>
    
    <!-- 时间指示器 - 放到最后 -->
    <Tooltip 
      v-if="props.todo.deadline && !props.todo.completed"
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
}

const props = defineProps<Props>();

const emit = defineEmits<{
  toggle: [index: number];
  delete: [index: number];
  contextmenu: [event: MouseEvent, todo: Todo];
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

function showContextMenu(event: MouseEvent) {
  emit('contextmenu', event, props.todo);
}

function formatCreatedDate(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');
  return `${year}-${month}-${day}`;
}

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
  padding: clamp(4px, 1vh, 6px) clamp(8px, 2vw, 10px);
  margin-bottom: clamp(3px, 0.8vh, 4px);
  border-radius: clamp(6px, 1.2vw, 8px);
  display: flex;
  align-items: center;
  gap: clamp(6px, 1.5vw, 8px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
  border: 1px solid rgba(229, 231, 235, 0.15);
  backdrop-filter: blur(10px);
  min-height: clamp(28px, 4vh, 32px);
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

/* 操作按钮区域 */
.action-buttons {
  display: flex;
  gap: clamp(4px, 1vw, 6px);
  opacity: 0;
  transition: opacity 0.3s ease;
  flex-shrink: 0;
}

.todo-item:hover .action-buttons {
  opacity: 1;
}

.action-btn {
  width: clamp(20px, 4vw, 24px);
  height: clamp(20px, 4vw, 24px);
  border: none;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: clamp(0.65rem, 1.5vw, 0.75rem);
  cursor: pointer;
  transition: all 0.2s ease;
  flex-shrink: 0;
  background: rgba(255, 255, 255, 0.8);
  color: #666;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
}

.action-btn:hover {
  transform: scale(1.1);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.complete-btn:hover {
  background: #4CAF50;
  color: white;
}

.complete-btn:active {
  transform: scale(0.9);
  animation: complete-pulse 0.4s ease;
}

.uncomplete-btn:hover {
  background: #FF9800;
  color: white;
}

.uncomplete-btn:active {
  transform: scale(0.9);
}

@keyframes complete-pulse {
  0% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.2);
    box-shadow: 0 0 0 8px rgba(76, 175, 80, 0.2);
  }
  100% {
    transform: scale(1);
  }
}

.drag-btn {
  cursor: grab;
  font-size: clamp(0.8rem, 2vw, 1rem);
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
  font-size: clamp(0.75rem, 2vw, 0.85rem);
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
  padding: clamp(2px, 0.5vh, 4px) clamp(6px, 1.2vw, 8px);
  font-size: clamp(0.6rem, 1.5vw, 0.7rem);
  font-weight: bold;
  color: #333;
  margin-left: auto;
  flex-shrink: 0;
  box-shadow: 0 2px 6px rgba(255, 224, 130, 0.4);
  border: 1px solid rgba(255, 224, 130, 0.6);
  user-select: none;
  white-space: nowrap;
  min-width: fit-content;
  transition: all 0.3s ease;
  cursor: help;
}

.days-indicator:hover {
  transform: scale(1.05);
  box-shadow: 0 3px 10px rgba(255, 224, 130, 0.6);
}

.countdown-indicator {
  background: #4CAF50;
  color: white;
  border-radius: 12px;
  padding: clamp(2px, 0.5vh, 4px) clamp(6px, 1.2vw, 8px);
  font-size: clamp(0.6rem, 1.3vw, 0.7rem);
  font-weight: bold;
  margin-left: auto;
  border: 1px solid rgba(76, 175, 80, 0.6);
  box-shadow: 0 2px 8px rgba(76, 175, 80, 0.3);
  backdrop-filter: blur(3px);
  flex-shrink: 0;
  white-space: nowrap;
  min-width: fit-content;
  transition: all 0.3s ease;
  user-select: none;
  cursor: help;
}

.countdown-indicator:hover {
  transform: scale(1.05);
  box-shadow: 0 3px 12px rgba(76, 175, 80, 0.5);
}

.countdown-indicator.overdue {
  background: #F44336;
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

/* 夜间主题 */
body.dark-theme .todo-item.completed {
  background: rgba(40, 40, 40, 0.8);
}
</style>
