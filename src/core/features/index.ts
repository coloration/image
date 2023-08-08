import { FeatureType, FeatureGroup } from './base'

export const featureNameGroup: { [key: string]: string } = {
  [FeatureType.Grayscale]: 'Grayscale'
}

export const featureGroups: { 
  group: FeatureGroup, 
  features: { value: FeatureType, name: string }[] 
}[] = [
  {
    group: FeatureGroup.Color,
    features: [
      { name: featureNameGroup[FeatureType.Grayscale], value: FeatureType.Grayscale }
    ]
  }
]

export * from './base'
export * from './FeatureStrategy'
export * from './Feature'
export * from './Grayscale'