<script setup lang="ts">
import { appWindow } from '@tauri-apps/api/window'
</script>

<template>
  <div data-tauri-drag-region class="titlebar" @click="appWindow.minimize()">
    <div id="titlebar-minimize" class="titlebar-button">
      <div class="i-mdi:window-minimize" />
    </div>
    <div id="titlebar-maximize" class="titlebar-button" @click="appWindow.toggleMaximize()">
      <div class="i-mdi:window-maximize" />
    </div>
    <div id="titlebar-close" class="titlebar-button" @click="() => appWindow.close()">
      <div class="i-mdi:window-close" />
    </div>
  </div>
</template>

<style scoped>
.titlebar {
  height: var(--titlebar-height);
  background: var(--titlebar-bg);
  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  border-bottom: 1px solid var(--color-border);
}
.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  color: var(--titlebar-color);
  width: var(--titlebar-height);
  height: var(--titlebar-height);
}
.titlebar-button:hover {
  background: var(--titlebar-hover-bg);
}
</style>
