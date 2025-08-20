<template>
  <div class="container">
    <header :data-tauri-drag-region="!isDragDisabled ? '' : null">
      <div class="header-title" :data-tauri-drag-region="!isDragDisabled ? '' : null">🐝 DeskHive</div>
      <div class="header-right">
        <div class="progress-indicator">{{ completedTasks }}/{{ totalTasks }}</div>
        <button class="settings-btn" @click="openSettings">⚙️</button>
      </div>
    </header>

    <div class="todo-container">
      <div class="todo-section">
        <h3 class="section-title">待完成</h3>
        <div class="todo-list">
          <TransitionGroup name="todo-list" tag="div">
            <div v-for="(todo, index) in pendingTodos" :key="index" :class="['todo-item']" @dblclick="deleteTodo(index)">
              <div class="todo-checkbox" @click="toggleTodo(index)"></div>
              <span>{{ todo.text }}</span>
            </div>
          </TransitionGroup>
        </div>
      </div>
      
      <div v-if="completedTodos.length > 0" class="todo-section completed-section">
        <h3 class="section-title">已完成</h3>
        <div class="todo-list">
          <TransitionGroup name="todo-list" tag="div">
            <div v-for="(todo, index) in completedTodos" :key="index" class="todo-item completed" @dblclick="deleteCompletedTodo(index)">
              <div class="todo-checkbox completed" @click="toggleCompletedTodo(index)"></div>
              <span>{{ todo.text }}</span>
            </div>
          </TransitionGroup>
        </div>
      </div>
    </div>
    <div class="add-task">
      <input type="text" placeholder="添加新任务..." v-model="newTaskText" @keypress.enter="addTask">
      <button @click="addTask">➕</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { TransitionGroup } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';

interface Todo {
  text: string;
  completed: boolean;
}

const pendingTodos = ref<Todo[]>([]);
const completedTodos = ref<Todo[]>([]);

const newTaskText = ref('');
const totalTasks = computed(() => pendingTodos.value.length + completedTodos.value.length);
const completedTasks = computed(() => completedTodos.value.length);

// 拖动设置状态
const isDragDisabled = ref(false);

// 保存数据到本地文件
async function saveTodoData() {
  try {
    await invoke('save_todo_data', {
      pendingTodos: pendingTodos.value,
      completedTodos: completedTodos.value
    });
    console.log('数据保存成功');
  } catch (error) {
    console.error('保存数据失败:', error);
  }
}

// 从本地文件加载数据
async function loadTodoData() {
  try {
    const data = await invoke('load_todo_data') as {
      pending_todos: Todo[],
      completed_todos: Todo[]
    };
    pendingTodos.value = data.pending_todos;
    completedTodos.value = data.completed_todos;
    console.log('数据加载成功');
  } catch (error) {
    console.error('加载数据失败:', error);
    // 如果加载失败，使用默认数据
    pendingTodos.value = [
      { text: '学习SpringBoot3.5', completed: false },
      { text: '测试部署到服务器', completed: false }
    ];
    completedTodos.value = [
      { text: '完成UI设计', completed: true }
    ];
  }
}

async function openSettings() {
  try {
    await invoke('open_settings_window');
  } catch (error) {
    console.error('打开设置窗口失败:', error);
  }
}

function addTask() {
  const taskText = newTaskText.value.trim();
  if (taskText) {
    pendingTodos.value.push({
      text: taskText,
      completed: false
    });
    newTaskText.value = '';
    // 保存数据
    saveTodoData();
  }
}

function toggleTodo(index: number) {
  const todo = pendingTodos.value[index];
  pendingTodos.value.splice(index, 1);
  completedTodos.value.push({
    text: todo.text,
    completed: true
  });
  // 保存数据
  saveTodoData();
}

function toggleCompletedTodo(index: number) {
  const todo = completedTodos.value[index];
  completedTodos.value.splice(index, 1);
  pendingTodos.value.push({
    text: todo.text,
    completed: false
  });
  // 保存数据
  saveTodoData();
}

function deleteTodo(index: number) {
  pendingTodos.value.splice(index, 1);
  // 保存数据
  saveTodoData();
}

function deleteCompletedTodo(index: number) {
  completedTodos.value.splice(index, 1);
  // 保存数据
  saveTodoData();
}

