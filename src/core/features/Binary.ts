import { FeatureFieldType } from '../ui'
import { Feature } from './Feature'
import { FeatureGroup, FeatureType } from './base'

export class Binary extends Feature {
  type = FeatureType.Binary
  group = FeatureGroup.Color
  uni = FeatureType.Binary
  
  threshold: number = 128
  fields = [
    { 
      label: 'threshold', 
      key: 'threshold',
      value: '2222',
      type: FeatureFieldType.text
    },
    { 
      label: 'threshold2', 
      key: 'threshold2',
      value: '',
      type: FeatureFieldType.text
    },
  ]
  check() {
    return true
  }
}