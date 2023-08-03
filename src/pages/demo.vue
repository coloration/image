<script lang="ts" setup>
  
</script>
<template>
<div class="demo">
I'm 'demo'
<p v-for="i in 10" :key="i" data-aos="fade-up" :data-aos-delay="i * 100">
  {{ i }}
</p>

<i-ant-design-menu-unfold-outlined />
</div>
</template>
<style lang="postcss">
.demo {
  @apply;
}
</style>
