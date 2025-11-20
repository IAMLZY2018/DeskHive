<template>
  <div class="add-task-menu-container">
    <div class="add-task">
      <input
        v-model="newTaskText"
        @keyup.enter="addTask"
        placeholder="添加任务，或输入 /分组名 创建分组..."
        ref="inputRef"
      />
      <button 
        class="add-btn"
        @click="addTask"
        @contextmenu.prevent="showMenu"
      >
        ➕
      </button>
    </div>
    
    <!-- 右键菜单 -->
    <div 
      v-if="showContextMenu" 
      class="context-menu"
      :style="{ top: menuPosition.y + 'px', left: menuPosition.x + 'px' }"
      @click.stop
    >
      <div class="menu-item" @click="handleAddTask">
        <span class="menu-icon">☐</span>
        <span>新建任务</span>
      </div>
      <div class="menu-item" @click="handleAddGroup">
        <span class="menu-icon">📁</span>
        <span>新建分组</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';

const newTaskText = ref('');
const inputRef = ref<HTMLInputElement | null>(null);
const showContextMenu = ref(false);
const menuPosition = ref({ x: 0, y: 0 });

const emit = defineEmits<{
  'add-task': [text: string];
  'add-group': [groupName?: string];
}>();

function addTask() {
  const text = newTaskText.value.trim();
  if (!text) return;
  
  // 如果以 / 开头，创建分组
  if (text.startsWith('/')) {
    const groupName = text.slice(1).trim();
    if (groupName) {
      emit('add-group', groupName);
      newTaskText.value = '';
    }
  } else {
    // 否则创建任务
    emit('add-task', text);
    newTaskText.value = '';
  }
}

function showMenu(event: MouseEvent) {
  event.preventDefault();
  menuPosition.value = { x: event.clientX, y: event.clientY };
  showContextMenu.value = true;
  document.addEventListener('click', hideMenu);
}

function hideMenu() {
  showContextMenu.value = false;
  document.removeEventListener('click', hideMenu);
}

function handleAddTask() {
  hideMenu();
  inputRef.value?.focus();
}

function handleAddGroup() {
  hideMenu();
  emit('add-group'); // 不传参数，显示对话框
}

onMounted(() => {
  inputRef.value?.focus();
});

onUnmounted(() => {
  document.removeEventListener('click', hideMenu);
});
</script>

<style scoped>
.add-task-menu-container {
  position: relative;
}

.add-task {
  display: flex;
  padding: clamp(6px, 1.5vh, 10px);
  border-top: 1px solid rgba(229, 231, 235, 0.2);
  background: rgba(255, 255, 255, 0.6);
  gap: clamp(6px, 1.2vw, 10px);
  backdrop-filter: blur(10px);
  min-height: clamp(34px, 6vh, 42px);
  align-items: center;
}

.add-task input {
  flex: 1;
  padding: clamp(6px, 1.5vh, 8px) clamp(8px, 2vw, 10px);
  border: 1px solid rgba(229, 231, 235, 0.3);
  border-radius: clamp(8px, 1.5vw, 12px);
  outline: none;
  background: rgba(255, 255, 255, 0.8);
  color: #333;
  font-size: clamp(0.75rem, 2vw, 0.85rem);
  transition: all 0.3s ease;
  height: clamp(28px, 4.5vh, 32px);
}

.add-task input:focus {
  border-color: #007aff;
  box-shadow: 0 0 8px rgba(0, 122, 255, 0.3);
  background: rgba(255, 255, 255, 0.95);
}

.add-task input::placeholder {
  color: rgba(51, 51, 51, 0.5);
}

.add-btn {
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

.add-btn:hover {
  background: rgba(255, 255, 255, 0.95);
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.15);
  transform: translateY(-2px);
}

.add-btn:active {
  transform: translateY(0);
}

.context-menu {
  position: fixed;
  background: rgba(255, 255, 255, 0.95);
  border: 1px solid rgba(229, 231, 235, 0.3);
  border-radius: 8px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
  backdrop-filter: blur(10px);
  z-index: 1000;
  min-width: 160px;
  padding: 4px;
}

.menu-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 12px;
  cursor: pointer;
  border-radius: 6px;
  transition: all 0.2s ease;
  font-size: 0.85rem;
  color: #333;
}

.menu-item:hover {
  background: rgba(0, 122, 255, 0.1);
}

.menu-icon {
  font-size: 1rem;
}

/* 夜间主题 */
body.dark-theme .add-task {
  background: rgba(24, 26, 27, 0.8);
  border-top: 1px solid rgba(231, 233, 237, 0.2);
}

body.dark-theme .add-task input {
  background: rgba(37, 38, 39, 0.8);
  color: #e7e9ed;
  border-color: rgba(231, 233, 237, 0.2);
}

body.dark-theme .add-task input::placeholder {
  color: rgba(231, 233, 237, 0.5);
}

body.dark-theme .add-btn {
  background: rgba(37, 38, 39, 0.9);
  color: #e7e9ed;
  border-color: rgba(231, 233, 237, 0.2);
}

body.dark-theme .context-menu {
  background: rgba(37, 38, 39, 0.95);
  border-color: rgba(231, 233, 237, 0.3);
}

body.dark-theme .menu-item {
  color: #e7e9ed;
}

body.dark-theme .menu-item:hover {
  background: rgba(0, 122, 255, 0.2);
}
</style>
