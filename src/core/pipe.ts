import { FeatureStrategy, type Feature, FeatureType } from './features'

export class Pipe {

  features: Feature[] = []
  sources: ArrayBuffer[] = []
  step: number = 0
  index: number = 0
  

  addFeature(feat: FeatureType) {
    this.features.push(FeatureStrategy.create(feat))
  }

  delFeature(index: number) {
    this.features.splice(index, 1)
  }

  async render() {
    await this.features.reduce(async(last, feat) => {
      await last
      return feat.render()
    }, Promise.resolve(new Uint8ClampedArray()))
  }

  toJSONConfigure() {

  }
}