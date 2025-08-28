<template>
  <div class="add-task">
    <input 
      type="text" 
      placeholder="添加新任务..." 
      v-model="taskText" 
      @keypress.enter="onAddTask"
    >
    <button @click="onAddTask">➕</button>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const taskText = ref('');

interface Emits {
  (e: 'add-task', text: string): void;
}

const emit = defineEmits<Emits>();

function onAddTask() {
  const text = taskText.value.trim();
  if (text) {
    emit('add-task', text);
    taskText.value = '';
  }
}
</script>

<style scoped>
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
</style>