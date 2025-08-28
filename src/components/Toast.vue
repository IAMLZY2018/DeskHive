<template>
  <div v-if="show" :class="['toast-notification', `toast-${type}`]">
    <div class="toast-content">
      <span class="toast-icon">
        <span v-if="type === 'success'">✓</span>
        <span v-else-if="type === 'warning'">⚠️</span>
        <span v-else>⚠️</span>
      </span>
      <span class="toast-message">{{ message }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  show: boolean;
  message: string;
  type: 'success' | 'warning' | 'error';
}

defineProps<Props>();
</script>

<style scoped>
/* Toast 内部提示样式 */
.toast-notification {
  position: fixed;
  top: 20px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 3000;
  padding: 12px 20px;
  border-radius: 8px;
  backdrop-filter: blur(10px);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);
  font-size: 0.9rem;
  font-weight: 600;
  animation: toast-slide-in 0.3s ease-out;
  max-width: 300px;
  min-width: 200px;
}

.toast-success {
  background: rgba(76, 175, 80, 0.95);
  color: white;
  border: 1px solid rgba(76, 175, 80, 0.8);
}

.toast-error {
  background: rgba(244, 67, 54, 0.95);
  color: white;
  border: 1px solid rgba(244, 67, 54, 0.8);
}

.toast-warning {
  background: rgba(255, 193, 7, 0.95);
  color: #333;
  border: 1px solid rgba(255, 193, 7, 0.8);
}

.toast-content {
  display: flex;
  align-items: center;
  gap: 8px;
}

.toast-icon {
  font-size: 1.1rem;
  flex-shrink: 0;
}

.toast-message {
  flex: 1;
}

/* Toast 动画 */
@keyframes toast-slide-in {
  0% {
    opacity: 0;
    transform: translateX(-50%) translateY(-20px);
  }
  100% {
    opacity: 1;
    transform: translateX(-50%) translateY(0);
  }
}
</style>