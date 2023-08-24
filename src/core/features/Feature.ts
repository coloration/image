import type { FeatureFieldType } from '../ui'
import { FeatureGroup, FeatureType } from './base'

export interface FeatureField {
  label: string, 
  key: string, 
  value: any, 
  type: FeatureFieldType
}

export class Feature {

  uni: string = ''
  type: FeatureType = FeatureType.None
  group: FeatureGroup = FeatureGroup.None
  fields: FeatureField[] = []
  check() {
    return false
  }

}