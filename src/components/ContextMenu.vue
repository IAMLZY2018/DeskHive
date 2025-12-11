<template>
  <div 
    v-if="props.show" 
    ref="contextMenuRef"
    class="context-menu" 
    :style="{ left: initialPosition.x + 'px', top: initialPosition.y + 'px' }"
  >
    <!-- 状态信息 -->
    <div class="menu-info">
      <div class="info-row">
        <span :class="['status-dot', props.todo?.completed ? 'completed' : 'pending']"></span>
        <span class="info-text">{{ props.todo?.completed ? '已完成' : '待完成' }}</span>
        <span v-if="props.todo?.priority === 1" class="time-indicator priority-high">
          高优先级
        </span>
        <span v-if="!props.todo?.completed && getTimeIndicator()" :class="['time-indicator', getTimeIndicatorClass()]">
          {{ getTimeIndicator() }}
        </span>
      </div>
      <div v-if="props.todo?.completed" class="info-row">
        <span class="info-label">耗时</span>
        <span class="info-text">{{ getCompletedDaysText() }}</span>
      </div>
      <div v-if="!props.todo?.completed" class="info-row">
        <span class="info-label">创建</span>
        <span class="info-text">{{ props.todo ? formatDateTime(props.todo.createdAt) : '' }}</span>
      </div>

      <div v-if="props.todo?.deadline && !props.todo?.completed" class="info-row">
        <span class="info-label">截止</span>
        <span class="info-text">{{ formatDeadlineTime(props.todo.deadline) }}</span>
      </div>
      <div v-if="props.todo?.deadline && !props.todo?.completed" class="info-row">
        <span class="info-label"></span>
        <span :class="['deadline-status', getDeadlineStatusClass()]">{{ getDeadlineStatus() }}</span>
      </div>
    </div>
    
    <div class="menu-divider"></div>
    
    <!-- 操作按钮 -->
    <div class="menu-actions">
      <button v-if="!props.todo?.completed" class="menu-btn" @click="onEditTodo">
        <svg class="menu-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M11 4H4C3.46957 4 2.96086 4.21071 2.58579 4.58579C2.21071 4.96086 2 5.46957 2 6V20C2 20.5304 2.21071 21.0391 2.58579 21.4142C2.96086 21.7893 3.46957 22 4 22H18C18.5304 22 19.0391 21.7893 19.4142 21.4142C19.7893 21.0391 20 20.5304 20 20V13" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M18.5 2.50001C18.8978 2.10219 19.4374 1.87869 20 1.87869C20.5626 1.87869 21.1022 2.10219 21.5 2.50001C21.8978 2.89784 22.1213 3.4374 22.1213 4.00001C22.1213 4.56262 21.8978 5.10219 21.5 5.50001L12 15L8 16L9 12L18.5 2.50001Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <span>编辑任务</span>
      </button>
      
      <button v-if="!props.todo?.completed" class="menu-btn" @click="onSetDeadline">
        <svg class="menu-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect x="3" y="4" width="18" height="18" rx="2" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M16 2V6M8 2V6M3 10H21" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <span>{{ props.todo?.deadline ? '修改截止时间' : '设置截止时间' }}</span>
      </button>
      
      <button v-if="!props.todo?.completed && props.todo?.deadline" class="menu-btn" @click="onRemoveDeadline">
        <svg class="menu-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M18 6L6 18M6 6L18 18" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <span>移除截止时间</span>
      </button>
      
      <button class="menu-btn delete-btn" @click="onDeleteTodo">
        <svg class="menu-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M3 6H5H21" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M8 6V4C8 3.46957 8.21071 2.96086 8.58579 2.58579C8.96086 2.21071 9.46957 2 10 2H14C14.5304 2 15.0391 2.21071 15.4142 2.58579C15.7893 2.96086 16 3.46957 16 4V6M19 6V20C19 20.5304 18.7893 21.0391 18.4142 21.4142C18.0391 21.7893 17.5304 22 17 22H7C6.46957 22 5.96086 21.7893 5.58579 21.4142C5.21071 21.0391 5 20.5304 5 20V6H19Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <span>删除任务</span>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, nextTick, inject, computed } from 'vue';
import type { Ref } from 'vue';
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
  editTodo: []; // 添加编辑事件
  removeOldCompleted: []; // 移除旧的已完成任务
}>();

