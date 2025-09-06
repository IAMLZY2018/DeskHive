<template>
  <div 
    :class="['todo-item', { completed: props.todo.completed }]"
    @dblclick="deleteTodo"
    @contextmenu.prevent="showContextMenu"
  >
    <div 
      v-if="!props.todo.completed" 
      class="todo-checkbox" 
      @click="toggleTodo"
    ></div>
    <div 
      v-else 
      class="todo-checkbox completed" 
      @click="toggleTodo"
    ></div>
    <span>{{ props.todo.text }}</span>
    <div 
      v-if="props.todo.deadline && !props.todo.completed" 
      class="countdown-indicator" 
      :class="{ 'overdue': isOverdue(props.todo.deadline) }"
    >
      {{ getCountdownText(props.todo.deadline) }}
    </div>
    <div 
      v-else-if="calculateDaysCreated(props.todo.createdAt) >= 1" 
      class="days-indicator"
    >
      {{ calculateDaysCreated(props.todo.createdAt) }}天
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Todo } from '../../src/types';

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

// 切换任务完成状态
function toggleTodo() {
  emit('toggle', props.index);
}

// 删除任务
function deleteTodo() {
  emit('delete', props.index);
}

// 显示右键菜单
function showContextMenu(event: MouseEvent) {
  emit('contextmenu', event, props.todo);
}
</script>

<style scoped>
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

/* 天数指示器样式 */
.days-indicator {
  background: #FFE082; /* 更淡的黄色 */
  border-radius: 12px; /* 椭圆形状 */
  padding: clamp(2px, 0.5vh, 4px) clamp(6px, 1.2vw, 8px);
  font-size: clamp(0.6rem, 1.5vw, 0.7rem);
  font-weight: bold;
  color: #333;
  margin-right: clamp(6px, 1.5vw, 8px);
  flex-shrink: 0;
  box-shadow: 0 2px 6px rgba(255, 224, 130, 0.4);
  border: 1px solid rgba(255, 224, 130, 0.6);
  user-select: none;
  white-space: nowrap;
  min-width: fit-content;
  transition: all 0.3s ease;
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
</style>