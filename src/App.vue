<script setup lang="ts">
import { useHead } from '@vueuse/head'
import { useNprogress, useAos } from '@/composables'

useHead({
  title: 'Image - Colorlives',
  meta: [
    { name: 'description', content: '图像处理，视频流处理' },
  ],
})

useNprogress()
useAos()

</script>

<template>
  <router-view />
</template>

<style scoped>
</style>