// 注入当前时间戳（用于倒计时实时更新）
const currentTimestamp = inject<Ref<number>>('currentTimestamp');

// 格式化时间（精确到秒）
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

// 格式化截止时间（只到分钟）
function formatDeadlineTime(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    hour12: false
  });
}

// 计算已创建天数（响应式）
const daysCreatedComputed = computed(() => {
  if (!props.todo) return 0;
  
  // 如果任务已完成，使用完成时间计算耗时
  if (props.todo.completed && props.todo.completedAt) {
    const completedTime = props.todo.completedAt * 1000;
    const createdTime = props.todo.createdAt * 1000;
    const diffMs = completedTime - createdTime;
    return Math.floor(diffMs / (1000 * 60 * 60 * 24));
  }
  
  // 未完成的任务，使用当前时间计算已创建天数
  const now = currentTimestamp?.value || Date.now();
  const createdTime = props.todo.createdAt * 1000;
  const diffMs = now - createdTime;
  return Math.floor(diffMs / (1000 * 60 * 60 * 24));
});

function calculateDaysCreated(): number {
  return daysCreatedComputed.value;
}

// 获取已完成任务的耗时文本
function getCompletedDaysText(): string {
  const days = calculateDaysCreated();
  return days < 1 ? '当天完成' : `${days}天`;
}

// 计算完成后的天数
function calculateDaysFromCompleted(): number {
  if (!props.todo?.completedAt) return 0;
  const now = Date.now();
  const completedTime = props.todo.completedAt * 1000;
  const diffMs = now - completedTime;
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));
  return diffDays;
}

// 计算距离截止时间（秒）（响应式）
const deadlineDiffSeconds = computed(() => {
  if (!props.todo?.deadline) return 0;
  // 依赖 currentTimestamp 以实现自动更新
  const now = currentTimestamp?.value || Date.now();
  return props.todo.deadline - Math.floor(now / 1000);
});

function calculateDeadlineDiffSeconds(): number {
  return deadlineDiffSeconds.value;
}

// 获取截止时间状态文本
function getDeadlineStatus(): string {
  if (!props.todo?.deadline) return '';
  
  const diffSeconds = calculateDeadlineDiffSeconds();
  
  // 已逾期
  if (diffSeconds < 0) {
    const overdueDiff = Math.abs(diffSeconds);
    if (overdueDiff < 60) {
      return '已逾期（不到1分钟）';
    } else if (overdueDiff < 3600) {
      const minutes = Math.floor(overdueDiff / 60);
      return `已逾期 ${minutes} 分`;
    } else if (overdueDiff < 86400) {
      const hours = Math.floor(overdueDiff / 3600);
      const minutes = Math.floor((overdueDiff % 3600) / 60);
      return minutes > 0 ? `已逾期 ${hours} 时 ${minutes} 分` : `已逾期 ${hours} 时`;
    } else {
      const days = Math.floor(overdueDiff / 86400);
      const hours = Math.floor((overdueDiff % 86400) / 3600);
      return hours > 0 ? `已逾期 ${days} 天 ${hours} 时` : `已逾期 ${days} 天`;
    }
  }
  
  // 未逾期 - 显示剩余时间
  if (diffSeconds < 60) {
    return '剩余不到 1 分';
  } else if (diffSeconds < 3600) {
    const minutes = Math.floor(diffSeconds / 60);
    return `剩余 ${minutes} 分`;
  } else if (diffSeconds < 86400) {
    const hours = Math.floor(diffSeconds / 3600);
    const minutes = Math.floor((diffSeconds % 3600) / 60);
    return minutes > 0 ? `剩余 ${hours} 时 ${minutes} 分` : `剩余 ${hours} 时`;
  } else {
    const days = Math.floor(diffSeconds / 86400);
    const hours = Math.floor((diffSeconds % 86400) / 3600);
    return hours > 0 ? `剩余 ${days} 天 ${hours} 时` : `剩余 ${days} 天`;
  }
}

// 获取截止时间状态样式类
function getDeadlineStatusClass(): string {
  if (!props.todo?.deadline) return '';
  
  const diffSeconds = calculateDeadlineDiffSeconds();
  const daysDiff = Math.floor(diffSeconds / (60 * 60 * 24));
  
  if (diffSeconds < 0) {
    return 'overdue'; // 已逾期
  } else if (daysDiff === 0) {
    return 'urgent'; // 今天到期
  } else if (daysDiff <= 1) {
    return 'urgent'; // 明天到期
  } else if (daysDiff <= 3) {
    return 'warning'; // 3天内到期
  } else {
    return 'normal'; // 正常
  }
}

