<script lang="ts" setup>
import { isNumber } from '@vueuse/core'

const props = withDefaults(
  defineProps<{
    label?: string,
    modelValue?: number
    max?: number
    min?: number
  }>(),
  {
    label: '',
    modelValue: 0
  }
)

const emits = defineEmits<{
  (e: 'update:modelValue', val: number) : void
}>()

function handleModelValueChange(val: string) {
  let v = Number(val)
  if (isNaN(v)) v = 0
  else if (isNumber(props.max) && v > props.max) v = props.max
  else if (isNumber(props.min) && v < props.min) v = props.min
  emits('update:modelValue', v)
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
