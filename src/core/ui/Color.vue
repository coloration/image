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
  (e: 'update:modelValue', val: string): void
}>()

function handleModelValueChange(val: string) {
  console.log(val)
  const isColor = (_c: any) => true
  if (!isColor(val)) val = '#ffffff'
  emits('update:modelValue', val)
}
</script>

<template>
  <FeatureOptEditContainer>
    <!-- -->
    <label class="feat-opt-range-label">
      <div class="flex">
        <span>{{ label }}: </span>
        <input
          class="text-center w-20 text-sm rounded border-1 border-blue-700 ml-2"
          type="text"
          :value="modelValue"
          @input="(e: any) => handleModelValueChange(e.target.value)">
      </div>
      <label class="flex items-center">
        <i-ant-design-bg-colors-outlined />

        <div class="w-14 h-6 rounded overflow-hidden">
          <input
          id=""
          type="color"
          :value="modelValue"
          @input="(e: any) => handleModelValueChange(e.target.value)">
        </div>
      </label>
    </label>
  </FeatureOptEditContainer>
</template>

<style lang="postcss">
.feat-opt-range-label {
  @apply flex flex-1 gap-1 items-center justify-between;


}

.feat-opt-range-label input[type="color"] {
  @apply w-full h-full;
}
</style>
