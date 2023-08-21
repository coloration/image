import { FeatureStrategy, type Feature, FeatureType } from './features'
import { toArray } from '@coloration/kit'
import { readAsBase64, readAsBuffer } from './utils'
import init, {} from 'frame-handler'
export class Pipe {

  features: Feature[] = []
  sources: File[] = []
  displaySources: string[] = []
  displayResults: string[] = []
  step: number = 0
  index: number = 0
  isReady: boolean = false

  check() {
    if (
      this.features.length === 0
      || this.sources.length === 0
    ) {
      return this.isReady = false
    }

    this.isReady = this.features.every(feat => feat.check())
  }

  async addSource(file: File | File[]) {
    const newFiles = toArray(file)
    this.sources = this.sources.concat(newFiles)
    this.check()
    const convertSources = await Promise.all(newFiles.map(readAsBase64))
    this.displaySources = this.displaySources.concat(convertSources)
  }


  delSource(index: number)  {
    this.sources.splice(index, 1)
    this.check()
  }
 

  addFeature(featType: FeatureType) {
    const feat = FeatureStrategy.create(featType, this.features)
    if (!feat) return
    console.log('????????')
    this.features.push(feat)
    this.check()
  }

  delFeature(index: number) {
    this.features.splice(index, 1)
    this.check()
  }

  async handle() {
    const bufSources = await Promise.all(this.sources.map(readAsBuffer))

    await init()
    // this.displayResults = bufSources.map(async (buf) => {
    //   return await this.features.reduce(async(last, feat) => {
    //     const prev = await last
    //     this.step += 1
    //     return feat.render(prev)
    //   }, Promise.resolve(new Uint8ClampedArray(buf)))
    // })
  }

  toJSONConfigure() {

  }
}