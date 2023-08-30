import { FeatureFieldType } from '../ui'
import { Feature } from './Feature'
import { FeatureGroup, FeatureType } from './base'

export class Resize extends Feature {
  type = FeatureType.Resize
  group = FeatureGroup.Shape
  uni = ''
  
  fields = [
    { 
      label: 'width', 
      key: 'width',
      value: 100,
      type: FeatureFieldType.number,
      props: {
        min: 1,
      }
    },
    { 
      label: 'height', 
      key: 'height',
      value: 100,
      type: FeatureFieldType.number,
      props: {
        min: 1,
      }
    },
  ]
  check() {
    return true
  }
}