<template>
  <div v-if="props.show" class="dialog-overlay" @click="onCancel">
    <div class="dialog-box" @click.stop>
      <div class="dialog-header">
        <svg class="dialog-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M11 4H4C3.46957 4 2.96086 4.21071 2.58579 4.58579C2.21071 4.96086 2 5.46957 2 6V20C2 20.5304 2.21071 21.0391 2.58579 21.4142C2.96086 21.7893 3.46957 22 4 22H18C18.5304 22 19.0391 21.7893 19.4142 21.4142C19.7893 21.0391 20 20.5304 20 20V13" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M18.5 2.50001C18.8978 2.10219 19.4374 1.87869 20 1.87869C20.5626 1.87869 21.1022 2.10219 21.5 2.50001C21.8978 2.89784 22.1213 3.4374 22.1213 4.00001C22.1213 4.56262 21.8978 5.10219 21.5 5.50001L12 15L8 16L9 12L18.5 2.50001Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <h3 class="dialog-title">编辑任务</h3>
      </div>
      
      <div class="dialog-content">
        <textarea 
          id="taskText"
          ref="textareaRef"
          v-model="editText"
          placeholder="请输入任务内容..."
          class="dialog-input"
          rows="3"
          @keydown.ctrl.enter="onConfirm"
          @keydown.escape="onCancel"
        ></textarea>
      </div>
      
      <div class="dialog-buttons">
        <button class="dialog-btn cancel" @click="onCancel">
          <svg class="btn-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M18 6L6 18M6 6L18 18" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
          </svg>
          <span>取消</span>
        </button>
        <button 
          class="dialog-btn confirm" 
          @click="onConfirm"
          :disabled="!editText.trim()"
        >
          <svg class="btn-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M20 6L9 17L4 12" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          <span>保存</span>
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
.dialog-overlay {
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
  animation: fadeIn 0.2s ease;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

.dialog-box {
  background: rgba(255, 255, 255, 0.98);
  border-radius: 10px;
  padding: 16px;
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.2);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(0, 0, 0, 0.08);
  min-width: 240px;
  max-width: 280px;
  animation: slideUp 0.3s ease;
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.dialog-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 14px;
  padding-bottom: 10px;
  border-bottom: 1px solid rgba(0, 0, 0, 0.06);
}

.dialog-icon {
  width: 20px;
  height: 20px;
  color: #888;
  flex-shrink: 0;
}

.dialog-title {
  font-size: 0.88rem;
  font-weight: 600;
  color: #333;
  margin: 0;
}

.dialog-content {
  margin-bottom: 14px;
}

.dialog-input {
  width: 100%;
  padding: 8px 10px;
  border: 1.5px solid rgba(0, 0, 0, 0.1);
  border-radius: 7px;
  outline: none;
  background: rgba(255, 255, 255, 0.95);
  color: #333;
  font-size: 0.8rem;
  font-family: inherit;
  transition: all 0.2s ease;
  box-sizing: border-box;
  resize: vertical;
  min-height: 70px;
  line-height: 1.5;
}

.dialog-input:hover {
  border-color: rgba(0, 0, 0, 0.15);
}

.dialog-input:focus {
  border-color: #6EE748;
  box-shadow: 0 0 0 3px rgba(110, 231, 72, 0.1);
  background: #fff;
}

.dialog-input::placeholder {
  color: #aaa;
}

.dialog-buttons {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}

.dialog-btn {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 7px 14px;
  border: none;
  border-radius: 7px;
  font-size: 0.75rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  font-family: inherit;
}

.btn-icon {
  width: 14px;
  height: 14px;
}

.dialog-btn.cancel {
  background: rgba(0, 0, 0, 0.04);
  color: #666;
}

.dialog-btn.cancel:hover {
  background: rgba(0, 0, 0, 0.08);
  color: #333;
}

.dialog-btn.cancel:active {
  transform: scale(0.96);
}

.dialog-btn.confirm {
  background: #6EE748;
  color: #000;
}

.dialog-btn.confirm:hover:not(:disabled) {
  background: #5fc940;
  box-shadow: 0 4px 12px rgba(110, 231, 72, 0.3);
}

.dialog-btn.confirm:active:not(:disabled) {
  transform: scale(0.96);
}

.dialog-btn.confirm:disabled {
  background: rgba(0, 0, 0, 0.1);
  color: #999;
  cursor: not-allowed;
  box-shadow: none;
}

/* 夜间主题 */
body.dark-theme .dialog-overlay {
  background: rgba(0, 0, 0, 0.7);
}

body.dark-theme .dialog-box {
  background: rgba(30, 30, 30, 0.98);
  border-color: rgba(255, 255, 255, 0.1);
}

body.dark-theme .dialog-header {
  border-bottom-color: rgba(255, 255, 255, 0.08);
}

body.dark-theme .dialog-title {
  color: #e0e0e0;
}

body.dark-theme .dialog-input {
  background: rgba(255, 255, 255, 0.05);
  border-color: rgba(255, 255, 255, 0.1);
  color: #e0e0e0;
}

body.dark-theme .dialog-input:hover {
  border-color: rgba(255, 255, 255, 0.15);
}

body.dark-theme .dialog-input:focus {
  background: rgba(255, 255, 255, 0.08);
  border-color: #6EE748;
  box-shadow: 0 0 0 3px rgba(110, 231, 72, 0.15);
}

body.dark-theme .dialog-input::placeholder {
  color: #666;
}

body.dark-theme .dialog-btn.cancel {
  background: rgba(255, 255, 255, 0.06);
  color: #aaa;
}

body.dark-theme .dialog-btn.cancel:hover {
  background: rgba(255, 255, 255, 0.1);
  color: #e0e0e0;
}

body.dark-theme .dialog-btn.confirm:disabled {
  background: rgba(255, 255, 255, 0.06);
  color: #666;
}
</style>