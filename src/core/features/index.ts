import { FeatureType, FeatureGroup } from './base'

export const featureNameGroup: { [key: string]: string } = {
  [FeatureType.Grayscale]: 'Grayscale',
  [FeatureType.Invert]: 'Invert',
  [FeatureType.Binary]: 'Binary',
  [FeatureType.ColorReplace]: 'Replace color',

  [FeatureType.Format]: 'Format',

  [FeatureType.Crop]: 'Crop',
  [FeatureType.Resize]: 'Resize',

  [FeatureType.None]: 'None',
}


export const groupColor : { [key: string]: string } = {
  [FeatureGroup.Color]: 'blue',
  [FeatureGroup.Format]: 'red',
  [FeatureGroup.Shape]: 'green',
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
      { name: featureNameGroup[FeatureType.ColorReplace], value: FeatureType.ColorReplace },
    ]
  }, 
  {
    group: FeatureGroup.Format,
    features: [
      { name: featureNameGroup[FeatureType.Format], value: FeatureType.Format },
    ]
  },
  {
    group: FeatureGroup.Shape,
    features: [
      { name: featureNameGroup[FeatureType.Resize], value: FeatureType.Resize },
      { name: featureNameGroup[FeatureType.Crop], value: FeatureType.Crop },
    ]
  },
]

export * from './base'
export * from './FeatureStrategy'
export * from './Feature'
export * from './Grayscale'
export * from './Format'