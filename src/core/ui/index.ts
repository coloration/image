export enum FeatureFieldType {
  text,
  number,
  range,
  select,
  radio,
  color,
}

import { default as Text } from './Text.vue';
import { default as NumberText } from './Number.vue';
import { default as Range } from './Range.vue';
import { default as Select } from './Select.vue';
import { default as Color } from './Color.vue';

export const FeatureFieldComponent = {
  [FeatureFieldType.text]: Text,
  [FeatureFieldType.number]: NumberText,
  [FeatureFieldType.range]: Range,
  [FeatureFieldType.select]: Select,
  [FeatureFieldType.color]: Color,
}