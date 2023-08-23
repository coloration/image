import { Feature } from './Feature'
import { FeatureGroup, FeatureType } from './base'

export class Binary extends Feature {
  type = FeatureType.Binary
  group = FeatureGroup.Color
  uni = true

  check() {
    return true
  }
}