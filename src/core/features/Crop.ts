import { FeatureFieldType } from '../ui'
import { Feature } from './Feature'
import { FeatureGroup, FeatureType } from './base'

export class Crop extends Feature {
  type = FeatureType.Crop
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
    { 
      label: 'x', 
      key: 'x',
      value: 0,
      type: FeatureFieldType.number,
      props: {
        min: 0,
      }
    },
    { 
      label: 'y', 
      key: 'y',
      value: 0,
      type: FeatureFieldType.number,
      props: {
        min: 0,
      }
    },
  ]
  check() {
    return true
  }
}