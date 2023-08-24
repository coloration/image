<script lang="ts" setup>
import { featureNameGroup, type Feature } from '@/core'
import { FeatureFieldComponent } from '@/core/ui'
const props = withDefaults(
  defineProps<{
    feature: Feature
  }>(),
  {

  }
)

const emits = defineEmits<{
  (e: 'change', val: any): void
}>()

function handleModelValueChange(fIndex: number, v: any) {
  props.feature.fields[fIndex].value = v
  console.log('emits')
  emits('change', props.feature.fields)
}
</script>

<template>
  <FeatureButton v-if="feature" size="lg">
    <div>
      {{ featureNameGroup[feature.type] }}


      <div v-for="(field, i) in feature.fields">
        <component
          :is="(FeatureFieldComponent as any)[field.type]"
          :label="field.label"
          :modelValue="field.value"
          @update:modelValue="(v: any) => handleModelValueChange(i, v)"
        >
        </component>
      </div>
    </div>

  </FeatureButton>
</template>

<style lang="postcss">
.feature-opt-button {
  @apply;
}
</style>
