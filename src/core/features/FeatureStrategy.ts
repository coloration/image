import { Binary } from './Binary'
import { Feature } from './Feature'
import { Grayscale } from './Grayscale'
import { Invert } from './Invert'
import { FeatureType } from './base'

export class FeatureStrategy {
  
  static create(type: FeatureType, oldFeats: Feature[]): Feature | null {
    
    if (oldFeats.find(feat => feat.uni && feat.type === type)) {
      return null
    }


    if (type === FeatureType.Grayscale) {
      return new Grayscale()
    }

    if (type === FeatureType.Invert) {
      return new Invert()
    }

    if (type === FeatureType.Binary) {
      return new Binary()
    }

    return new Feature()
  }
}