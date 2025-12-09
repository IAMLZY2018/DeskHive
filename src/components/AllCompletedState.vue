<template>
  <div class="all-completed-state">
    <div class="celebration-message">
      🎉 太棒啦~所有任务都完成啦~ 🎉
    </div>
  </div>
</template>

<script setup lang="ts">
</script>

<style scoped>
/* 全部完成状态样式 */
.all-completed-state {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: clamp(30px, 5vh, 50px);
}

.celebration-message {
  font-size: clamp(1.1rem, 3vw, 1.3rem);
  font-weight: 700;
  color: #6EE748;
  background: linear-gradient(135deg, rgba(110, 231, 72, 0.1), rgba(110, 231, 72, 0.2));
  padding: clamp(16px, 3vh, 24px) clamp(20px, 4vw, 32px);
  border-radius: clamp(12px, 2.5vw, 16px);
  border: 2px solid rgba(76, 175, 80, 0.3);
  backdrop-filter: blur(10px);
  box-shadow: 0 8px 24px rgba(76, 175, 80, 0.2);
  text-align: center;
  animation: celebration-bounce 0.8s ease-out;
  cursor: pointer;
  transition: all 0.3s ease;
}

.celebration-message:hover {
  animation: celebration-bounce 0.8s ease-out;
  box-shadow: 0 10px 28px rgba(76, 175, 80, 0.3);
  transform: translateY(-2px);
}

/* 庆祝弹跳动画 */
@keyframes celebration-bounce {
  0%, 20%, 50%, 80%, 100% {
    transform: translateY(0) scale(1);
  }
  40% {
    transform: translateY(-8px) scale(1.05);
  }
  60% {
    transform: translateY(-4px) scale(1.02);
  }
}
</style>