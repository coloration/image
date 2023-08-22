import { FeatureGroup, FeatureType } from './base'

export class Feature {

  uni: boolean = false
  type: FeatureType = FeatureType.None
  group: FeatureGroup = FeatureGroup.None

  check() {
    return false
  }

}