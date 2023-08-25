<script lang="ts" setup>
import { isNumber } from '@vueuse/core'

const props = withDefaults(
  defineProps<{
    label?: string,
    modelValue?: string
    options?: { name: string, value: string }[]

  }>(),
  {
    label: '',
    modelValue: '',
    options: () => ([]),
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
  <FeatureOptEditContainer>
    <!-- -->
    <label class="flex justify-between items-center">
      <span>{{ label }}:</span>
      <select 
        :value="modelValue"
        @change="(e: any) => handleModelValueChange(e.target.value)">
        <option
          v-for="(opt, i) in options"
          :key="i"
          :value="opt.value">
          {{ opt.name }}
        </option>
      </select>
    </label>
  </FeatureOptEditContainer>
</template>

<style lang="postcss">
.feat-opt-text {
  @apply;
}
</style>
