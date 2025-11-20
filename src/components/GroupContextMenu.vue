<template>
  <div 
    v-if="show" 
    class="group-context-menu"
    :style="{ top: position.y + 'px', left: position.x + 'px' }"
    @click.stop
  >
    <div class="menu-item" @click="handleRename">
      <span class="menu-icon">✏️</span>
      <span>重命名</span>
    </div>
    <div class="menu-item" @click="handleToggleCollapse">
      <span class="menu-icon">{{ group?.collapsed ? '📂' : '📁' }}</span>
      <span>{{ group?.collapsed ? '展开' : '折叠' }}</span>
    </div>
    <div 
      v-if="!isDefaultGroup"
      class="menu-item danger" 
      @click="handleDelete"
    >
      <span class="menu-icon">🗑️</span>
      <span>删除分组</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
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
</script>

<style scoped>
.group-context-menu {
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

.menu-item.danger {
  color: #f44336;
}

.menu-item.danger:hover {
  background: rgba(244, 67, 54, 0.1);
}

.menu-icon {
  font-size: 1rem;
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
