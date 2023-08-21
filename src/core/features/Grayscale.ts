import { Feature } from './Feature'
import { FeatureGroup, FeatureType } from './base'
import init, { grayscale } from 'frame-handler'


export class Grayscale extends Feature {
  type = FeatureType.Grayscale
  group = FeatureGroup.Color

  check() {
    return true
  }

  async render(arr) {
    await init()
    grayscale(arr)

  }
}