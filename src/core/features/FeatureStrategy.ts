import { Feature } from './Feature'
import { Grayscale } from './Grayscale'
import { FeatureType } from './base'

export class FeatureStrategy {
  
  static create(type: FeatureType, oldFeats: Feature[]): Feature | null {
    
    if (oldFeats.find(feat => feat.uni && feat.type === type)) {
      return null
    }


    if (type === FeatureType.Grayscale) {
      return new Grayscale()
    }

    return new Feature()
  }
}