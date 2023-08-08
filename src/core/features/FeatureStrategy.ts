import { Feature } from './Feature'
import { Grayscale } from './Grayscale'
import { FeatureType } from './base'

export class FeatureStrategy {
  
  static create(type: FeatureType): Feature {

    if (type === FeatureType.Grayscale) {
      return new Grayscale()
    }

    return new Feature()
  }
}