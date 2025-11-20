<template>
  <div class="todo-list-wrapper">
    <draggable 
      v-model="localTodos"
      item-key="id"
      class="todo-list"
      :animation="250"
      handle=".drag-handle"
      ghost-class="ghost"
      chosen-class="chosen"
      drag-class="drag"
      :force-fallback="true"
      @start="onDragStart"
      @end="onDragEnd"
    >
      <template #item="{ element, index }">
        <div class="todo-item-wrapper">
          <TodoItem
            :todo="element"
            :index="index"
            :is-completed-list="props.isCompletedList"
            @toggle="toggleTodo"
            @delete="deleteTodo"
            @contextmenu="showContextMenu"
          />
        </div>
      </template>
    </draggable>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import draggable from 'vuedraggable';
import TodoItem from './TodoItem.vue';
import type { Todo } from '../types';

interface Props {
  todos: Todo[];
  isCompletedList?: boolean;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  toggle: [index: number];
  delete: [index: number];
  contextmenu: [event: MouseEvent, todo: Todo];
  reorder: [newOrder: Todo[]];
}>();

const localTodos = computed({
  get: () => props.todos,
  set: (value) => {
    emit('reorder', value);
  }
});

function toggleTodo(index: number) {
  emit('toggle', index);
}

function deleteTodo(index: number) {
  emit('delete', index);
}

function showContextMenu(event: MouseEvent, todo: Todo) {
  emit('contextmenu', event, todo);
}

function onDragStart(evt: any) {
  // 拖拽开始
}

function onDragEnd(evt: any) {
  // 拖拽结束
}
</script>

<style scoped>
.todo-list-wrapper {
  width: 100%;
}

.todo-list {
  min-height: 50px;
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 0;
}

.todo-item-wrapper {
  width: 100%;
}

/* 拖动时的占位符 - 保持原样 */
.ghost {
  opacity: 1 !important;
}

/* 被选中准备拖动的项 */
.chosen {
  opacity: 1;
}

/* 正在拖动中的项 - 完全隐藏 */
.drag {
  opacity: 0 !important;
  visibility: hidden !important;
}
</style>
