<template>
  <div class="container">
    <header data-tauri-drag-region>
      <div class="header-title" data-tauri-drag-region>📋 Todo 桌面助手</div>
      <div class="header-right">
        <div class="progress-indicator">{{ completedTasks }}/{{ totalTasks }}</div>
        <button class="settings-btn" @click="openSettings">⚙️</button>
      </div>
    </header>
    <div class="todo-list">
      <div v-for="(todo, index) in todos" :key="index" :class="['todo-item', { completed: todo.completed }]">
        <div :class="['todo-checkbox', { completed: todo.completed }]" @click="toggleTodo(index)"></div>
        <span>{{ todo.text }}</span>
      </div>
    </div>
    <div class="add-task">
      <input type="text" placeholder="添加新任务..." v-model="newTaskText" @keypress.enter="addTask">
      <button @click="addTask">➕</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, nextTick } from 'vue';

interface Todo {
  text: string;
  completed: boolean;
}

const todos = ref<Todo[]>([
  { text: '完成UI设计', completed: true },
  { text: '学习SpringBoot3.5', completed: false },
  { text: '测试部署到服务器', completed: false }
]);

const newTaskText = ref('');
const totalTasks = computed(() => todos.value.length);
const completedTasks = computed(() => todos.value.filter(todo => todo.completed).length);

function openSettings() {
  alert('设置功能开发中...🚀 可添加的功能： • 主题切换 • 任务分类 • 导入/导出 • 提醒设置');
}

function addTask() {
  const taskText = newTaskText.value.trim();
  if (taskText) {
    todos.value.push({
      text: taskText,
      completed: false
    });
    newTaskText.value = '';
  }
}

function toggleTodo(index: number) {
  todos.value[index].completed = !todos.value[index].completed;
}

// 简化onMounted钩子，减少不必要的延迟
onMounted(() => {
  console.log('前端渲染完成');
});
</script>

<style>
* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}
html, body, #app {
  width: 100%;
  height: 100%;
  overflow: hidden;
}
body {
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  background: #e8e4f3;
  color: #333;
}
#app {
  display: flex;
  justify-content: center;
  align-items: center;
}
.container {
  width: 100%;
  height: 100%;
  background: #ffffff;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border-radius: 0;
}
header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: clamp(6px, 1.2vh, 8px) clamp(10px, 2.5vw, 14px);
  font-size: clamp(0.85rem, 2.2vw, 0.95rem);
  background: rgba(255, 255, 255, 0.6);
  border-bottom: 1px solid rgba(104, 58, 183, 0.1);
  color: #333;
  font-weight: 600;
  backdrop-filter: blur(10px);
  min-height: clamp(30px, 5vh, 36px);
}
.header-title {
  flex: 1;
  text-align: center;
}
.header-right {
  display: flex;
  align-items: center;
  gap: clamp(6px, 1.5vw, 10px);
}
.progress-indicator {
  font-size: clamp(0.7rem, 1.8vw, 0.8rem);
  color: #8e7cc3;
  font-weight: 600;
  background: rgba(142, 124, 195, 0.1);
  padding: clamp(2px, 0.5vh, 4px) clamp(6px, 1.5vw, 8px);
  border-radius: clamp(8px, 1.5vw, 12px);
  border: 1px solid rgba(142, 124, 195, 0.2);
  min-width: clamp(28px, 6vw, 35px);
  text-align: center;
  backdrop-filter: blur(5px);
}
.settings-btn {
  width: clamp(24px, 4.5vw, 28px);
  height: clamp(24px, 4.5vw, 28px);
  border: none;
  border-radius: 50%;
  background: linear-gradient(135deg, #8e7cc3, #6a4c93);
  color: #fff;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: clamp(0.65rem, 1.8vw, 0.8rem);
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(104, 58, 183, 0.3);
  flex-shrink: 0;
}
.settings-btn:hover {
  transform: rotate(90deg) scale(1.08);
  box-shadow: 0 6px 16px rgba(104, 58, 183, 0.4);
}
.settings-btn:hover {
  transform: rotate(90deg) scale(1.1);
  box-shadow: 0 0 15px rgba(255, 0, 255, 0.6);
}
.todo-list {
  flex: 1;
  padding: clamp(8px, 2vh, 12px);
  overflow-y: auto;
  background: transparent;
  min-height: 0;
}
.todo-list::-webkit-scrollbar {
  width: 5px;
}
.todo-list::-webkit-scrollbar-track {
  background: rgba(104, 58, 183, 0.1);
  border-radius: 3px;
}
.todo-list::-webkit-scrollbar-thumb {
  background: linear-gradient(180deg, #8e7cc3, #6a4c93);
  border-radius: 3px;
}
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
}
.todo-item:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(104, 58, 183, 0.15);
  border-color: rgba(104, 58, 183, 0.2);
}
.todo-checkbox {
  width: clamp(14px, 2.5vw, 18px);
  height: clamp(14px, 2.5vw, 18px);
  border: 2px solid #8e7cc3;
  border-radius: 50%;
  margin-right: clamp(6px, 1.5vw, 10px);
  cursor: pointer;
  transition: all 0.3s ease;
  flex-shrink: 0;
  position: relative;
}
.todo-checkbox.completed {
  background: linear-gradient(135deg, #8e7cc3, #6a4c93);
  border-color: #6a4c93;
}
.todo-checkbox.completed::after {
  content: '✓';
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  color: white;
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
}
.todo-item.completed span {
  text-decoration: line-through;
  color: #888;
}
.add-task {
  display: flex;
  padding: clamp(6px, 1.5vh, 10px);
  border-top: 1px solid rgba(104, 58, 183, 0.1);
  background: rgba(255, 255, 255, 0.6);
  gap: clamp(6px, 1.2vw, 10px);
  backdrop-filter: blur(10px);
  min-height: clamp(34px, 6vh, 42px);
  align-items: center;
}
.add-task input {
  flex: 1;
  padding: clamp(6px, 1.5vh, 8px) clamp(8px, 2vw, 10px);
  border: 1px solid rgba(104, 58, 183, 0.2);
  border-radius: clamp(8px, 1.5vw, 12px);
  outline: none;
  background: rgba(255, 255, 255, 0.8);
  color: #333;
  font-size: clamp(0.75rem, 2vw, 0.85rem);
  transition: all 0.3s ease;
  height: clamp(28px, 4.5vh, 32px);
}
.add-task input:focus {
  border-color: #8e7cc3;
  box-shadow: 0 0 8px rgba(104, 58, 183, 0.2);
  background: rgba(255, 255, 255, 0.95);
}
.add-task input::placeholder {
  color: rgba(51, 51, 51, 0.5);
}
.add-task button {
  width: clamp(28px, 4.5vh, 32px);
  height: clamp(28px, 4.5vh, 32px);
  background: linear-gradient(135deg, #8e7cc3, #6a4c93);
  border: none;
  border-radius: 50%;
  cursor: pointer;
  color: #fff;
  font-weight: 700;
  box-shadow: 0 4px 12px rgba(104, 58, 183, 0.3);
  transition: all 0.3s ease;
  font-size: clamp(0.7rem, 2vw, 0.85rem);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}
.add-task button:hover {
  transform: translateY(-1px);
  box-shadow: 0 6px 16px rgba(104, 58, 183, 0.4);
}
.add-task button:active {
  transform: translateY(0);
}
.add-task button:hover {
  background: linear-gradient(135deg, magenta, cyan);
}
</style>