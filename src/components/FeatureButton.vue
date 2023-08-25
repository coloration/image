<script lang="ts" setup>
const props = withDefaults(
  defineProps<{
    size?: 'md' | 'lg',
    badge?: 'purple' | 'blue' | 'red' | 'green' | 'orange'
    disabled?: boolean
    closable?: boolean
    title: string
  }>(),
  {
    size: 'md',
    badge: 'blue',
    disabled: false,
    closable: false
  }
)

const emits = defineEmits<{
  (e: 'close'): void
}>()

function close() {
  emits('close')
}

</script>

<template>
  <div
    class="feature-button group" 
    :class="[size, disabled ? 'disabled' : '']"
  >
    <!-- -->
    <div
      class="fb-close invisible group-hover:visible"
      v-if="closable"
      @click="close">
      <i-ant-design-close-outlined class="" />
    </div>
    <div class="fb-contain">
      <div class="fb-badge" :class="badge">

      </div>
      <div class="fb-content">
        <div class="select-none">{{ title }}</div>
        <slot></slot>
      </div>
    </div>
  </div>
</template>

<style lang="postcss">
.feature-button {
  @apply px-2 py-2 bg-white relative;
  border-radius: 8px 0;

  &.lg {
    @apply py-3;
  }

  &.disabled {
    @apply bg-gray-200 bg-opacity-70 cursor-not-allowed;
  }
}

.fb-contain {

  @apply flex gap-2 w-full;
}

.fb-content {
  @apply flex-1;
}

.fb-close {
  @apply absolute right-1 top-1 text-sm text-gray-400 cursor-pointer;
}

.fb-badge {
  @apply w-5px h-6;
  border-radius: 6px 0;

  &.purple {
    @apply bg-purple-500;
  }
  &.blue {
    @apply bg-blue-500;
  }
  &.red {
    @apply bg-red-500;
  }
  &.green {
    @apply bg-green-500;
  }
  &.orange {
    @apply bg-orange-500;
  }
}
</style>
