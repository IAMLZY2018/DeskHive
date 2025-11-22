<template>
  <div 
    v-if="show" 
    ref="menuRef"
    class="group-context-menu"
    :style="{ top: adjustedPosition.y + 'px', left: adjustedPosition.x + 'px' }"
    @click.stop
  >
    <div class="menu-item" @click="handleRename">
      <span>重命名</span>
    </div>
    <div class="menu-item" @click="handleToggleCollapse">
      <span>{{ group?.collapsed ? '展开' : '折叠' }}</span>
    </div>
    <div 
      v-if="!isDefaultGroup"
      class="menu-item danger" 
      @click="handleDelete"
    >
      <span>删除分组</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch, nextTick } from 'vue';
import type { TodoGroup } from '../types';

interface Props {
  show: boolean;
  position: { x: number; y: number };
  group: TodoGroup | null;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  'rename': [];
  'toggle-collapse': [];
  'delete': [];
}>();

const menuRef = ref<HTMLElement | null>(null);
const adjustedPosition = ref({ x: 0, y: 0 });

const isDefaultGroup = computed(() => props.group?.id === 'default');

function handleRename() {
  emit('rename');
}

function handleToggleCollapse() {
  emit('toggle-collapse');
}

function handleDelete() {
  emit('delete');
}

// 调整菜单位置以防止溢出屏幕
function adjustMenuPosition() {
  if (!menuRef.value) return;

  const menu = menuRef.value;
  const rect = menu.getBoundingClientRect();
  const viewportWidth = window.innerWidth;
  const viewportHeight = window.innerHeight;

  let newX = props.position.x;
  let newY = props.position.y;

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

  adjustedPosition.value = { x: newX, y: newY };
}

// 监听显示状态和位置变化
watch(() => [props.show, props.position.x, props.position.y], () => {
  if (props.show) {
    adjustedPosition.value = { ...props.position };
    nextTick(() => {
      adjustMenuPosition();
    });
  }
}, { immediate: true });
</script>

<style scoped>
.group-context-menu {
  position: fixed;
  background: rgba(255, 255, 255, 0.95);
  border: 1px solid rgba(229, 231, 235, 0.3);
  border-radius: clamp(6px, 1.2vw, 8px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  backdrop-filter: blur(10px);
  z-index: 1000;
  min-width: clamp(100px, 20vw, 120px);
  padding: clamp(2px, 0.5vh, 3px);
}

.menu-item {
  display: flex;
  align-items: center;
  padding: clamp(4px, 1vh, 6px) clamp(8px, 2vw, 10px);
  cursor: pointer;
  border-radius: clamp(4px, 0.8vw, 5px);
  transition: all 0.2s ease;
  font-size: clamp(0.7rem, 1.8vw, 0.8rem);
  color: #333;
}

.menu-item:hover {
  background: rgba(0, 122, 255, 0.1);
}

.menu-item.danger {
  color: #f44336;
}

.menu-item.danger:hover {
  background: rgba(244, 67, 54, 0.1);
}



/* 夜间主题 */
body.dark-theme .group-context-menu {
  background: rgba(37, 38, 39, 0.95);
  border-color: rgba(231, 233, 237, 0.3);
}

body.dark-theme .menu-item {
  color: #e7e9ed;
}

body.dark-theme .menu-item:hover {
  background: rgba(0, 122, 255, 0.2);
}

body.dark-theme .menu-item.danger:hover {
  background: rgba(244, 67, 54, 0.2);
}
</style>
