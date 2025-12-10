<template>
  <div ref="wrapperRef" class="tooltip-wrapper" @mouseenter="handleMouseEnter" @mouseleave="handleMouseLeave">
    <slot></slot>
    <Transition name="tooltip-fade">
      <div 
        v-if="showTooltip" 
        class="tooltip" 
        :class="{ 'tooltip-below': showBelow, 'tooltip-above': !showBelow }" 
        :style="tooltipStyle"
      >
        {{ text }}
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';

interface Props {
  text: string;
  delay?: number;
}

const props = withDefaults(defineProps<Props>(), {
  delay: 300
});

const showTooltip = ref(false);
const tooltipTimer = ref<number | null>(null);

function handleMouseLeave() {
  if (tooltipTimer.value) {
    clearTimeout(tooltipTimer.value);
    tooltipTimer.value = null;
  }
  showTooltip.value = false;
}

const wrapperRef = ref<HTMLElement | null>(null);
const showBelow = ref(false);

function handleMouseEnter() {
  // 先检测位置
  if (wrapperRef.value) {
    const rect = wrapperRef.value.getBoundingClientRect();
    const tooltipHeight = 60; // 估计的提示框高度（增加更多余量以避免遮挡）
    const spaceAbove = rect.top;
    
    // 如果上方空间不足，显示在下方
    showBelow.value = spaceAbove < tooltipHeight;
  }
  
  tooltipTimer.value = window.setTimeout(() => {
    showTooltip.value = true;
  }, props.delay);
}

const tooltipStyle = computed(() => ({}));
</script>

<style scoped>
.tooltip-wrapper {
  position: relative;
  display: inline-flex;
}

.tooltip {
  position: absolute;
  background: linear-gradient(135deg, rgba(30, 30, 30, 0.95) 0%, rgba(20, 20, 20, 0.95) 100%);
  color: #fff;
  padding: 6px 12px;
  border-radius: 8px;
  font-size: 0.7rem;
  line-height: 1.4;
  white-space: nowrap;
  z-index: 10000000;
  pointer-events: none;
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.25), 0 0 0 1px rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(12px);
  text-align: left;
  right: 0;
  font-weight: 500;
  letter-spacing: 0.3px;
}

.tooltip.tooltip-above {
  bottom: calc(100% + 10px);
}

.tooltip.tooltip-below {
  top: calc(100% + 10px);
}

.tooltip::before {
  content: '';
  position: absolute;
  left: auto;
  right: 12px;
  width: 0;
  height: 0;
  border: 5px solid transparent;
}

/* 显示在上方时 */
.tooltip.tooltip-above::before {
  top: 100%;
  border-top-color: rgba(20, 20, 20, 0.95);
}

/* 显示在下方时 */
.tooltip.tooltip-below::before {
  bottom: 100%;
  border-bottom-color: rgba(30, 30, 30, 0.95);
}

.tooltip-fade-enter-active {
  transition: opacity 0.25s cubic-bezier(0.4, 0, 0.2, 1), 
              transform 0.25s cubic-bezier(0.4, 0, 0.2, 1);
}

.tooltip-fade-leave-active {
  transition: opacity 0.15s ease, 
              transform 0.15s ease;
}

.tooltip-fade-enter-from {
  opacity: 0;
  transform: translateY(-4px) scale(0.95);
}

.tooltip-fade-leave-to {
  opacity: 0;
  transform: translateY(2px);
}

.tooltip-fade-enter-to,
.tooltip-fade-leave-from {
  opacity: 1;
  transform: translateY(0) scale(1);
}

/* 夜间主题 */
body.dark-theme .tooltip {
  background: linear-gradient(135deg, rgba(245, 245, 245, 0.98) 0%, rgba(255, 255, 255, 0.98) 100%);
  color: #1a1a1a;
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.3), 0 0 0 1px rgba(0, 0, 0, 0.1);
}

body.dark-theme .tooltip.tooltip-above::before {
  border-top-color: rgba(255, 255, 255, 0.98);
}

body.dark-theme .tooltip.tooltip-below::before {
  border-bottom-color: rgba(245, 245, 245, 0.98);
}
</style>
