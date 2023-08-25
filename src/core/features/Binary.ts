import { FeatureFieldType } from '../ui'
import { Feature } from './Feature'
import { FeatureGroup, FeatureType } from './base'

export class Binary extends Feature {
  type = FeatureType.Binary
  group = FeatureGroup.Color
  uni = FeatureType.Binary
  
  fields = [
    { 
      label: 'threshold', 
      key: 'threshold',
      value: 128,
      type: FeatureFieldType.range,
      props: {
        min: 0,
        max: 255
      }
    },
  ]
  check() {
    return true
  }
}