<template>
  <div v-if="show" class="dialog-overlay" @click="onClose">
    <div class="dialog-box" @click.stop>
      <h3 class="dialog-title">📅 设置截止时间</h3>
      <div class="dialog-content">
        <div class="input-group">
          <label for="deadline-date">日期：</label>
          <input 
            type="date" 
            id="deadline-date" 
            v-model="deadlineDate" 
            class="dialog-input"
          >
        </div>
        <div class="input-group">
          <label for="deadline-time">时间：</label>
          <input 
            type="time" 
            id="deadline-time" 
            v-model="deadlineTime" 
            class="dialog-input"
          >
        </div>
      </div>
      <div class="dialog-buttons">
        <button class="dialog-btn cancel" @click="onClose">取消</button>
        <button class="dialog-btn confirm" @click="onConfirm">确定</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';

interface Props {
  show: boolean;
  initialDate?: string;
  initialTime?: string;
}

interface Emits {
  (e: 'close'): void;
  (e: 'confirm', date: string, time: string): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const deadlineDate = ref(props.initialDate || '');
const deadlineTime = ref(props.initialTime || '');

watch(() => props.initialDate, (newVal) => {
  deadlineDate.value = newVal || '';
});

watch(() => props.initialTime, (newVal) => {
  deadlineTime.value = newVal || '';
});

function onClose() {
  emit('close');
}

function onConfirm() {
  if (deadlineDate.value && deadlineTime.value) {
    emit('confirm', deadlineDate.value, deadlineTime.value);
  }
}
</script>

<style scoped>
/* 对话框遮罩 */
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
  backdrop-filter: blur(3px);
}

/* 对话框主体 */
.dialog-box {
  background: rgba(255, 255, 255, 0.95);
  border-radius: 12px;
  padding: 24px;
  box-shadow: 0 12px 32px rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(104, 58, 183, 0.2);
  min-width: 300px;
  max-width: 400px;
}

/* 对话框标题 */
.dialog-title {
  font-size: 1.1rem;
  font-weight: 600;
  color: #333;
  margin-bottom: 20px;
  text-align: center;
}

/* 对话框内容 */
.dialog-content {
  margin-bottom: 24px;
}

/* 输入组 */
.input-group {
  margin-bottom: 16px;
}

.input-group label {
  display: block;
  margin-bottom: 6px;
  font-size: 0.9rem;
  font-weight: 500;
  color: #555;
}

/* 对话框输入框 */
.dialog-input {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid rgba(104, 58, 183, 0.3);
  border-radius: 8px;
  outline: none;
  background: rgba(255, 255, 255, 0.9);
  color: #333;
  font-size: 0.9rem;
  transition: all 0.3s ease;
  box-sizing: border-box;
}

.dialog-input:focus {
  border-color: #683ab7;
  box-shadow: 0 0 8px rgba(104, 58, 183, 0.2);
  background: rgba(255, 255, 255, 1);
}

/* 对话框按钮组 */
.dialog-buttons {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
}

/* 对话框按钮 */
.dialog-btn {
  padding: 10px 20px;
  border: none;
  border-radius: 8px;
  font-size: 0.9rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
  backdrop-filter: blur(5px);
}

.dialog-btn.cancel {
  background: rgba(158, 158, 158, 0.2);
  color: #666;
  border: 1px solid rgba(158, 158, 158, 0.3);
}

.dialog-btn.cancel:hover {
  background: rgba(158, 158, 158, 0.3);
  color: #333;
}

.dialog-btn.confirm {
  background: #683ab7;
  color: white;
  border: 1px solid #683ab7;
}

.dialog-btn.confirm:hover {
  background: #5e35a1;
  box-shadow: 0 4px 12px rgba(104, 58, 183, 0.3);
}
</style>