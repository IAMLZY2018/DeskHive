<template>
  <div v-if="show" class="dialog-overlay" @click="handleCancel">
    <div class="dialog" @click.stop>
      <h3 class="dialog-title">{{ isEdit ? '重命名分组' : '新建分组' }}</h3>
      <input
        v-model="groupName"
        @keyup.enter="handleConfirm"
        @keyup.esc="handleCancel"
        placeholder="输入分组名称..."
        ref="inputRef"
        class="dialog-input"
      />
      <div class="dialog-actions">
        <button class="dialog-btn cancel" @click="handleCancel">取消</button>
        <button class="dialog-btn confirm" @click="handleConfirm">确定</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, nextTick } from 'vue';

interface Props {
  show: boolean;
  initialName?: string;
  isEdit?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  initialName: '',
  isEdit: false
});

const emit = defineEmits<{
  'confirm': [name: string];
  'cancel': [];
}>();

const groupName = ref('');
const inputRef = ref<HTMLInputElement | null>(null);

watch(() => props.show, (newVal) => {
  if (newVal) {
    groupName.value = props.initialName;
    nextTick(() => {
      inputRef.value?.focus();
      inputRef.value?.select();
    });
  }
});

function handleConfirm() {
  if (groupName.value.trim()) {
    emit('confirm', groupName.value.trim());
    groupName.value = '';
  }
}

function handleCancel() {
  emit('cancel');
  groupName.value = '';
}
</script>

<style scoped>
.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
  backdrop-filter: blur(4px);
}

.dialog {
  background: rgba(255, 255, 255, 0.95);
  border-radius: clamp(6px, 1.2vw, 8px);
  padding: clamp(12px, 2.5vh, 14px) clamp(14px, 3vw, 16px);
  min-width: clamp(200px, 40vw, 240px);
  max-width: clamp(260px, 55vw, 300px);
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(229, 231, 235, 0.3);
}

.dialog-title {
  font-size: clamp(0.85rem, 2vw, 0.95rem);
  font-weight: 600;
  color: #333;
  margin-bottom: clamp(8px, 1.8vh, 10px);
}

.dialog-input {
  width: 100%;
  padding: clamp(6px, 1.3vh, 7px) clamp(9px, 1.8vw, 10px);
  border: 1px solid rgba(229, 231, 235, 0.3);
  border-radius: clamp(5px, 1vw, 6px);
  outline: none;
  background: rgba(255, 255, 255, 0.8);
  color: #333;
  font-size: clamp(0.7rem, 1.6vw, 0.8rem);
  transition: all 0.3s ease;
  margin-bottom: clamp(8px, 1.8vh, 10px);
}

.dialog-input:focus {
  border-color: #007aff;
  box-shadow: 0 0 6px rgba(0, 122, 255, 0.3);
  background: rgba(255, 255, 255, 0.95);
}

.dialog-actions {
  display: flex;
  gap: clamp(5px, 1vw, 6px);
  justify-content: flex-end;
}

.dialog-btn {
  padding: clamp(5px, 1vh, 6px) clamp(10px, 2.2vw, 12px);
  border: none;
  border-radius: clamp(4px, 0.8vw, 5px);
  cursor: pointer;
  font-size: clamp(0.65rem, 1.6vw, 0.75rem);
  font-weight: 500;
  transition: all 0.2s ease;
  min-width: clamp(45px, 11vw, 55px);
}

.dialog-btn.cancel {
  background: rgba(0, 0, 0, 0.05);
  color: #666;
}

.dialog-btn.cancel:hover {
  background: rgba(0, 0, 0, 0.1);
  transform: translateY(-1px);
}

.dialog-btn.confirm {
  background: #007aff;
  color: white;
}

.dialog-btn.confirm:hover {
  background: #0051d5;
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0, 122, 255, 0.3);
}

/* 夜间主题 */
body.dark-theme .dialog {
  background: rgba(37, 38, 39, 0.95);
  border-color: rgba(231, 233, 237, 0.3);
  box-shadow: 0 8px 28px rgba(0, 0, 0, 0.4);
}

body.dark-theme .dialog-title {
  color: #e7e9ed;
}

body.dark-theme .dialog-input {
  background: rgba(24, 26, 27, 0.8);
  color: #e7e9ed;
  border-color: rgba(231, 233, 237, 0.2);
}

body.dark-theme .dialog-input:focus {
  box-shadow: 0 0 6px rgba(0, 122, 255, 0.4);
}

body.dark-theme .dialog-btn.cancel {
  background: rgba(255, 255, 255, 0.05);
  color: #aaa;
}

body.dark-theme .dialog-btn.cancel:hover {
  background: rgba(255, 255, 255, 0.1);
  transform: translateY(-1px);
}

body.dark-theme .dialog-btn.confirm:hover {
  box-shadow: 0 2px 8px rgba(0, 122, 255, 0.4);
}
</style>
