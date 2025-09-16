<template>
  <div class="calendar-container">
    <header class="calendar-header">
      <h2>全年日历</h2>
      <button class="close-btn" @click="closeWindow">✕</button>
    </header>
    <div class="calendar-grid">
      <div v-for="(row, rowIndex) in calendarRows" :key="rowIndex" class="calendar-row">
        <div v-for="(month, colIndex) in row" :key="colIndex" class="month-container">
          <div class="month-header">{{ getMonthNameWithYear(month) }}</div>
          <div class="month-grid">
            <div class="weekdays">
              <div class="weekday">日</div>
              <div class="weekday">一</div>
              <div class="weekday">二</div>
              <div class="weekday">三</div>
              <div class="weekday">四</div>
              <div class="weekday">五</div>
              <div class="weekday">六</div>
            </div>
            <div class="days-grid">
              <!-- 填充月初的空白天 -->
              <div 
                v-for="blank in getFirstDayOfMonth(month)" 
                :key="`blank-${blank}`" 
                class="day empty"
              ></div>
              <!-- 渲染月份的每一天 -->
              <div 
                v-for="day in getDaysInMonth(month)" 
                :key="day" 
                class="day"
                :class="getDayClass(month, day)"
                @click="onDayClick(month, day)"
              >
                <span class="day-number">{{ day }}</span>
                <div v-if="hasTodos(month, day)" class="todo-indicator">
                  <div 
                    v-for="(color, index) in getTodoColors(month, day)" 
                    :key="index"
                    class="color-segment"
                    :style="{ backgroundColor: color }"
                  ></div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import type { Todo } from '../types';

const todos = ref<Todo[]>([]);
// 添加定时器引用
const refreshTimer = ref<number | null>(null);

// 计算日历行，每行3个月，共4行
const calendarRows = computed(() => {
  const rows = [];
  const currentYear = new Date().getFullYear();
  
  for (let row = 0; row < 4; row++) {
    const months = [];
    for (let col = 0; col < 3; col++) {
      const month = row * 3 + col; // 0-11 月份
      months.push(new Date(currentYear, month, 1));
    }
    rows.push(months);
  }
  
  return rows;
});

// 获取月份名称（带年份）
function getMonthNameWithYear(date: Date): string {
  const months = [
    '一月', '二月', '三月', '四月', '五月', '六月',
    '七月', '八月', '九月', '十月', '十一月', '十二月'
  ];
  return `${date.getFullYear()}年${months[date.getMonth()]}`;
}

// 获取月份的天数
function getDaysInMonth(date: Date): number {
  return new Date(date.getFullYear(), date.getMonth() + 1, 0).getDate();
}

// 获取月份第一天是星期几 (0=周日, 6=周六)
function getFirstDayOfMonth(date: Date): number {
  return new Date(date.getFullYear(), date.getMonth(), 1).getDay();
}

// 获取某天的待办事项颜色
function getTodoColors(month: Date, day: number): string[] {
  const date = new Date(month.getFullYear(), month.getMonth(), day);
  const dateString = date.toISOString().split('T')[0]; // YYYY-MM-DD 格式
  
  // 查找匹配的待办事项
  const matchingTodos = todos.value.filter(todo => {
    if (todo.deadline) {
      const deadlineDate = new Date(todo.deadline * 1000);
      const deadlineString = deadlineDate.toISOString().split('T')[0];
      return deadlineString === dateString;
    }
    return false;
  });
  
  // 根据待办事项状态确定颜色
  const colors: string[] = [];
  matchingTodos.forEach(todo => {
    if (todo.completed) {
      colors.push('#4CAF50'); // 绿色 - 已完成
    } else {
      // 检查是否接近截止日期
      const deadlineDate = new Date(todo.deadline! * 1000);
      const now = new Date();
      const diffTime = deadlineDate.getTime() - now.getTime();
      const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));
      
      if (diffDays <= 0) {
        colors.push('#F44336'); // 红色 - 已过期
      } else if (diffDays <= 3) {
        colors.push('#FF9800'); // 橙色 - 即将到期
      } else {
        colors.push('#2196F3'); // 蓝色 - 正常
      }
    }
  });
  
  // 去重并限制颜色数量
  const uniqueColors = [...new Set(colors)];
  
  // 如果有多种颜色，进行分割显示
  if (uniqueColors.length > 1) {
    return splitColors(uniqueColors);
  }
  
  return uniqueColors;
}

