<template>
  <div class="timeline-view">
    <div class="timeline-container">
      <!-- 时间轴线 -->
      <div class="timeline-line"></div>
      
      <!-- 时间节点 -->
      <div 
        v-for="(item, index) in timelineItems" 
        :key="item.todo.id"
        class="timeline-item"
        :class="{ 
          'completed': item.todo.completed,
          'has-deadline': item.todo.deadline,
          'overdue': isOverdue(item.todo),
          'priority': item.todo.priority === 1
        }"
        :style="{ '--day-color': item.dayColor, '--priority-color': priorityColor }"
        @contextmenu="(e) => emit('contextmenu', e, item.todo)"
        @dblclick="emit('togglePriority', item.todo)"
      >
        <!-- 彩虹连接线 -->
        <div class="timeline-connector" :style="{ backgroundColor: item.dayColor }"></div>
        
        <!-- 时间点 -->
        <div class="timeline-dot" :style="{ backgroundColor: getDotColor(item.todo), borderColor: item.dayColor }">
          <svg v-if="item.todo.completed" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M5 13l4 4L19 7" stroke="white" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </div>
        
        <!-- 时间标签 -->
        <div class="timeline-date">
          <div class="date-text">{{ formatDate(item.displayTime) }}</div>
          <div class="time-text">{{ formatTime(item.displayTime) }}</div>
        </div>
        
        <!-- 任务内容 -->
        <div class="timeline-content">
          <div class="task-header">
            <span class="task-text" :class="{ 'completed-text': item.todo.completed }">
              {{ item.todo.text }}
            </span>
            <div class="task-actions">
              <Tooltip :text="item.todo.completed ? '标记为未完成' : '标记为完成'">
                <button 
                  class="action-btn toggle-btn" 
                  @click="emit('toggle', item.todo)"
                >
                  <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <circle cx="12" cy="12" r="10" :stroke="item.todo.completed ? '#4CAF50' : 'currentColor'" 
                      :fill="item.todo.completed ? '#4CAF50' : 'none'" stroke-width="2"/>
                    <path v-if="item.todo.completed" d="M7 12L10.5 15.5L17 9" stroke="white" 
                      stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>
                  </svg>
                </button>
              </Tooltip>
            </div>
          </div>
          
          <div class="task-meta">
            <span class="group-badge">{{ getGroupName(item.todo.groupId) }}</span>
            <span v-if="item.todo.priority === 1" class="priority-badge">优先</span>
            <span v-if="item.todo.deadline" class="deadline-badge">截止</span>
            <span v-if="item.todo.deadline" class="meta-item" :class="{ 'overdue-text': isOverdue(item.todo) }">
              <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2"/>
                <path d="M12 6v6l4 2" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
              </svg>
              {{ formatRemainingTime(item.todo.deadline) }}
            </span>
          </div>
        </div>
      </div>
      
      <!-- 空状态 -->
      <div v-if="timelineItems.length === 0" class="empty-timeline">
        <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" 
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <p>暂无任务</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, inject } from 'vue';
import type { Ref } from 'vue';
import type { Todo, TodoGroup } from '../types';
import Tooltip from './Tooltip.vue';

// 注入当前时间戳（用于倒计时实时更新）
const currentTimestamp = inject<Ref<number>>('currentTimestamp');

interface Props {
  todos: Todo[];
  groups: TodoGroup[];
  priorityColor?: string;
  deadlinePriority?: boolean; // 是否优先使用截止时间
}

interface TimelineItem {
  todo: Todo;
  displayTime: number; // 用于排序和显示的时间（截止时间或创建时间）
  dayColor: string; // 当天的彩虹色
}

const props = withDefaults(defineProps<Props>(), {
  deadlinePriority: true
});

const emit = defineEmits<{
  toggle: [todo: Todo];
  delete: [todo: Todo];
  contextmenu: [event: MouseEvent, todo: Todo];
  togglePriority: [todo: Todo];
}>();

// 彩虹色板 - 柔和的颜色
const rainbowColors = [
  '#FF6B9D', // 粉红
  '#FFA07A', // 浅橙
  '#FFD93D', // 金黄
  '#6BCB77', // 薄荷绿
  '#4D96FF', // 天蓝
  '#9D84B7', // 淡紫
  '#FF8C94', // 珊瑚红
];

