import { Feature } from './Feature'
import { FeatureGroup, FeatureType } from './base'

export class Invert extends Feature {
  type = FeatureType.Invert
  group = FeatureGroup.Color
  uni = FeatureType.Invert

  check() {
    return true
  }
}