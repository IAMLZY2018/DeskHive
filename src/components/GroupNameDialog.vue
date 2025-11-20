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
  border-radius: 12px;
  padding: 20px;
  min-width: 300px;
  max-width: 400px;
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(229, 231, 235, 0.3);
}

.dialog-title {
  font-size: 1.1rem;
  font-weight: 600;
  color: #333;
  margin-bottom: 16px;
}

.dialog-input {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid rgba(229, 231, 235, 0.3);
  border-radius: 8px;
  outline: none;
  background: rgba(255, 255, 255, 0.8);
  color: #333;
  font-size: 0.9rem;
  transition: all 0.3s ease;
  margin-bottom: 16px;
}

.dialog-input:focus {
  border-color: #007aff;
  box-shadow: 0 0 8px rgba(0, 122, 255, 0.3);
  background: rgba(255, 255, 255, 0.95);
}

.dialog-actions {
  display: flex;
  gap: 10px;
  justify-content: flex-end;
}

.dialog-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.85rem;
  font-weight: 500;
  transition: all 0.2s ease;
}

.dialog-btn.cancel {
  background: rgba(0, 0, 0, 0.05);
  color: #666;
}

.dialog-btn.cancel:hover {
  background: rgba(0, 0, 0, 0.1);
}

.dialog-btn.confirm {
  background: #007aff;
  color: white;
}

.dialog-btn.confirm:hover {
  background: #0051d5;
}

/* 夜间主题 */
body.dark-theme .dialog {
  background: rgba(37, 38, 39, 0.95);
  border-color: rgba(231, 233, 237, 0.3);
}

body.dark-theme .dialog-title {
  color: #e7e9ed;
}

body.dark-theme .dialog-input {
  background: rgba(24, 26, 27, 0.8);
  color: #e7e9ed;
  border-color: rgba(231, 233, 237, 0.2);
}

body.dark-theme .dialog-btn.cancel {
  background: rgba(255, 255, 255, 0.05);
  color: #aaa;
}

body.dark-theme .dialog-btn.cancel:hover {
  background: rgba(255, 255, 255, 0.1);
}
</style>
