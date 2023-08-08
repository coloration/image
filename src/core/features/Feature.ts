import { FeatureGroup, FeatureType } from './base'

export class Feature {
  type: FeatureType = FeatureType.None
  group: FeatureGroup = FeatureGroup.None
  
  params: any = {}

  toArrayBuffer() {

  }

  toBase64() {

  }

  async render() {

    return new Uint8ClampedArray()
  }
}