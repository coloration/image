export function readAsBase64(f: File) {
  return new Promise<string>((resolve, reject) => {
    const reader = new FileReader()

    reader.onload = () => {
      resolve(reader.result as string)
    }

    reader.onerror = reject
    reader.readAsDataURL(f)
  })
}

export function readAsBuffer(f: File) {
  return new Promise<ArrayBuffer>((resolve, reject) => {
    const reader = new FileReader()

    reader.onload = () => {
      resolve(reader.result as ArrayBuffer)
    }

    reader.onerror = reject
    reader.readAsArrayBuffer(f)
  })
}