// 获取时间指示器文本
function getTimeIndicator(): string {
  if (!props.todo) return '';
  
  // 如果有截止时间
  if (props.todo.deadline) {
    const diffSeconds = calculateDeadlineDiffSeconds();
    const daysDiff = Math.floor(diffSeconds / (60 * 60 * 24));
    
    if (diffSeconds < 0) {
      // 已逾期 - 不在这里显示，移到截止时间下面
      return '';
    } else if (daysDiff === 0) {
      return '今天到期';
    } else if (daysDiff === 1) {
      return '明天到期';
    } else if (daysDiff <= 3) {
      return `${daysDiff}天后到期`;
    } else {
      return `距离到期${daysDiff}天`;
    }
  }
  
  // 没有截止时间，显示已创建天数
  const daysCreated = calculateDaysCreated();
  if (daysCreated >= 1) {
    return `已创建${daysCreated}天`;
  }
  
  return '';
}

// 获取时间指示器样式类
function getTimeIndicatorClass(): string {
  if (!props.todo) return '';
  
  if (props.todo.deadline) {
    const diffSeconds = calculateDeadlineDiffSeconds();
    const daysDiff = Math.floor(diffSeconds / (60 * 60 * 24));
    
    if (daysDiff < 0) {
      return 'overdue'; // 逾期
    } else if (daysDiff <= 1) {
      return 'urgent'; // 紧急（今天或明天）
    } else if (daysDiff <= 3) {
      return 'warning'; // 警告（3天内）
    } else {
      return 'normal'; // 正常
    }
  }
  
  return 'created'; // 已创建
}

