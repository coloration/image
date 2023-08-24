export enum FeatureFieldType {
  text,
  number,
  range,
  select,
  radio
}

import { default as Text } from './Text.vue';

export const FeatureFieldComponent = {
  [FeatureFieldType.text]: Text
}