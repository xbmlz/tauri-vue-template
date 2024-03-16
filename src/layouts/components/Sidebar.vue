<script lang="ts" setup>
const activedTabIndex = ref(0)

const menus = computed(() => {
  return [
    {
      label: 'menu.home',
      key: 'home',
      class: 'i-carbon-home',
    },
    {
      label: 'menu.home',
      key: 'database',
      class: 'i-carbon-db2-database',
    },
  ]
})
</script>

<template>
  <div class="h-full flex flex-col b-r b-[var(--color-border)] bg-[var(--siderbar-bg)] px-1 py-2">
    <a-tooltip v-for="(m, i) in menus" :key="i" position="right" :content="m.label" mini>
      <div
        class="flex flex-col cursor-pointer items-center justify-center gap-3 rd-md px-3 py-2 hover:bg-[var(--siderbar-hover)]"
        :class="{ 'menu-item-active': activedTabIndex === i }"
        @click="activedTabIndex = i"
      >
        <div :class="`${m.class} text-xl`" />
      </div>
    </a-tooltip>

    <div class="flex-grow-1" />

    <div class="flex flex-col items-center justify-center gap-3 px-3 py-2 text-xl">
      <a-tooltip position="right" content="theme" mini>
        <button @click="toggleDark()">
          <div class="i-carbon-sun dark:i-carbon-moon" />
        </button>
      </a-tooltip>

      <a-tooltip position="right" content="settings" mini>
        <button>
          <div class="i-carbon-settings" />
        </button>
      </a-tooltip>

      <a
        class="i-mdi:github" rel="noreferrer" href="https://github.com/xbmlz/tauri-vue-starter"
        target="_blank" title="GitHub"
      />
    </div>
  </div>
</template>

<style scoped lang="less">
.menu-item-active {
  position: relative;
  display: flex;
  justify-content: center;
  align-items: center;

  &::after {
    content: '';
    width: 3px;
    height: 50%;
    border-radius: 10px;
    position: absolute;
    left: 0;
    background-color: rgb(var(--primary-5));
  }
}
</style>