// 编辑任务
function onEditTodo() {
  emit('editTodo');
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

// 移除旧的已完成任务
function onRemoveOldCompleted() {
  emit('removeOldCompleted');
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
.context-menu {
  position: fixed;
  background: rgba(255, 255, 255, 0.98);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 8px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
  backdrop-filter: blur(20px);
  z-index: 1000;
  min-width: 180px;
  padding: 4px;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  box-sizing: border-box;
}

/* 信息区域 */
.menu-info {
  padding: 4px 6px;
  display: flex;
  flex-direction: column;
  gap: 3px;
}

.info-row {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 0.7rem;
  color: #666;
  line-height: 1.3;
}

.status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  flex-shrink: 0;
}

.status-dot.completed {
  background: #10b981;
  box-shadow: 0 0 0 1.5px rgba(16, 185, 129, 0.2);
}

.status-dot.pending {
  background: #f59e0b;
  box-shadow: 0 0 0 1.5px rgba(245, 158, 11, 0.2);
}

.info-label {
  font-weight: 600;
  color: #888;
  min-width: 28px;
  font-size: 0.68rem;
}

.info-text {
  color: #333;
  font-weight: 500;
  font-size: 0.68rem;
}

.overdue-text {
  color: #dc2626;
  font-weight: 600;
  font-size: 0.68rem;
}

.deadline-status {
  font-weight: 600;
  font-size: 0.68rem;
  padding: 2px 6px;
  border-radius: 4px;
}

.deadline-status.overdue {
  color: #dc2626;
  background: rgba(220, 38, 38, 0.1);
}

.deadline-status.urgent {
  color: #d97706;
  background: rgba(217, 119, 6, 0.1);
}

.deadline-status.warning {
  color: #ca8a04;
  background: rgba(202, 138, 4, 0.1);
}

.deadline-status.normal {
  color: #059669;
  background: rgba(5, 150, 105, 0.1);
}

.time-indicator {
  margin-left: auto;
  font-size: 0.65rem;
  font-weight: 600;
  padding: 2px 6px;
  border-radius: 4px;
  white-space: nowrap;
}

.time-indicator.overdue {
  background: rgba(239, 68, 68, 0.1);
  color: #dc2626;
}

.time-indicator.urgent {
  background: rgba(245, 158, 11, 0.1);
  color: #d97706;
}

.time-indicator.warning {
  background: rgba(251, 191, 36, 0.1);
  color: #ca8a04;
}

.time-indicator.normal {
  background: rgba(16, 185, 129, 0.1);
  color: #059669;
}

.time-indicator.created {
  background: rgba(148, 163, 184, 0.1);
  color: #64748b;
}

.time-indicator.priority-high {
  background: rgba(255, 152, 0, 0.1);
  color: #f57c00;
}

/* 分割线 */
.menu-divider {
  height: 1px;
  background: rgba(0, 0, 0, 0.06);
  margin: 3px 0;
}

/* 操作按钮区域 */
.menu-actions {
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.menu-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 8px;
  background: transparent;
  border: none;
  border-radius: 6px;
  font-size: 0.72rem;
  font-weight: 500;
  color: #333;
  cursor: pointer;
  transition: all 0.15s ease;
  text-align: left;
  width: 100%;
  font-family: inherit;
  line-height: 1.2;
}

.menu-btn:hover {
  background: rgba(0, 0, 0, 0.04);
  color: #000;
}

.menu-btn:active {
  background: rgba(0, 0, 0, 0.08);
  transform: scale(0.98);
}

.menu-icon {
  width: 14px;
  height: 14px;
  flex-shrink: 0;
  color: #666;
  transition: color 0.15s ease;
}

.menu-btn:hover .menu-icon {
  color: #333;
}

/* 删除按钮特殊样式 */
.delete-btn {
  color: #ef4444;
}

.delete-btn:hover {
  background: rgba(239, 68, 68, 0.08);
  color: #dc2626;
}

.delete-btn .menu-icon {
  color: #ef4444;
}

.delete-btn:hover .menu-icon {
  color: #dc2626;
}

/* 夜间主题 */
body.dark-theme .context-menu {
  background: rgba(30, 30, 30, 0.98);
  border-color: rgba(255, 255, 255, 0.1);
}

body.dark-theme .info-row {
  color: #aaa;
}

body.dark-theme .info-label {
  color: #888;
}

body.dark-theme .info-text {
  color: #e0e0e0;
}

body.dark-theme .overdue-text {
  color: #fca5a5;
}

body.dark-theme .menu-divider {
  background: rgba(255, 255, 255, 0.08);
}

body.dark-theme .menu-btn {
  color: #e0e0e0;
}

body.dark-theme .menu-btn:hover {
  background: rgba(255, 255, 255, 0.06);
  color: #fff;
}

body.dark-theme .menu-btn:active {
  background: rgba(255, 255, 255, 0.1);
}

body.dark-theme .menu-icon {
  color: #aaa;
}

body.dark-theme .menu-btn:hover .menu-icon {
  color: #e0e0e0;
}

body.dark-theme .delete-btn {
  color: #f87171;
}

body.dark-theme .delete-btn:hover {
  background: rgba(248, 113, 113, 0.12);
  color: #fca5a5;
}

body.dark-theme .delete-btn .menu-icon {
  color: #f87171;
}

body.dark-theme .delete-btn:hover .menu-icon {
  color: #fca5a5;
}

body.dark-theme .time-indicator.overdue {
  background: rgba(248, 113, 113, 0.15);
  color: #fca5a5;
}

body.dark-theme .time-indicator.urgent {
  background: rgba(251, 191, 36, 0.15);
  color: #fbbf24;
}

body.dark-theme .time-indicator.warning {
  background: rgba(253, 224, 71, 0.15);
  color: #fde047;
}

body.dark-theme .time-indicator.normal {
  background: rgba(52, 211, 153, 0.15);
  color: #6ee7b7;
}

body.dark-theme .time-indicator.created {
  background: rgba(148, 163, 184, 0.15);
  color: #94a3b8;
}

body.dark-theme .time-indicator.priority-high {
  background: rgba(255, 152, 0, 0.15);
  color: #ffb74d;
}

body.dark-theme .deadline-status.overdue {
  color: #fca5a5;
  background: rgba(252, 165, 165, 0.15);
}

body.dark-theme .deadline-status.urgent {
  color: #fbbf24;
  background: rgba(251, 191, 36, 0.15);
}

body.dark-theme .deadline-status.warning {
  color: #fde047;
  background: rgba(253, 224, 71, 0.15);
}

body.dark-theme .deadline-status.normal {
  color: #6ee7b7;
  background: rgba(110, 231, 183, 0.15);
}
</style>