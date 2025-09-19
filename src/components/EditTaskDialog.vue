<template>
  <div v-if="props.show" class="modal-overlay" @click="onCancel">
    <div class="edit-task-dialog" @click.stop>
      <div class="dialog-header">
        <h3>编辑任务</h3>
        <button class="close-button" @click="onCancel">✕</button>
      </div>
      
      <div class="dialog-content">
        <div class="input-group">
          <label for="taskText">任务内容</label>
          <textarea 
            id="taskText"
            ref="textareaRef"
            v-model="editText"
            placeholder="请输入任务内容..."
            class="task-input"
            rows="3"
            @keydown.enter.prevent="onConfirm"
            @keydown.escape="onCancel"
          ></textarea>
        </div>
      </div>
      
      <div class="dialog-actions">
        <button class="cancel-button" @click="onCancel">取消</button>
        <button 
          class="confirm-button" 
          @click="onConfirm"
          :disabled="!editText.trim()"
        >
          保存
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, nextTick } from 'vue';
import type { Todo } from '../../src/types';

interface Props {
  show: boolean;
  todo: Todo | null;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  confirm: [newText: string];
  cancel: [];
}>();

const editText = ref('');
const textareaRef = ref<HTMLTextAreaElement | null>(null);

// 监听 show 和 todo 的变化，初始化编辑文本
watch([() => props.show, () => props.todo], ([show, todo]) => {
  if (show && todo) {
    editText.value = todo.text;
    // 下一个 tick 时聚焦并选择文本
    nextTick(() => {
      if (textareaRef.value) {
        textareaRef.value.focus();
        textareaRef.value.select();
      }
    });
  }
});

// 确认编辑
function onConfirm() {
  const newText = editText.value.trim();
  if (newText && newText !== props.todo?.text) {
    emit('confirm', newText);
  } else {
    emit('cancel');
  }
}

// 取消编辑
function onCancel() {
  emit('cancel');
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
  backdrop-filter: blur(4px);
}

.edit-task-dialog {
  background: rgba(255, 255, 255, 0.95);
  border-radius: 8px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  min-width: 280px;
  max-width: 320px;
  width: 85%;
  animation: dialogFadeIn 0.2s ease-out;
}

@keyframes dialogFadeIn {
  from {
    opacity: 0;
    transform: scale(0.9) translateY(-10px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

.dialog-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 16px;
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
}

.dialog-header h3 {
  margin: 0;
  font-size: 0.95rem;
  font-weight: 600;
  color: #333;
}

.close-button {
  background: none;
  border: none;
  font-size: 1rem;
  color: #666;
  cursor: pointer;
  padding: 2px;
  border-radius: 3px;
  transition: all 0.2s ease;
}

.close-button:hover {
  background: rgba(0, 0, 0, 0.1);
  color: #333;
}

.dialog-content {
  padding: 12px 16px;
}

.input-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.input-group label {
  font-size: 0.8rem;
  font-weight: 500;
  color: #555;
}

.task-input {
  width: 100%;
  padding: 8px 10px;
  border: 1px solid rgba(0, 0, 0, 0.2);
  border-radius: 6px;
  font-size: 0.85rem;
  font-family: inherit;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(5px);
  transition: all 0.2s ease;
  resize: vertical;
  min-height: 50px;
  box-sizing: border-box;
}

.task-input:focus {
  outline: none;
  border-color: #4CAF50;
  box-shadow: 0 0 0 2px rgba(76, 175, 80, 0.2);
  background: rgba(255, 255, 255, 0.9);
}

.task-input::placeholder {
  color: #999;
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 8px 16px;
  border-top: 1px solid rgba(0, 0, 0, 0.1);
}

.cancel-button,
.confirm-button {
  padding: 6px 12px;
  border: none;
  border-radius: 5px;
  font-size: 0.8rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.cancel-button {
  background: rgba(0, 0, 0, 0.1);
  color: #666;
}

.cancel-button:hover {
  background: rgba(0, 0, 0, 0.15);
  color: #333;
}

.confirm-button {
  background: #4CAF50;
  color: white;
}

.confirm-button:hover:not(:disabled) {
  background: #45a049;
  transform: translateY(-1px);
}

.confirm-button:disabled {
  background: #ccc;
  color: #999;
  cursor: not-allowed;
  transform: none;
}
</style>