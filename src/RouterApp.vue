<template>
  <router-view />
</template>

<script setup lang="ts">
// 这是主应用的路由包装器
</script>

<style>
html, body, #app {
  width: 100%;
  height: 100%;
  overflow: hidden;
  margin: 0;
  padding: 0;
}

body {
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  background: #181a1b;
  color: #e7e9ed;
}

/* 夜间主题样式 */
body.dark-theme {
  background: #181a1b;
  color: #e7e9ed;
}

body.dark-theme .container {
  background: #181a1b;
  color: #e7e9ed;
}

/* 设置页面的夜间主题样式 */
body.dark-theme .sidebar {
  background: #252627;
  border-right: 1px solid #444b4f;
}

body.dark-theme .sidebar-header {
  border-bottom: 1px solid #444b4f;
}

body.dark-theme .sidebar-header h1 {
  color: #e7e9ed;
}

body.dark-theme .menu-item {
  color: #e7e9ed;
}

body.dark-theme .menu-item:hover {
  background: #3d4549;
}

body.dark-theme .menu-item.active {
  background: #007aff;
  color: #ffffff;
}

body.dark-theme .content-header {
  border-bottom: 1px solid #444b4f;
}

body.dark-theme .content-header h2 {
  color: #e7e9ed;
}

body.dark-theme .setting-group {
  background: #252627;
  border: 1px solid #444b4f;
}

body.dark-theme .setting-item {
  border-bottom: 1px solid #444b4f;
}

body.dark-theme .setting-item:last-child {
  border-bottom: none;
}

body.dark-theme .setting-label {
  color: #e7e9ed;
}

body.dark-theme .setting-description {
  color: #a0a6aa;
}

body.dark-theme .content-footer {
  background: #252627;
  border-top: 1px solid #444b4f;
}

body.dark-theme .btn-secondary {
  background: #252627;
  color: #007aff;
  border: 1px solid #007aff;
}

body.dark-theme .btn-secondary:hover {
  background: #3d4549;
}

/* 主应用的夜间主题样式 */
body.dark-theme header {
  background: rgba(24, 26, 27, 0.8);
  border-bottom: 1px solid rgba(231, 233, 237, 0.3);
  color: #e7e9ed;
}

body.dark-theme .progress-indicator {
  background: rgba(24, 26, 27, 0.9);
  color: #e7e9ed;
  border: 1px solid rgba(231, 233, 237, 0.2);
}

body.dark-theme .settings-btn {
  background: rgba(24, 26, 27, 0.9);
  color: #e7e9ed;
  border: 1px solid rgba(231, 233, 237, 0.2);
}

body.dark-theme .todo-item {
  background: #252627;
  border: 1px solid rgba(231, 233, 237, 0.3);
  color: #e7e9ed;
  border-radius: 10px;
}

body.dark-theme .todo-item:hover {
  border-color: rgba(231, 233, 237, 0.5);
}

body.dark-theme .todo-checkbox {
  background: rgba(24, 26, 27, 0.8);
  border: 1px solid rgba(231, 233, 237, 0.2);
}

body.dark-theme .todo-checkbox.completed {
  background: rgba(24, 26, 27, 0.9);
  border-color: rgba(76, 175, 80, 0.8);
}

body.dark-theme .todo-checkbox.completed::after {
  color: #e7e9ed;
}

body.dark-theme .todo-item span {
  color: #e7e9ed;
}

body.dark-theme .todo-item.completed span {
  color: #a0a6aa;
}

body.dark-theme .add-task {
  background: rgba(24, 26, 27, 0.8);
  border-top: 1px solid rgba(231, 233, 237, 0.3);
}

body.dark-theme .add-task input {
  background: #252627;
  border: 1px solid rgba(231, 233, 237, 0.5);
  color: #e7e9ed;
  border-radius: 10px;
}

body.dark-theme .add-task input::placeholder {
  color: rgba(231, 233, 237, 0.5);
}

body.dark-theme .add-task button {
  background: #252627;
  color: #e7e9ed;
  border: 1px solid rgba(231, 233, 237, 0.2);
  border-radius: 50%;
}

body.dark-theme .section-title {
  color: #a0a6aa;
}

body.dark-theme .completed-section {
  border-top: 1px dashed rgba(231, 233, 237, 0.5);
}

body.dark-theme .completed-count {
  background: #4CAF50;
  color: white;
}

body.dark-theme .clear-completed-btn {
  color: #a0a6aa;
}

body.dark-theme .clear-completed-btn:hover {
  background: rgba(244, 67, 54, 0.2);
  color: #f44336;
}
</style>