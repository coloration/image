<script lang="ts" setup>
const props = withDefaults(
  defineProps<{
    disabled?: boolean
  }>(),
  {
    disabled: false
  }
)

const emits = defineEmits<{
  (e: 'click'): void
}>()

function handleClick() {
  if (props.disabled) return
  emits('click')
}
</script>

<template>
  <div
    class="opt-button"
    :class="{ disabled }"
    @click="handleClick">
    <!-- -->
    <slot />
  </div>
</template>

<style lang="postcss">
.opt-button {
  @apply text-white font-500 text-sm px-3 py-1 bg-blue-500 cursor-pointer select-none hover:bg-blue-400 transform-gpu transition-all;
  border-radius: 0 0.5rem;

  &.disabled {
    @apply bg-gray-400 hover:bg-gray-400 cursor-not-allowed;
  }
}

</style>
