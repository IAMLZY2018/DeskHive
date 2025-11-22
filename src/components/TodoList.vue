<template>
  <div class="todo-list-wrapper">
    <draggable 
      v-model="localTodos"
      item-key="id"
      :class="['todo-list', { 'with-border': props.showBorder }]"
      group="todos"
      :animation="250"
      handle=".drag-handle"
      ghost-class="ghost"
      chosen-class="chosen"
      drag-class="drag"
      :force-fallback="true"
      @start="onDragStart"
      @end="onDragEnd"
      @change="onChange"
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
  showBorder?: boolean;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  toggle: [index: number];
  delete: [index: number];
  contextmenu: [event: MouseEvent, todo: Todo];
  reorder: [newOrder: Todo[]];
  'drag-start': [todo: Todo];
  'drag-end': [];
  'change': [event: any];
}>();

const localTodos = computed({
  get: () => props.todos,
  set: (value) => {
    // 触发 reorder 事件，传递新的顺序
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
  // 拖拽开始，通知父组件
  const draggedTodo = props.todos[evt.oldIndex];
  if (draggedTodo) {
    emit('drag-start', draggedTodo);
  }
}

function onDragEnd(evt: any) {
  // 拖拽结束，通知父组件
  emit('drag-end');
}

function onChange(evt: any) {
  console.log('TodoList onChange:', evt);
  // 拖拽变化事件（添加、移除或移动）
  emit('change', evt);
}
</script>

<style scoped>
.todo-list-wrapper {
  width: 100%;
}

.todo-list {
  min-height: 60px;
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 0;
  position: relative;
  z-index: 1;
}

.todo-list.with-border {
  background: transparent;
  border-radius: clamp(6px, 1.2vw, 8px);
  border: 1px dashed transparent;
  padding: clamp(2px, 0.5vh, 3px);
  transition: background 0.3s ease, border-color 0.3s ease;
}

/* 拖动悬停时的高亮效果 */
.todo-list.with-border:has(.sortable-drag) {
  background: rgba(59, 130, 246, 0.05);
  border-color: rgba(59, 130, 246, 0.3);
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

/* 夜间主题 */
body.dark-theme .todo-list.with-border {
  background: rgba(255, 255, 255, 0.02);
  border-color: rgba(255, 255, 255, 0.1);
}
</style>
