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
        <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M12 5V19M5 12H19" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
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
  padding: 8px;
  border-top: 1px solid rgba(229, 231, 235, 0.2);
  background: rgba(255, 255, 255, 0.6);
  gap: 8px;
  backdrop-filter: blur(10px);
  min-height: 38px;
  align-items: center;
}

.add-task input {
  flex: 1;
  padding: 7px 9px;
  border: 1px solid rgba(229, 231, 235, 0.3);
  border-radius: 10px;
  outline: none;
  background: rgba(255, 255, 255, 0.8);
  color: #333;
  font-size: 0.8rem;
  transition: all 0.3s ease;
  height: 30px;
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
  width: 30px;
  height: 30px;
  background: rgba(255, 255, 255, 0.8);
  border: none;
  border-radius: 50%;
  cursor: pointer;
  color: #333;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  backdrop-filter: blur(5px);
  border: 1px solid rgba(229, 231, 235, 0.2);
  padding: 0;
}

.add-btn svg {
  width: 18px;
  height: 18px;
  transition: transform 0.3s ease;
}

.add-btn:hover {
  background: rgba(255, 255, 255, 0.95);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.12);
  transform: translateY(-1px);
}

.add-btn:hover svg {
  transform: rotate(90deg);
}

.add-btn:active {
  transform: translateY(0);
}

.context-menu {
  position: fixed;
  background: rgba(255, 255, 255, 0.95);
  border: 1px solid rgba(229, 231, 235, 0.3);
  border-radius: 7px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
  backdrop-filter: blur(10px);
  z-index: 1000;
  min-width: 130px;
  padding: 3.5px;
}

.menu-item {
  display: flex;
  align-items: center;
  gap: 7px;
  padding: 6.5px 9px;
  cursor: pointer;
  border-radius: 4.5px;
  transition: all 0.2s ease;
  font-size: 0.75rem;
  color: #333;
  white-space: nowrap;
}

.menu-item:hover {
  background: rgba(0, 122, 255, 0.1);
  transform: translateX(2px);
}

.menu-icon {
  font-size: 0.85rem;
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
