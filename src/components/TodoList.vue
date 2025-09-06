<template>
  <div class="todo-list">
    <TransitionGroup 
      :name="props.isCompletedList ? 'completed-list' : 'pending-list'" 
      tag="div"
      move-class="todo-item-move"
    >
      <TodoItem
        v-for="(todo, index) in props.todos"
        :key="todo.id"
        :todo="todo"
        :index="index"
        :is-completed-list="props.isCompletedList"
        @toggle="toggleTodo"
        @delete="deleteTodo"
        @contextmenu="showContextMenu"
      />
    </TransitionGroup>
  </div>
</template>

<script setup lang="ts">
import TodoItem from './TodoItem.vue';
import type { Todo } from '../../src/types';

interface Props {
  todos: Todo[];
  isCompletedList?: boolean;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  toggle: [index: number];
  delete: [index: number];
  contextmenu: [event: MouseEvent, todo: Todo];
}>();

// 切换任务完成状态
function toggleTodo(index: number) {
  emit('toggle', index);
}

// 删除任务
function deleteTodo(index: number) {
  emit('delete', index);
}

// 显示右键菜单
function showContextMenu(event: MouseEvent, todo: Todo) {
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