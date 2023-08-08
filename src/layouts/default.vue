<script lang="ts" setup>
  
</script>
<template>
<div class="default-layout">
  <header class="h-20 bg-white bg-opacity-10">
    <div class="max-w-7xl mx-auto h-full flex items-center">
      <a href="/">
        <img src="/image-logo.png" alt="logo">
      </a>
    </div>

  </header>
  <div class="max-w-7xl mx-auto">
    <router-view></router-view>
  </div>
</div>
</template>
<style lang="postcss">
.default-layout {

}
</style>