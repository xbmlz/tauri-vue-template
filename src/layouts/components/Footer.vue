<template>
  <nav class="mt-6 inline-flex gap-2 text-xl">
    <button @click="toggleDark()">
      <div class="i-carbon-sun dark:i-carbon-moon" />
    </button>

    <a
      class="i-mdi:github" rel="noreferrer" href="https://github.com/xbmlz/tauri-vue-starter"
      target="_blank" title="GitHub"
    />
  </nav>
</template>