// 分割颜色显示
function splitColors(colors: string[]): string[] {
  if (colors.length === 2) {
    // 两种颜色，各占一半
    return [colors[0], colors[1]];
  } else if (colors.length >= 3) {
    // 三种或更多颜色，取前三种并三叉分割
    return colors.slice(0, 3);
  }
  return colors;
}

// 检查某天是否有待办事项
function hasTodos(month: Date, day: number): boolean {
  const date = new Date(month.getFullYear(), month.getMonth(), day);
  const dateString = date.toISOString().split('T')[0]; // YYYY-MM-DD 格式
  
  return todos.value.some(todo => {
    if (todo.deadline) {
      const deadlineDate = new Date(todo.deadline * 1000);
      const deadlineString = deadlineDate.toISOString().split('T')[0];
      return deadlineString === dateString;
    }
    return false;
  });
}

// 获取日期的CSS类
function getDayClass(month: Date, day: number): string[] {
  const classes = [];
  const today = new Date();
  
  // 检查是否是今天
  if (
    month.getFullYear() === today.getFullYear() &&
    month.getMonth() === today.getMonth() &&
    day === today.getDate()
  ) {
    classes.push('today');
  }
  
  // 检查是否有待办事项
  if (hasTodos(month, day)) {
    classes.push('has-todos');
  }
  
  return classes;
}

// 点击日期事件
function onDayClick(month: Date, day: number) {
  console.log(`点击了 ${month.getFullYear()}年${month.getMonth() + 1}月${day}日`);
  // 这里可以添加点击日期后的处理逻辑
}

// 关闭窗口
async function closeWindow() {
  const currentWindow = getCurrentWindow();
  await currentWindow.close();
}

// 从本地文件加载数据
async function loadTodoData() {
  try {
    const data = await invoke('load_todo_data') as {
      pending_todos: { id?: string; text: string; completed: boolean; created_at: number; deadline?: number }[],
      completed_todos: { id?: string; text: string; completed: boolean; created_at: number; deadline?: number }[]
    };
    // 转换数据格式（后端使用下划线命名，前端使用驼峰命名）
    const pendingTodos = data.pending_todos.map(todo => ({
      id: todo.id || generateUniqueId(), // 如果没有ID则生成新的
      text: todo.text,
      completed: todo.completed,
      createdAt: todo.created_at,
      deadline: todo.deadline // 处理可选的deadline字段
    }));
    const completedTodos = data.completed_todos.map(todo => ({
      id: todo.id || generateUniqueId(), // 如果没有ID则生成新的
      text: todo.text,
      completed: todo.completed,
      createdAt: todo.created_at,
      deadline: todo.deadline // 处理可选的deadline字段
    }));
    
    todos.value = [...pendingTodos, ...completedTodos];
    console.log('待办数据加载成功');
  } catch (error) {
    console.error('加载待办数据失败:', error);
    todos.value = [];
  }
}

// 生成唯一ID的函数
function generateUniqueId(): string {
  // 生成真正的UUID v4
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
    const r = Math.random() * 16 | 0;
    const v = c == 'x' ? r : (r & 0x3 | 0x8);
    return v.toString(16);
  });
}

// 启动定时刷新
function startRefreshTimer() {
  if (refreshTimer.value) {
    clearInterval(refreshTimer.value);
  }
  
  // 每分钟更新一次
  refreshTimer.value = window.setInterval(() => {
    // 重新加载数据以更新颜色计算
    loadTodoData();
  }, 60000); // 60秒 = 1分钟
}

// 组件挂载时加载数据
onMounted(async () => {
  await loadTodoData();
  // 启动定时刷新
  startRefreshTimer();
});

// 组件卸载时清理定时器
onUnmounted(() => {
  if (refreshTimer.value) {
    clearInterval(refreshTimer.value);
    refreshTimer.value = null;
  }
});
</script>

<style scoped>
.calendar-container {
  width: 100%;
  height: 100%;
  background: #ffffff;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border-radius: 0;
}

