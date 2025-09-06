<template>
  <div 
    v-if="props.show" 
    ref="contextMenuRef"
    class="context-menu" 
    :style="{ left: initialPosition.x + 'px', top: initialPosition.y + 'px' }"
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
    <div class="context-menu-button" @click="onDeleteTodo">
      🗑️ 删除任务
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, nextTick } from 'vue';
import type { Todo } from '../../src/types';

interface Props {
  show: boolean;
  position: { x: number; y: number };
  todo: Todo | null;
}

const props = defineProps<Props>();
const contextMenuRef = ref<HTMLElement | null>(null);
const initialPosition = ref({ x: 0, y: 0 });

const emit = defineEmits<{
  setDeadline: [];
  removeDeadline: [];
  deleteTodo: []; // 添加删除事件
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

// 删除任务
function onDeleteTodo() {
  emit('deleteTodo');
}

// 调整菜单位置以确保完整显示
function adjustMenuPosition() {
  if (!contextMenuRef.value) return;

  const menu = contextMenuRef.value;
  const rect = menu.getBoundingClientRect();
  const viewportWidth = window.innerWidth;
  const viewportHeight = window.innerHeight;

  // 获取初始位置
  let newX = props.position.x;
  let newY = props.position.y;

  // 检查右侧是否超出视口
  if (newX + rect.width > viewportWidth) {
    newX = viewportWidth - rect.width - 10; // 保持10px边距
  }

  // 检查底部是否超出视口
  if (newY + rect.height > viewportHeight) {
    newY = viewportHeight - rect.height - 10; // 保持10px边距
  }

  // 确保不会小于0
  newX = Math.max(0, newX);
  newY = Math.max(0, newY);

  // 应用调整后的位置
  menu.style.left = newX + 'px';
  menu.style.top = newY + 'px';
}

// 监听show属性变化，当菜单显示时调整位置
watch(() => props.show, (newVal) => {
  if (newVal) {
    // 设置初始位置
    initialPosition.value = { ...props.position };
    
    // 在DOM更新后调整位置
    nextTick(() => {
      adjustMenuPosition();
    });
  }
});

// 监听位置变化
watch(() => props.position, (newPos) => {
  if (props.show) {
    initialPosition.value = { ...newPos };
    
    // 在DOM更新后调整位置
    nextTick(() => {
      adjustMenuPosition();
    });
  }
});
</script>

<style scoped>
/* 右键菜单样式 */
.context-menu {
  position: fixed;
  background: rgba(255, 255, 255, 0.95);
  border: 1px solid rgba(0, 0, 0, 0.15);
  border-radius: 10px;
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.12);
  backdrop-filter: blur(20px);
  z-index: 1000;
  min-width: 200px;
  padding: 6px;
  font-family: 'Segoe UI', system-ui, -apple-system, sans-serif;
  box-sizing: border-box;
}

.context-menu-item {
  padding: 6px 10px;
  font-size: 0.8rem;
  color: #333;
  border-radius: 6px;
  transition: background-color 0.2s ease;
  margin: 1px 0;
}

.context-menu-item:hover {
  background: rgba(0, 0, 0, 0.03);
}

.context-menu-label {
  font-weight: 500;
  color: #666;
  margin-bottom: 2px;
  font-size: 0.75rem;
}

.context-menu-value {
  font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Fira Code', monospace;
  background: rgba(0, 0, 0, 0.05);
  padding: 4px 8px;
  border-radius: 5px;
  font-size: 0.75rem;
  color: #444;
  font-weight: 500;
  border: 1px solid rgba(0, 0, 0, 0.08);
}

.status-value {
  display: flex;
  align-items: center;
  gap: 6px;
  font-family: 'Segoe UI', system-ui, -apple-system, sans-serif;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  display: inline-block;
  flex-shrink: 0;
}

.status-dot.completed {
  background-color: #10b981; /* emerald-500 */
  box-shadow: 0 0 6px rgba(16, 185, 129, 0.4);
}

.status-dot.pending {
  background-color: #ef4444; /* red-500 */
  box-shadow: 0 0 6px rgba(239, 68, 68, 0.4);
}

/* 右键菜单分割线 */
.context-menu-divider {
  height: 1px;
  background: rgba(0, 0, 0, 0.08);
  margin: 4px 0;
}

/* 右键菜单按钮 */
.context-menu-button {
  padding: 6px 10px;
  font-size: 0.8rem;
  color: #333;
  cursor: pointer;
  border-radius: 6px;
  transition: all 0.2s ease;
  user-select: none;
  font-weight: 500;
  display: flex;
  align-items: center;
  gap: 6px;
  margin: 1px 0;
  border: 1px solid transparent;
}

.context-menu-button:hover {
  background: rgba(0, 0, 0, 0.05);
  color: #111;
  border: 1px solid rgba(0, 0, 0, 0.1);
}

.context-menu-button:active {
  background: rgba(0, 0, 0, 0.08);
  transform: translateY(1px);
}
</style>