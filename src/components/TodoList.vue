<template>
  <div class="todo-list">
    <TransitionGroup name="todo-list" tag="div">
      <TodoItem
        v-for="(todo, index) in todos"
        :key="index"
        :todo="todo"
        @toggle="() => onToggle(index)"
        @delete="() => onDelete(index)"
        @contextmenu="(event) => onContextMenu(event, todo)"
      />
    </TransitionGroup>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { Todo } from '../../src/types';
import TodoItem from './TodoItem.vue';

interface Props {
  todos: Todo[];
  isCompletedList?: boolean;
}

interface Emits {
  (e: 'toggle', index: number): void;
  (e: 'delete', index: number): void;
  (e: 'contextmenu', event: MouseEvent, todo: Todo): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

function onToggle(index: number) {
  emit('toggle', index);
}

function onDelete(index: number) {
  emit('delete', index);
}

function onContextMenu(event: MouseEvent, todo: Todo) {
  emit('contextmenu', event, todo);
}
</script>

<style scoped>
.todo-list {
  min-height: 0;
}

/* Todo列表过渡动画 */
.todo-list-enter-active,
.todo-list-leave-active {
  transition: all 0.5s ease;
}

.todo-list-enter-from {
  opacity: 0;
  transform: translateY(30px);
}

.todo-list-leave-to {
  opacity: 0;
  transform: translateX(100px);
}

.todo-list-move {
  transition: transform 0.5s ease;
}
</style>