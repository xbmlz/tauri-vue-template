<script lang="ts" setup>
import Titlebar from './components/Titlebar.vue'
import Sidebar from './components/Sidebar.vue'
</script>

<template>
  <a-layout
    text="center gray-700 dark:gray-200"
    mt="[var(--titlebar-height)]"
    h-full
  >
    <!-- header -->
    <a-layout-header>
      <Titlebar />
    </a-layout-header>
    <!-- main -->
    <a-layout>
      <!-- sidebar -->
      <a-layout-sider :width="50">
        <Sidebar />
      </a-layout-sider>

      <!-- content -->
      <a-layout-content style="padding: 1rem;">
        <RouterView />
      </a-layout-content>
    </a-layout>
  </a-layout>
</template>
