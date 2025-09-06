<template>
  <div 
    v-if="props.show" 
    class="context-menu" 
    :style="{ left: props.position.x + 'px', top: props.position.y + 'px' }"
  >
    <div class="context-menu-item">
      <div class="context-menu-label">状态：</div>
      <div class="context-menu-value status-value">
        <span :class="['status-dot', props.todo?.completed ? 'completed' : 'pending']"></span>
        {{ props.todo?.completed ? '已完成' : '待完成' }}
      </div>
    </div>
    <div class="context-menu-item">
      <div class="context-menu-label">创建时间：</div>
      <div class="context-menu-value">
        {{ props.todo ? formatDateTime(props.todo.createdAt) : '' }}
      </div>
    </div>
    <div v-if="props.todo?.deadline" class="context-menu-item">
      <div class="context-menu-label">截止时间：</div>
      <div class="context-menu-value">
        {{ formatDateTime(props.todo.deadline) }}
      </div>
    </div>
    <div class="context-menu-divider"></div>
    <div class="context-menu-button" @click="onSetDeadline">
      📅 设置截止时间
    </div>
    <div v-if="props.todo?.deadline" class="context-menu-button" @click="onRemoveDeadline">
      🗑️ 移除截止时间
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Todo } from '../../src/types';

interface Props {
  show: boolean;
  position: { x: number; y: number };
  todo: Todo | null;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  setDeadline: [];
  removeDeadline: [];
}>();

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

// 设置截止时间
function onSetDeadline() {
  emit('setDeadline');
}

// 移除截止时间
function onRemoveDeadline() {
  emit('removeDeadline');
}
</script>

<style scoped>
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
</style>