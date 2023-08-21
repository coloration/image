import { FeatureGroup, FeatureType } from './base'

export class Feature {

  uni: boolean = false
  type: FeatureType = FeatureType.None
  group: FeatureGroup = FeatureGroup.None
  source?: Uint8ClampedArray
  params: any = {}

  check() {
    return false
  }

  toArrayBuffer() {

  }

  toBase64() {

    return ''
  }

  async render(arr: Uint8ClampedArray) {
    this.source = arr
  }
}