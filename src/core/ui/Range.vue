<script lang="ts" setup>
import { isNumber } from '@vueuse/core'

const props = withDefaults(
  defineProps<{
    label?: string,
    modelValue?: number
    max: number
    min: number
  }>(),
  {
    label: '',
    modelValue: 0,
  }
)

const emits = defineEmits<{
  (e: 'update:modelValue', val: number): void
}>()

function handleModelValueChange(val: string) {
  let v = Number(val.replace(/\D+/g, ''))
  if (isNaN(v)) v = 0
  else if (isNumber(props.max) && v > props.max) v = props.max
  else if (isNumber(props.min) && v < props.min) v = props.min

  
  emits('update:modelValue', v)
}
</script>

<template>
  <FeatureOptEditContainer>
    <!-- -->
    <label class="feat-opt-range-label">
      <div class="flex">
        <span>{{ label }}: </span>
        <input
          class="w-10 text-center rounded border-1 border-blue-700 ml-2"
          type="text"
          :value="modelValue"
          @input="(e: any) => handleModelValueChange(e.target.value)"
        >
      </div>
      <input
        type="range"
        :min="min"
        :max="max"
        :value="modelValue"
        @input="(e: any) => handleModelValueChange(e.target.value)"
      >
    </label>
  </FeatureOptEditContainer>
</template>

<style lang="postcss">
.feat-opt-range-label {
  @apply flex flex-1 gap-1 items-center justify-between;


}

.feat-opt-range-label input[type="range"] {
  @apply bg-blue-300 rounded-full cursor-pointer;
}
</style>
