import { Binary } from './Binary'
import { Feature } from './Feature'
import { Format } from './Format'
import { Grayscale } from './Grayscale'
import { Invert } from './Invert'
import { ColorReplace } from './ColorReplace'
import { Resize } from './Resize'
import { Crop } from './Crop'


import { FeatureType } from './base'

export class FeatureStrategy {
  
  static create(type: FeatureType, oldFeats: Feature[]): Feature | null {
    
    if (oldFeats.find(feat => feat.uni && feat.type === type)) {
      return null
    }


    if (type === FeatureType.Grayscale) {
      return new Grayscale()
    }

    else if (type === FeatureType.Invert) {
      return new Invert()
    }

    else if (type === FeatureType.Binary) {
      return new Binary()
    }

    else if (type === FeatureType.Format) {
      return new Format()
    }

    else if (type === FeatureType.ColorReplace) {
      return new ColorReplace()
    }
    else if (type === FeatureType.Resize) {
      return new Resize()
    }
    else if (type === FeatureType.Crop) {
      return new Crop()
    }

    return new Feature()
  }
}