// 生成时间轴项目
const timelineItems = computed<TimelineItem[]>(() => {
  const sorted = props.todos
    .filter(todo => !todo.completed) // 只显示未完成的任务
    .map(todo => ({
      todo,
      // 根据设置决定使用截止时间还是创建时间
      displayTime: props.deadlinePriority && todo.deadline ? todo.deadline : todo.createdAt,
      dayColor: '' // 稍后填充
    }))
    .sort((a, b) => a.displayTime - b.displayTime); // 从旧到新排序（正序）
  
  // 为每一天分配颜色
  let lastDate = '';
  let colorIndex = 0;
  let lastColorIndex = -1;
  
  sorted.forEach((item) => {
    const date = new Date(item.displayTime * 1000);
    const dateKey = `${date.getFullYear()}-${date.getMonth()}-${date.getDate()}`;
    
    if (dateKey !== lastDate) {
      // 新的一天，选择不同于上一个的颜色
      do {
        colorIndex = (colorIndex + 1) % rainbowColors.length;
      } while (colorIndex === lastColorIndex && rainbowColors.length > 1);
      
      lastColorIndex = colorIndex;
      lastDate = dateKey;
    }
    
    item.dayColor = rainbowColors[colorIndex];
  });
  
  return sorted;
});

// 获取分组名称
function getGroupName(groupId: string): string {
  const group = props.groups.find(g => g.id === groupId);
  return group?.name || '未分组';
}

// 判断是否过期（响应式）
function isOverdue(todo: Todo): boolean {
  if (!todo.deadline || todo.completed) return false;
  // 依赖 currentTimestamp 以实现自动更新
  const now = currentTimestamp?.value || Date.now();
  return todo.deadline < Math.floor(now / 1000);
}

// 获取圆点颜色
function getDotColor(todo: Todo): string {
  if (todo.completed) return '#4CAF50';
  if (isOverdue(todo)) return '#f44336';
  if (todo.priority === 1) return props.priorityColor || '#FF9800';
  return '#3b82f6';
}

// 格式化日期
function formatDate(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  const now = new Date();
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
  const targetDate = new Date(date.getFullYear(), date.getMonth(), date.getDate());
  
  const diffDays = Math.floor((targetDate.getTime() - today.getTime()) / (1000 * 60 * 60 * 24));
  
  if (diffDays === 0) return '今天';
  if (diffDays === 1) return '明天';
  if (diffDays === -1) return '昨天';
  
  const month = date.getMonth() + 1;
  const day = date.getDate();
  return `${month}月${day}日`;
}

// 格式化时间
function formatTime(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  const hours = date.getHours().toString().padStart(2, '0');
  const minutes = date.getMinutes().toString().padStart(2, '0');
  return `${hours}:${minutes}`;
}

// 格式化完整日期时间
function formatDateTime(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  const month = date.getMonth() + 1;
  const day = date.getDate();
  const hours = date.getHours().toString().padStart(2, '0');
  const minutes = date.getMinutes().toString().padStart(2, '0');
  return `${month}月${day}日 ${hours}:${minutes}`;
}

// 格式化剩余时间（响应式）
function formatRemainingTime(deadline: number): string {
  // 依赖 currentTimestamp 以实现自动更新
  const now = currentTimestamp?.value || Date.now();
  const diff = deadline - Math.floor(now / 1000);
  
  // 如果已过期
  if (diff < 0) {
    const absDiff = Math.abs(diff);
    const days = Math.floor(absDiff / (24 * 60 * 60));
    const hours = Math.floor((absDiff % (24 * 60 * 60)) / (60 * 60));
    const minutes = Math.floor((absDiff % (60 * 60)) / 60);
    
    let result = '已逾期';
    if (days > 0) result += ` ${days}天`;
    if (hours > 0) result += ` ${hours}时`;
    if (minutes > 0 && days === 0) result += ` ${minutes}分`;
    return result;
  }
  
  // 计算剩余时间
  const days = Math.floor(diff / (24 * 60 * 60));
  const hours = Math.floor((diff % (24 * 60 * 60)) / (60 * 60));
  const minutes = Math.floor((diff % (60 * 60)) / 60);
  
  let result = '剩余';
  if (days > 0) result += ` ${days}天`;
  if (hours > 0) result += ` ${hours}时`;
  if (minutes > 0 && days === 0) result += ` ${minutes}分`;
  
  // 如果都是0，显示"即将到期"
  if (days === 0 && hours === 0 && minutes === 0) {
    result = '即将到期';
  }
  
  return result;
}
</script>

