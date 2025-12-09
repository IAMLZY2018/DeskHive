<template>
  <div v-if="show" class="dialog-overlay" @click="onClose">
    <div class="dialog-box" @click.stop>
      <div class="dialog-header">
        <svg class="dialog-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect x="3" y="4" width="18" height="18" rx="2" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M16 2V6M8 2V6M3 10H21" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        <h3 class="dialog-title">设置截止时间</h3>
      </div>
      
      <div class="dialog-content">
        <div class="input-group">
          <label for="deadline-date" class="input-label">
            <svg class="label-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <rect x="3" y="4" width="18" height="18" rx="2" stroke="currentColor" stroke-width="2"/>
              <path d="M3 10H21M8 2V6M16 2V6" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
            </svg>
            <span>日期</span>
          </label>
          <input 
            type="date" 
            id="deadline-date" 
            v-model="deadlineDate" 
            class="dialog-input"
          >
        </div>
        
        <div class="input-group">
          <label for="deadline-time" class="input-label">
            <svg class="label-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2"/>
              <path d="M12 6V12L16 14" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
            </svg>
            <span>时间</span>
          </label>
          <input 
            type="time" 
            id="deadline-time" 
            v-model="deadlineTime" 
            class="dialog-input"
          >
        </div>
      </div>
      
      <div class="dialog-buttons">
        <button class="dialog-btn cancel" @click="onClose">
          <svg class="btn-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M18 6L6 18M6 6L18 18" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
          </svg>
          <span>取消</span>
        </button>
        <button class="dialog-btn confirm" @click="onConfirm">
          <svg class="btn-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M20 6L9 17L4 12" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
          <span>确定</span>
        </button>
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

// 获取当前日期和时间
function getCurrentDateTime() {
  const now = new Date();
  const date = now.toISOString().split('T')[0]; // YYYY-MM-DD
  const hours = String(now.getHours()).padStart(2, '0');
  const minutes = String(now.getMinutes()).padStart(2, '0');
  const time = `${hours}:${minutes}`; // HH:MM
  return { date, time };
}

// 使用ref来创建响应式变量，初始值来自props或当前时间
const { date: currentDate, time: currentTime } = getCurrentDateTime();
const deadlineDate = ref(props.initialDate || currentDate);
const deadlineTime = ref(props.initialTime || currentTime);

// 监听props的变化，更新本地响应式变量
watch(() => props.initialDate, (newVal) => {
  if (newVal) {
    deadlineDate.value = newVal;
  } else {
    // 如果没有初始值，使用当前日期
    deadlineDate.value = getCurrentDateTime().date;
  }
});

watch(() => props.initialTime, (newVal) => {
  if (newVal) {
    deadlineTime.value = newVal;
  } else {
    // 如果没有初始值，使用当前时间
    deadlineTime.value = getCurrentDateTime().time;
  }
});

// 监听show属性，当对话框打开时重置为当前时间（如果没有初始值）
watch(() => props.show, (newVal) => {
  if (newVal && !props.initialDate && !props.initialTime) {
    const { date, time } = getCurrentDateTime();
    deadlineDate.value = date;
    deadlineTime.value = time;
  }
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

.input-group {
  margin-bottom: 12px;
}

.input-group:last-child {
  margin-bottom: 0;
}

.input-label {
  display: flex;
  align-items: center;
  gap: 5px;
  margin-bottom: 6px;
  font-size: 0.75rem;
  font-weight: 600;
  color: #555;
}

.label-icon {
  width: 14px;
  height: 14px;
  color: #888;
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
  cursor: pointer;
}

.dialog-input:hover {
  border-color: rgba(0, 0, 0, 0.15);
  background: #fff;
}

.dialog-input:focus {
  border-color: #6EE748;
  box-shadow: 0 0 0 3px rgba(110, 231, 72, 0.1);
  background: #fff;
  cursor: text;
}

/* 美化日期和时间选择器 */
.dialog-input[type="date"],
.dialog-input[type="time"] {
  position: relative;
  padding-right: 36px;
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.95) 0%, rgba(250, 250, 250, 0.95) 100%);
  font-weight: 500;
}

/* 日期选择器图标 */
.dialog-input[type="date"]::-webkit-calendar-picker-indicator,
.dialog-input[type="time"]::-webkit-calendar-picker-indicator {
  position: absolute;
  right: 8px;
  width: 18px;
  height: 18px;
  cursor: pointer;
  opacity: 0.5;
  transition: all 0.2s ease;
  background-size: contain;
  background-repeat: no-repeat;
  background-position: center;
}

