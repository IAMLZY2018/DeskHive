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
    const tooltipHeight = 40; // 估计的提示框高度
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
  z-index: 1;
}

.tooltip {
  position: absolute;
  background: rgba(0, 0, 0, 0.85);
  color: white;
  padding: 5px 10px;
  border-radius: 6px;
  font-size: 0.65rem;
  line-height: 1.3;
  white-space: nowrap;
  z-index: 99999;
  pointer-events: none;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(8px);
  text-align: left;
  right: 0;
}

.tooltip.tooltip-above {
  bottom: calc(100% + 8px);
}

.tooltip.tooltip-below {
  top: calc(100% + 8px);
}

.tooltip::after {
  content: '';
  position: absolute;
  left: auto;
  right: 10px;
  border: 4px solid transparent;
}

/* 显示在上方时 */
.tooltip.tooltip-above::after {
  top: 100%;
  border-top-color: rgba(0, 0, 0, 0.85);
}

/* 显示在下方时 */
.tooltip.tooltip-below::after {
  bottom: 100%;
  border-bottom-color: rgba(0, 0, 0, 0.85);
}

.tooltip-fade-enter-active,
.tooltip-fade-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.tooltip-fade-enter-from,
.tooltip-fade-leave-to {
  opacity: 0;
  transform: translateY(4px);
}

.tooltip-fade-enter-to,
.tooltip-fade-leave-from {
  opacity: 1;
  transform: translateY(0);
}

/* 夜间主题 */
body.dark-theme .tooltip {
  background: rgba(255, 255, 255, 0.9);
  color: #333;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
}

body.dark-theme .tooltip.tooltip-above::after {
  border-top-color: rgba(255, 255, 255, 0.9);
}

body.dark-theme .tooltip.tooltip-below::after {
  border-bottom-color: rgba(255, 255, 255, 0.9);
}
</style>