<style scoped>
.timeline-view {
  width: 100%;
  height: 100%;
  overflow-y: auto;
  overflow-x: hidden;
  padding: clamp(8px, 1.5vh, 10px) clamp(8px, 1.5vw, 12px);
}

/* 隐藏滚动条 */
.timeline-view::-webkit-scrollbar {
  display: none;
}

.timeline-view {
  -ms-overflow-style: none;  /* IE and Edge */
  scrollbar-width: none;  /* Firefox */
}

.timeline-container {
  position: relative;
  max-width: 700px;
  margin: 0 auto;
  padding-right: clamp(70px, 14vw, 90px);
}

.timeline-line {
  position: absolute;
  right: clamp(70px, 14vw, 90px);
  top: 0;
  bottom: 0;
  width: 0;
  display: none;
}

.timeline-item {
  position: relative;
  margin-bottom: clamp(12px, 2vh, 16px);
  animation: fadeIn 0.3s ease;
}

/* 彩虹连接线 */
.timeline-connector {
  position: absolute;
  right: clamp(-18px, -3.6vw, -22px);
  top: 0;
  bottom: -12px;
  width: 4px;
  opacity: 0.8;
  z-index: 1;
  border-radius: 2px;
}

.timeline-item:last-child .timeline-connector {
  bottom: 50%;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.timeline-dot {
  position: absolute;
  right: clamp(-20px, -4vw, -24px);
  top: clamp(3px, 0.6vh, 4px);
  width: clamp(14px, 2.8vw, 16px);
  height: clamp(14px, 2.8vw, 16px);
  border-radius: 50%;
  border: 3px solid white;
  background-color: var(--day-color);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2;
  transition: all 0.3s ease;
  box-shadow: 0 0 0 3px rgba(255, 255, 255, 0.1);
}

.timeline-dot svg {
  width: 70%;
  height: 70%;
}

.timeline-item:hover .timeline-dot {
  transform: scale(1.15);
  box-shadow: 0 0 0 5px var(--day-color);
  filter: brightness(1.1);
}

.timeline-item:hover .timeline-connector {
  opacity: 1;
  width: 5px;
}

.timeline-date {
  position: absolute;
  right: clamp(-70px, -14vw, -90px);
  top: 0;
  text-align: left;
  min-width: clamp(50px, 10vw, 65px);
  padding-left: clamp(8px, 1.6vw, 10px);
}

.date-text {
  font-size: clamp(10px, 2vw, 12px);
  font-weight: 600;
  color: var(--day-color);
  margin-bottom: 1px;
  line-height: 1.2;
}

.time-text {
  font-size: clamp(9px, 1.8vw, 11px);
  color: rgba(0, 0, 0, 0.5);
  line-height: 1.2;
}

.task-meta .priority-badge,
.task-meta .deadline-badge {
  display: inline-block;
  padding: clamp(1px, 0.2vh, 2px) clamp(5px, 1vw, 6px);
  color: white;
  border-radius: 3px;
  font-size: clamp(9px, 1.8vw, 10px);
  font-weight: 600;
}

.task-meta .priority-badge {
  background: var(--priority-color, #FF9800);
}

.task-meta .deadline-badge {
  background: #f44336;
}

.timeline-content {
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  border-radius: clamp(6px, 1.2vw, 8px);
  padding: clamp(8px, 1.6vh, 10px) clamp(10px, 2vw, 12px);
  border: 1px solid rgba(0, 0, 0, 0.08);
  transition: all 0.3s ease;
  cursor: pointer;
}

.timeline-item:hover .timeline-content {
  background: rgba(255, 255, 255, 0.95);
  border-color: rgba(59, 130, 246, 0.3);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  transform: translateX(-3px);
}

.timeline-item.completed .timeline-content {
  opacity: 0.7;
}

.timeline-item.overdue .timeline-content {
  border-color: rgba(244, 67, 54, 0.3);
}

.task-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: clamp(6px, 1.2vw, 8px);
  margin-bottom: clamp(4px, 0.8vh, 6px);
}

.task-text {
  flex: 1;
  font-size: clamp(11px, 2.2vw, 13px);
  font-weight: 500;
  color: rgba(0, 0, 0, 0.85);
  word-break: break-word;
  line-height: 1.4;
}

.completed-text {
  text-decoration: line-through;
  color: rgba(0, 0, 0, 0.4);
}

.task-actions {
  display: flex;
  gap: clamp(3px, 0.6vw, 4px);
  opacity: 0;
  transition: opacity 0.2s ease;
}

.timeline-item:hover .task-actions {
  opacity: 1;
}

.action-btn {
  width: clamp(18px, 3.6vw, 20px);
  height: clamp(18px, 3.6vw, 20px);
  border: none;
  background: transparent;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 3px;
  transition: all 0.2s ease;
  padding: 0;
}

.action-btn svg {
  width: 100%;
  height: 100%;
  color: rgba(0, 0, 0, 0.5);
}

.action-btn:hover {
  background: rgba(0, 0, 0, 0.05);
}

.delete-btn:hover svg {
  color: #f44336;
}

.task-meta {
  display: flex;
  flex-wrap: wrap;
  gap: clamp(6px, 1.2vw, 8px);
  align-items: center;
}

.group-badge {
  display: inline-block;
  padding: clamp(1px, 0.2vh, 2px) clamp(5px, 1vw, 6px);
  background: rgba(59, 130, 246, 0.1);
  color: #3b82f6;
  border-radius: 3px;
  font-size: clamp(9px, 1.8vw, 10px);
  font-weight: 600;
}

.meta-item {
  display: flex;
  align-items: center;
  gap: 3px;
  font-size: clamp(9px, 1.8vw, 10px);
  color: rgba(0, 0, 0, 0.7);
  line-height: 1.2;
  font-weight: 500;
}

.meta-item svg {
  width: clamp(11px, 2.2vw, 12px);
  height: clamp(11px, 2.2vw, 12px);
  flex-shrink: 0;
}

.meta-item.overdue-text {
  color: #f44336;
  font-weight: 600;
}

.empty-timeline {
  text-align: center;
  padding: clamp(40px, 8vh, 60px) clamp(16px, 3vw, 20px);
  color: rgba(0, 0, 0, 0.3);
}

.empty-timeline svg {
  width: clamp(48px, 10vw, 64px);
  height: clamp(48px, 10vw, 64px);
  margin-bottom: clamp(12px, 2.5vh, 16px);
  opacity: 0.3;
}

.empty-timeline p {
  font-size: clamp(12px, 2.5vw, 14px);
  margin: 0;
}

/* 夜间主题 */
body.dark-theme .timeline-line {
  background: linear-gradient(to bottom, 
    rgba(59, 130, 246, 0.4) 0%,
    rgba(59, 130, 246, 0.15) 100%);
}

body.dark-theme .timeline-dot {
  border-color: var(--day-color);
  box-shadow: 0 0 0 4px rgba(0, 0, 0, 0.2);
}

body.dark-theme .timeline-item:hover .timeline-dot {
  box-shadow: 0 0 0 6px var(--day-color);
}

body.dark-theme .date-text {
  color: var(--day-color);
  filter: brightness(1.2);
}

body.dark-theme .time-text {
  color: rgba(255, 255, 255, 0.5);
}

body.dark-theme .timeline-content {
  background: rgba(0, 0, 0, 0.3);
  border-color: rgba(255, 255, 255, 0.1);
}

body.dark-theme .timeline-item:hover .timeline-content {
  background: rgba(0, 0, 0, 0.4);
  border-color: rgba(59, 130, 246, 0.4);
}

body.dark-theme .task-text {
  color: rgba(255, 255, 255, 0.9);
}

body.dark-theme .completed-text {
  color: rgba(255, 255, 255, 0.4);
}

body.dark-theme .action-btn svg {
  color: rgba(255, 255, 255, 0.6);
}

body.dark-theme .action-btn:hover {
  background: rgba(255, 255, 255, 0.1);
}

body.dark-theme .meta-item {
  color: rgba(255, 255, 255, 0.7);
}

body.dark-theme .meta-item.overdue-text {
  color: #ff6b6b;
}

body.dark-theme .empty-timeline {
  color: rgba(255, 255, 255, 0.3);
}
</style>
