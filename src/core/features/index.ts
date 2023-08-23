import { FeatureType, FeatureGroup } from './base'

export const featureNameGroup: { [key: string]: string } = {
  [FeatureType.Grayscale]: 'Grayscale',
  [FeatureType.Invert]: 'Invert',
  [FeatureType.Binary]: 'Binary',
}

export const featureGroups: { 
  group: FeatureGroup, 
  features: { value: FeatureType, name: string }[] 
}[] = [
  {
    group: FeatureGroup.Color,
    features: [
      { name: featureNameGroup[FeatureType.Grayscale], value: FeatureType.Grayscale },
      { name: featureNameGroup[FeatureType.Invert], value: FeatureType.Invert },
      { name: featureNameGroup[FeatureType.Binary], value: FeatureType.Binary },
    ]
  }
]

export * from './base'
export * from './FeatureStrategy'
export * from './Feature'
export * from './Grayscale'