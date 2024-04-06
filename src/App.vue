<script setup lang="ts">
import { globalShortcut, window } from '@tauri-apps/api'

// https://github.com/vueuse/head
// you can use this to manipulate the document head in any components,
// they will be rendered correctly in the html results with vite-ssg
useHead({
  title: 'Vitesse',
  meta: [
    {
      name: 'description',
      content: 'Opinionated Vite Starter Template',
    },
    {
      name: 'theme-color',
      content: '#ffffff',
    },
  ],
  link: [
    {
      rel: 'icon',
      type: 'image/svg+xml',
      href: '/favicon.svg',
    },
  ],
})

onMounted(async () => {
  if (await globalShortcut.isRegistered('F11'))
    globalShortcut.unregister('F11')

  globalShortcut.register('F11', async (evt) => {
    window.getCurrent().hide()
  })
})
</script>

<template>
  <RouterView />
</template>