.dialog-input[type="date"]:hover::-webkit-calendar-picker-indicator,
.dialog-input[type="time"]:hover::-webkit-calendar-picker-indicator {
  opacity: 0.8;
  transform: scale(1.1);
}

/* 清除按钮样式 */
.dialog-input[type="date"]::-webkit-clear-button,
.dialog-input[type="time"]::-webkit-clear-button {
  display: none;
}

/* 内部输入框样式 */
.dialog-input[type="date"]::-webkit-datetime-edit,
.dialog-input[type="time"]::-webkit-datetime-edit {
  padding: 0;
}

.dialog-input[type="date"]::-webkit-datetime-edit-fields-wrapper,
.dialog-input[type="time"]::-webkit-datetime-edit-fields-wrapper {
  padding: 0;
}

/* 日期和时间字段样式 */
.dialog-input[type="date"]::-webkit-datetime-edit-year-field,
.dialog-input[type="date"]::-webkit-datetime-edit-month-field,
.dialog-input[type="date"]::-webkit-datetime-edit-day-field,
.dialog-input[type="time"]::-webkit-datetime-edit-hour-field,
.dialog-input[type="time"]::-webkit-datetime-edit-minute-field,
.dialog-input[type="time"]::-webkit-datetime-edit-ampm-field {
  padding: 3px 5px;
  border-radius: 4px;
  transition: all 0.2s ease;
  font-weight: 600;
}

.dialog-input[type="date"]::-webkit-datetime-edit-year-field:focus,
.dialog-input[type="date"]::-webkit-datetime-edit-month-field:focus,
.dialog-input[type="date"]::-webkit-datetime-edit-day-field:focus,
.dialog-input[type="time"]::-webkit-datetime-edit-hour-field:focus,
.dialog-input[type="time"]::-webkit-datetime-edit-minute-field:focus,
.dialog-input[type="time"]::-webkit-datetime-edit-ampm-field:focus {
  background: linear-gradient(135deg, rgba(110, 231, 72, 0.2) 0%, rgba(110, 231, 72, 0.15) 100%);
  color: #000;
  outline: none;
  box-shadow: 0 0 0 2px rgba(110, 231, 72, 0.1);
}

/* 分隔符样式 */
.dialog-input[type="date"]::-webkit-datetime-edit-text,
.dialog-input[type="time"]::-webkit-datetime-edit-text {
  color: #bbb;
  padding: 0 3px;
  font-weight: 400;
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

.dialog-btn.confirm:hover {
  background: #5fc940;
  box-shadow: 0 4px 12px rgba(110, 231, 72, 0.3);
}

.dialog-btn.confirm:active {
  transform: scale(0.96);
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

body.dark-theme .input-label {
  color: #aaa;
}

body.dark-theme .label-icon {
  color: #888;
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

/* 夜间主题 - 日期时间选择器 */
body.dark-theme .dialog-input[type="date"]::-webkit-calendar-picker-indicator,
body.dark-theme .dialog-input[type="time"]::-webkit-calendar-picker-indicator {
  filter: invert(1);
  opacity: 0.7;
}

body.dark-theme .dialog-input[type="date"]:hover::-webkit-calendar-picker-indicator,
body.dark-theme .dialog-input[type="time"]:hover::-webkit-calendar-picker-indicator {
  opacity: 1;
  transform: scale(1.1);
}

body.dark-theme .dialog-input[type="date"]::-webkit-datetime-edit-year-field:focus,
body.dark-theme .dialog-input[type="date"]::-webkit-datetime-edit-month-field:focus,
body.dark-theme .dialog-input[type="date"]::-webkit-datetime-edit-day-field:focus,
body.dark-theme .dialog-input[type="time"]::-webkit-datetime-edit-hour-field:focus,
body.dark-theme .dialog-input[type="time"]::-webkit-datetime-edit-minute-field:focus,
body.dark-theme .dialog-input[type="time"]::-webkit-datetime-edit-ampm-field:focus {
  background: linear-gradient(135deg, rgba(110, 231, 72, 0.25) 0%, rgba(110, 231, 72, 0.2) 100%);
  color: #fff;
  box-shadow: 0 0 0 2px rgba(110, 231, 72, 0.15);
}

body.dark-theme .dialog-input[type="date"]::-webkit-datetime-edit-text,
body.dark-theme .dialog-input[type="time"]::-webkit-datetime-edit-text {
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
</style>
