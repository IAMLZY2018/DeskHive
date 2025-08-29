<template>
  <div class="todo-list">
    <TransitionGroup :name="isCompletedList ? 'completed-list' : 'pending-list'" tag="div">
      <TodoItem
        v-for="(todo, index) in todos"
        :key="`${isCompletedList ? 'completed' : 'pending'}-${index}`"
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
.pending-list-enter-active,
.pending-list-leave-active,
.completed-list-enter-active,
.completed-list-leave-active {
  transition: all 0.4s cubic-bezier(0.25, 0.8, 0.25, 1);
}

.pending-list-enter-from {
  opacity: 0;
  transform: translateY(20px);
}

.pending-list-leave-to {
  opacity: 0;
  transform: translateY(-20px);
}

.completed-list-enter-from {
  opacity: 0;
  transform: translateY(20px);
}

.completed-list-leave-to {
  opacity: 0;
  transform: translateY(-20px);
}

.pending-list-move,
.completed-list-move {
  transition: transform 0.4s cubic-bezier(0.25, 0.8, 0.25, 1);
}
</style>