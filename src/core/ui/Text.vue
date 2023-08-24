<script lang="ts" setup>
const props = withDefaults(
  defineProps<{
    label?: string,
    modelValue?: string
  }>(),
  {
    label: '',
    modelValue: ''
  }
)

const emits = defineEmits<{
  (e: 'update:modelValue', val: string) : void
}>()

function handleModelValueChange(val: string) {
  emits('update:modelValue', val)
}
</script>

<template>
  <div class="feat-opt-text">
    <!-- -->
    <label for="">
      <span>{{ label }}</span>:
      <input 
        type="text"
        :value="modelValue"
        @input="(e: any) => handleModelValueChange(e.target.value)">
    </label>
  </div>
</template>

<style lang="postcss">
.feat-opt-text {
  @apply;
}
</style>
