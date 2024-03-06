<template>
  <main
    text="center gray-700 dark:gray-200"
    mt="[var(--titlebar-height)]"
    h-full px-4 py-10
  >
    <RouterView />
    <TheFooter />
    <div mx-auto mt-5 text-center text-sm opacity-50>
      [Default Layout]
    </div>
  </main>
</template>
