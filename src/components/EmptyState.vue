<template>
  <div class="empty-state">
    <div class="date-info">
      <div class="solar-date">
        <div class="date-main">{{ dateInfo.solar_date }}</div>
        <div class="weekday">{{ dateInfo.weekday }}</div>
      </div>
      <div class="lunar-date">
        <div class="lunar-main">{{ dateInfo.lunar_date }}</div>
      </div>
    </div>
    <div class="welcome-text">
      🌸 今天也要加油哦！
    </div>
  </div>
</template>

<script setup lang="ts">
import type { DateInfo } from '../../src/types';

interface Props {
  dateInfo: DateInfo;
}

defineProps<Props>();
</script>

<style scoped>
/* 空状态日期信息样式 */
.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: clamp(20px, 4vh, 40px);
  text-align: center;
}

.date-info {
  background: rgba(255, 255, 255, 0.9);
  border-radius: clamp(12px, 2.5vw, 16px);
  padding: clamp(16px, 3vh, 24px);
  box-shadow: 0 6px 20px rgba(104, 58, 183, 0.15);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(104, 58, 183, 0.1);
  margin-bottom: clamp(16px, 3vh, 24px);
  min-width: clamp(200px, 40vw, 260px);
}

.solar-date {
  margin-bottom: clamp(12px, 2vh, 16px);
}

.date-main {
  font-size: clamp(1.2rem, 4vw, 1.5rem);
  font-weight: 700;
  color: #333;
  margin-bottom: clamp(4px, 0.8vh, 6px);
  letter-spacing: 1px;
}

.weekday {
  font-size: clamp(0.85rem, 2.2vw, 1rem);
  color: #683ab7;
  font-weight: 600;
}

.lunar-date {
  border-top: 1px dashed rgba(104, 58, 183, 0.2);
  padding-top: clamp(10px, 2vh, 12px);
}

.lunar-main {
  font-size: clamp(1rem, 3vw, 1.2rem);
  font-weight: 600;
  color: #555;
  margin-bottom: 0;
  font-family: 'Microsoft YaHei', '微软雅黑', sans-serif;
}

.welcome-text {
  font-size: clamp(0.9rem, 2.5vw, 1.1rem);
  color: #683ab7;
  font-weight: 600;
  background: rgba(104, 58, 183, 0.1);
  padding: clamp(8px, 1.5vh, 12px) clamp(16px, 3vw, 20px);
  border-radius: clamp(8px, 1.5vw, 12px);
  border: 1px solid rgba(104, 58, 183, 0.2);
  backdrop-filter: blur(5px);
  box-shadow: 0 2px 8px rgba(104, 58, 183, 0.1);
  animation: gentle-pulse 3s ease-in-out infinite;
}

/* 温和的脉动动画 */
@keyframes gentle-pulse {
  0%, 100% {
    transform: scale(1);
    opacity: 0.9;
  }
  50% {
    transform: scale(1.02);
    opacity: 1;
  }
}
</style>