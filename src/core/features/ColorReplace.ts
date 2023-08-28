import { FeatureFieldType } from '../ui'
import { Feature } from './Feature'
import { FeatureGroup, FeatureType } from './base'

export class ColorReplace extends Feature {
  type = FeatureType.ColorReplace
  group = FeatureGroup.Color
  uni = ''
  
  fields = [
    { 
      label: 'from', 
      key: 'from',
      value: '#ffffff',
      type: FeatureFieldType.color,
    },
    { 
      label: 'to', 
      key: 'to',
      value: '#ffffff',
      type: FeatureFieldType.color,
    },
  ]
  check() {
    return true
  }
}