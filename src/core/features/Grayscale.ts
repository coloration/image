import { Feature } from './Feature'
import { FeatureGroup, FeatureType } from './base'

export class Grayscale extends Feature {
  type = FeatureType.Grayscale
  group = FeatureGroup.Color
}