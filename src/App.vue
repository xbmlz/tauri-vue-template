<template>
  <TheTitlebar />
  <main font-sans p="x-4 y-10" text="center gray-700 dark:gray-200" mt="~ [var(--titlebar-height)]">
    <a-config-provider>
      <RouterView />
    </a-config-provider>
    <TheFooter />
  </main>
</template>
