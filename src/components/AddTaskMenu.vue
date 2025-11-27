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
      ref="contextMenuRef"
      class="context-menu"
      :style="{ top: adjustedMenuPosition.y + 'px', left: adjustedMenuPosition.x + 'px' }"
      @click.stop
    >
      <div class="menu-item" @click="handleAddGroup">
        <span class="menu-icon">📁</span>
        <span>新建分组</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, nextTick } from 'vue';

const newTaskText = ref('');
const inputRef = ref<HTMLInputElement | null>(null);
const contextMenuRef = ref<HTMLElement | null>(null);
const showContextMenu = ref(false);
const menuPosition = ref({ x: 0, y: 0 });
const adjustedMenuPosition = ref({ x: 0, y: 0 });

const emit = defineEmits<{
  'add-task': [text: string];
  'add-group': [groupName?: string];
}>();

function addTask() {
  const text = newTaskText.value.trim();
  if (!text) return;
  
  // 直接发送文本，让父组件处理是任务还是分组
  emit('add-task', text);
  newTaskText.value = '';
}

// 调整菜单位置以防止溢出屏幕
function adjustMenuPosition() {
  if (!contextMenuRef.value) return;

  const menu = contextMenuRef.value;
  const rect = menu.getBoundingClientRect();
  const viewportWidth = window.innerWidth;
  const viewportHeight = window.innerHeight;

  let newX = menuPosition.value.x;
  let newY = menuPosition.value.y;

  // 检查右侧是否超出视口
  if (newX + rect.width > viewportWidth) {
    newX = viewportWidth - rect.width - 10;
  }

  // 检查底部是否超出视口
  if (newY + rect.height > viewportHeight) {
    newY = viewportHeight - rect.height - 10;
  }

  // 确保不会小于0
  newX = Math.max(10, newX);
  newY = Math.max(10, newY);

  adjustedMenuPosition.value = { x: newX, y: newY };
}

function showMenu(event: MouseEvent) {
  event.preventDefault();
  menuPosition.value = { x: event.clientX, y: event.clientY };
  adjustedMenuPosition.value = { x: event.clientX, y: event.clientY };
  showContextMenu.value = true;
  
  nextTick(() => {
    adjustMenuPosition();
  });
  
  document.addEventListener('click', hideMenu);
}

function hideMenu() {
  showContextMenu.value = false;
  document.removeEventListener('click', hideMenu);
}

function handleAddGroup() {
  hideMenu();
  emit('add-group'); // 不传参数，显示对话框
}

// 监听菜单显示状态
watch(showContextMenu, (newVal) => {
  if (newVal) {
    nextTick(() => {
      adjustMenuPosition();
    });
  }
});

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
  border-radius: clamp(6px, 1.2vw, 8px);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
  backdrop-filter: blur(10px);
  z-index: 1000;
  min-width: clamp(120px, 25vw, 140px);
  padding: clamp(3px, 0.6vh, 4px);
}

.menu-item {
  display: flex;
  align-items: center;
  gap: clamp(6px, 1.2vw, 8px);
  padding: clamp(6px, 1.2vh, 7px) clamp(8px, 1.8vw, 10px);
  cursor: pointer;
  border-radius: clamp(4px, 0.8vw, 5px);
  transition: all 0.2s ease;
  font-size: clamp(0.7rem, 1.8vw, 0.8rem);
  color: #333;
  white-space: nowrap;
}

.menu-item:hover {
  background: rgba(0, 122, 255, 0.1);
  transform: translateX(2px);
}

.menu-icon {
  font-size: clamp(0.8rem, 2vw, 0.9rem);
  flex-shrink: 0;
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
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.3);
}

body.dark-theme .menu-item {
  color: #e7e9ed;
}

body.dark-theme .menu-item:hover {
  background: rgba(0, 122, 255, 0.2);
  transform: translateX(2px);
}
</style>
