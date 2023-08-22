<script lang="ts" setup>
const props = withDefaults(
  defineProps<{
    source?: string
    closable?: boolean
  }>(),
  {
    closable: false
  }
)

const emits = defineEmits<{
  (e: 'close'): void
}>()

function handleClose() {
  emits('close')
}
</script>

<template>
  <div class="image-box group">
    <!-- -->
    <div class="ib-close-button" v-if="closable" @click="handleClose">
      <i-ant-design-close-outlined class="" />
    </div>
    <img v-if="source" :src="source" alt="">
  </div>
</template>

<style lang="postcss">
.image-box {
  @apply w-40 h-40 flex justify-center items-center bg-gray-300 group relative overflow-hidden;

  & img {
    @apply object-fill select-none;
  }
}

.ib-close-button {
  @apply absolute right-1 top-1 cursor-pointer text-sm text-gray-600 invisible group-hover:visible;
}


</style>
