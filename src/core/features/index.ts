import { FeatureType, FeatureGroup } from './base'

export const featureNameGroup: { [key: string]: string } = {
  [FeatureType.Grayscale]: 'Grayscale',
  [FeatureType.Invert]: 'Invert',
  [FeatureType.Binary]: 'Binary',
  [FeatureType.Format]: 'Format',
}


export const groupColor : { [key: string]: string } = {
  [FeatureGroup.Color]: 'blue',
  [FeatureGroup.Format]: 'red',
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
  }, 
  {
    group: FeatureGroup.Format,
    features: [
      { name: featureNameGroup[FeatureType.Format], value: FeatureType.Format },
    ]
  }
]

export * from './base'
export * from './FeatureStrategy'
export * from './Feature'
export * from './Grayscale'
export * from './Format'