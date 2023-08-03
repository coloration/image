<script lang="ts" setup>
import { ref } from 'vue'

const props = withDefaults(
  defineProps<{
    activeCondition?: boolean,
  }>(),
  {}
)

const expanded = ref(props.activeCondition)

const handleClick = () => {
  expanded.value = !expanded.value
}
</script>
<template>
  <li class="sidebar-link-group">
    <slot :handleClick="handleClick" :expanded="expanded" />
  </li>
</template>
<style lang="postcss">
.sidebar-link-group {
  @apply mb-1;
}
</style>