.calendar-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  background: rgba(255, 255, 255, 0.6);
  border-bottom: 1px solid rgba(229, 231, 235, 0.2);
  color: #333;
  font-weight: 600;
  backdrop-filter: blur(10px);
  flex-shrink: 0;
}

.calendar-header h2 {
  margin: 0;
  font-size: 1.2rem;
}

.close-btn {
  width: 30px;
  height: 30px;
  border: none;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.8);
  color: #333;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1rem;
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  flex-shrink: 0;
  backdrop-filter: blur(5px);
  border: 1px solid rgba(229, 231, 235, 0.2);
}

.close-btn:hover {
  transform: scale(1.1);
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.15);
  background: rgba(255, 255, 255, 0.95);
}

.calendar-grid {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 20px;
  overflow-y: auto;
}

.calendar-row {
  display: flex;
  gap: 20px;
  margin-bottom: 20px;
}

.calendar-row:last-child {
  margin-bottom: 0;
}

.month-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: rgba(255, 255, 255, 0.7);
  border-radius: 12px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
  border: 1px solid rgba(229, 231, 235, 0.2);
  backdrop-filter: blur(10px);
  min-width: 0;
}

.month-header {
  padding: 12px;
  text-align: center;
  font-weight: 600;
  font-size: 1.1rem;
  border-bottom: 1px solid rgba(229, 231, 235, 0.2);
  background: rgba(255, 255, 255, 0.5);
  flex-shrink: 0;
}

.month-grid {
  padding: 10px;
  flex: 1;
  display: flex;
  flex-direction: column;
}

.weekdays {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  text-align: center;
  font-weight: 600;
  font-size: 0.8rem;
  margin-bottom: 5px;
  color: #555;
  flex-shrink: 0;
}

.weekday {
  padding: 5px 0;
}

.days-grid {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  grid-auto-rows: 30px;
  grid-gap: 2px;
  flex: 1;
}

.day {
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
  background: rgba(255, 255, 255, 0.3);
  border: 1px solid rgba(229, 231, 235, 0.2);
  overflow: hidden;
  box-sizing: border-box;
}

.day:hover {
  background: rgba(255, 255, 255, 0.6);
  transform: scale(1.05);
}

.day.empty {
  background: transparent;
  cursor: default;
  border: none;
}

.day.today {
  background: rgba(33, 150, 243, 0.2);
  border: 1px solid #2196F3;
  font-weight: bold;
}

.day.has-todos {
  background: rgba(255, 255, 255, 0.5);
}

.day-number {
  font-size: 0.9rem;
  z-index: 1;
  text-align: center;
}

.todo-indicator {
  position: absolute;
  bottom: 2px;
  width: 80%;
  height: 4px;
  display: flex;
  border-radius: 2px;
  overflow: hidden;
}

.color-segment {
  flex: 1;
  height: 100%;
}

/* 暗色主题样式 */
body.dark-theme .calendar-container {
  background: #181a1b;
  color: #e7e9ed;
}

body.dark-theme .calendar-header {
  background: rgba(24, 26, 27, 0.8);
  border-bottom: 1px solid rgba(231, 233, 237, 0.3);
  color: #e7e9ed;
}

body.dark-theme .close-btn {
  background: rgba(24, 26, 27, 0.9);
  color: #e7e9ed;
  border: 1px solid rgba(231, 233, 237, 0.2);
}

body.dark-theme .close-btn:hover {
  background: rgba(37, 38, 39, 0.95);
}

body.dark-theme .month-container {
  background: #252627;
  border: 1px solid rgba(231, 233, 237, 0.3);
  color: #e7e9ed;
}

body.dark-theme .month-header {
  background: rgba(37, 38, 39, 0.5);
  border-bottom: 1px solid rgba(231, 233, 237, 0.3);
  color: #e7e9ed;
}

body.dark-theme .weekdays {
  color: #a0a6aa;
}

body.dark-theme .day {
  background: rgba(37, 38, 39, 0.3);
  border: 1px solid rgba(231, 233, 237, 0.2);
  color: #e7e9ed;
}

body.dark-theme .day:hover {
  background: rgba(37, 38, 39, 0.6);
}

body.dark-theme .day.empty {
  background: transparent;
}

body.dark-theme .day.today {
  background: rgba(33, 150, 243, 0.3);
  border: 1px solid #2196F3;
}
</style>