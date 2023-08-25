import { FeatureFieldType } from '../ui'
import { Feature } from './Feature'
import { FeatureGroup, FeatureType } from './base'

export class Format extends Feature {
  type = FeatureType.Format
  group = FeatureGroup.Format
  uni = FeatureType.Format
  
  fields = [
    { 
      label: 'format', 
      key: 'format',
      value: 'png',
      type: FeatureFieldType.select,
      props: {
        options: [
          { name: '.png', value: 'png' },
          { name: '.jpg', value: 'jpg' },
          { name: '.ico', value: 'ico' },
        ],
      }
    },
  ]
  check() {
    return true
  }
}