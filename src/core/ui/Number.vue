<script lang="ts" setup>
import { isNumber } from '@vueuse/core'

const props = withDefaults(
  defineProps<{
    label?: string,
    modelValue?: number
    max?: number
    min?: number
    fix?: number
  }>(),
  {
    label: '',
    modelValue: 0,
    
  }
)

const emits = defineEmits<{
  (e: 'update:modelValue', val: number) : void
}>()

function handleModelValueChange(val: string) {
  let v = Number(val)
  if (isNaN(v)) v = 0
  if (isNumber(props.max) && v > props.max) v = props.max
  if (isNumber(props.min) && v < props.min) v = props.min
  if (isNumber(props.fix)) v = Number(v.toFixed(props.fix))


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
        class="w-10 text-center rounded border-1 border-blue-700 ml-2"
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
