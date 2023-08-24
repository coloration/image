import { FeatureStrategy, type Feature, FeatureType } from './features'
import { toArray } from '@coloration/kit'
import { readAsBase64, readAsBuffer } from './utils'
import init, { Pipe as WasmPipe } from 'frame-handler'
export class Pipe {

  features: Feature[] = []
  sources: File[] = []
  displaySources: string[] = []
  displayResults: string[] = []
  step: number = 0
  index: number = 0
  isReady: boolean = false
  wasmPipe: WasmPipe | null = null

  constructor() {
    (async () => {
      await init()
      console.log('init wasm pipe')
      this.wasmPipe = new WasmPipe()
    })()

  }


  check() {
    console.log('wasm feature length', this.wasmPipe?.feature_len())
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
    this.displaySources.splice(index, 1)
    console.log(index)
    this.check()
  }
 

  addFeature(featType: FeatureType) {
    const feat = FeatureStrategy.create(featType, this.features)
    if (!feat) return
    this.wasmPipe?.add_feature(featType, { a: 12, b: 5 })
    this.features.push(feat)
    this.check()
  }

  delFeature(index: number) {
    this.features.splice(index, 1)
    this.wasmPipe?.del_feature(index)
    this.check()
  }

  editFeature(index: number, param: any) {
    console.log('TODO: ', index, param)
  }

  async handle() {
    return (await Promise.all(this.sources.map(readAsBuffer)))
      .map(arrBuf => new Uint8Array(arrBuf))
      .map((buf, i) => {
        const type = this.sources[i].name.match(/.(\w+)$/)![1] || 'png'
        console.log(type)
        return this.wasmPipe!.render(buf, type, type)
      })
  }

  toJSONConfigure() {

  }
}