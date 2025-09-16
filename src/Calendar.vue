<template>
  <div class="calendar-page">
    <CalendarView />
  </div>
</template>

<script setup lang="ts">
import CalendarView from './components/CalendarView.vue';
</script>

<style>
.calendar-page {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
}

html, body {
  width: 100%;
  height: 100%;
  margin: 0;
  padding: 0;
  overflow: hidden;
}
</style>