<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const greetMsg = ref('')
const name = ref('')

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke('greet', { name: name.value })
}
</script>

<template>
  <a-space direction="vertical" size="large">
    <h1 class="text-4xl">
      Tauri App
    </h1>
    <p>Welcome to your Tauri app!</p>
    <a-space>
      <a-input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <a-button type="primary" @click="greet()">
        Greet
      </a-button>
    </a-space>
    <p>{{ greetMsg }}</p>
  </a-space>
</template>
