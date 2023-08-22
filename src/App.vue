<script setup lang="ts">
import { useHead } from '@vueuse/head'
import { useNprogress, useAos } from '@/composables'

useHead({
  title: 'Image - Colorlives',
  meta: [
    { name: 'description', content: '图像处理，视频流处理，图像格式转换，图像二值化，图像灰度化' },
  ],
})

useNprogress()
useAos()

</script>

<template>
  <router-view />
</template>

<style>
html, body {
  background-color: #5996CE !important;
}
</style>