// 加载应用设置
async function loadAppSettings() {
  try {
    const settings = await invoke('load_app_settings') as {
      opacity: number,
      always_on_top: boolean,
      disable_drag: boolean,
      auto_show: boolean,
      minimize_to_tray: boolean,
      hotkey: string
    };
    isDragDisabled.value = settings.disable_drag;
    console.log('应用设置加载成功，拖动禁用状态:', isDragDisabled.value);
  } catch (error) {
    console.error('加载应用设置失败:', error);
  }
}

// 组件挂载时加载数据
onMounted(async () => {
  console.log('前端渲染完成');
  console.log('Vue 组件已挂载');
  console.log('待办事项数量:', pendingTodos.value.length);
  
  // 加载todo数据和应用设置
  await loadTodoData();
  await loadAppSettings();
  
  // 监听拖动设置变化
  const currentWindow = getCurrentWindow();
  await currentWindow.listen('drag-setting-changed', (event) => {
    isDragDisabled.value = event.payload as boolean;
    console.log('拖动设置已更新:', isDragDisabled.value);
  });
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
  cursor: default;
  user-select: none;
}
.header-right {
  display: flex;
  align-items: center;
  gap: clamp(6px, 1.5vw, 10px);
}
.progress-indicator {
  font-size: clamp(0.7rem, 1.8vw, 0.8rem);
  color: #333;
  font-weight: 600;
  background: rgba(255, 255, 255, 0.8);
  padding: clamp(2px, 0.5vh, 4px) clamp(6px, 1.5vw, 8px);
  border-radius: clamp(8px, 1.5vw, 12px);
  border: 1px solid rgba(255, 255, 255, 0.4);
  min-width: clamp(28px, 6vw, 35px);
  text-align: center;
  backdrop-filter: blur(5px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}
.settings-btn {
  width: clamp(24px, 4.5vw, 28px);
  height: clamp(24px, 4.5vw, 28px);
  border: none;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.8);
  color: #333;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: clamp(0.65rem, 1.8vw, 0.8rem);
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  flex-shrink: 0;
  backdrop-filter: blur(5px);
  border: 1px solid rgba(255, 255, 255, 0.4);
}
.settings-btn:hover {
  transform: rotate(90deg) scale(1.08);
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.15);
  background: rgba(255, 255, 255, 0.95);
}
.todo-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow-y: auto;
  background: transparent;
  min-height: 0;
}

.todo-container::-webkit-scrollbar {
  width: 5px;
}

.todo-container::-webkit-scrollbar-track {
  background: rgba(104, 58, 183, 0.1);
  border-radius: 3px;
}

.todo-container::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.8);
  border-radius: 3px;
  border: 1px solid rgba(255, 255, 255, 0.4);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
}

.todo-section {
  margin-bottom: 10px;
  padding: clamp(8px, 2vh, 12px);
}

.section-title {
  font-size: clamp(0.8rem, 2.2vw, 0.9rem);
  color: #555;
  margin-bottom: 8px;
  font-weight: 600;
  padding-left: 5px;
}

.completed-section {
  margin-top: auto;
  border-top: 1px dashed rgba(104, 58, 183, 0.2);
  padding-top: 10px;
}

.todo-list {
  min-height: 0;
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
  cursor: pointer;
  position: relative;
}
.todo-item:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(104, 58, 183, 0.15);
  border-color: rgba(104, 58, 183, 0.2);
}

.todo-item:hover::after {
  content: '双击删除';
  position: absolute;
  right: 10px;
  font-size: 0.7rem;
  color: #888;
  opacity: 0.7;
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
  border-color: rgba(255, 255, 255, 0.4);
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
  background: rgba(255, 255, 255, 0.8);
  border: none;
  border-radius: 50%;
  cursor: pointer;
  color: #333;
  font-weight: 700;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  transition: all 0.3s ease;
  font-size: clamp(0.7rem, 2vw, 0.85rem);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  backdrop-filter: blur(5px);
  border: 1px solid rgba(255, 255, 255, 0.4);
}
.add-task button:hover {
  transform: translateY(-1px);
  box-shadow: 0 6px 16px rgba(104, 58, 183, 0.4);
}
.add-task button:active {
  transform: translateY(0);
}
.add-task button:hover {
  background: rgba(255, 255, 255, 0.95);
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.15);
  transform: translateY(-2px);
}

/* Todo列表过渡动画 */
.todo-list-enter-active,
.todo-list-leave-active {
  transition: all 0.5s ease;
}
.todo-list-enter-from {
  opacity: 0;
  transform: translateY(30px);
}
.todo-list-leave-to {
  opacity: 0;
  transform: translateX(100px);
}
.todo-list-move {
  transition: transform 0.5s ease;
}
</style>