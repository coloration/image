<script lang="ts" setup>
import { featureNameGroup, type Feature, groupColor } from '@/core'
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
  emits('change', props.feature.fields)
}
</script>

<template>
  <FeatureButton
    :badge="groupColor[feature.group]"
    v-if="feature"
    size="lg" 
    :title="featureNameGroup[feature.type]">
    <div>
      <div v-for="(field, i) in feature.fields">
        <component
          :is="(FeatureFieldComponent as any)[field.type]"
          v-bind="field.props || {}"
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
