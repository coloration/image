export function readAsBase64(f: File) {
  return new Promise<string>((resolve, reject) => {
    const reader = new FileReader()

    reader.onload = () => {
      resolve(reader.result as string)
    }

    reader.onerror = (e) => {
      reject(e)
    }
    reader.readAsDataURL(f)
  })
}

export function readAsBuffer(f: File) {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()

    reader.onload = () => {
      resolve(reader.result as ArrayBuffer)
    }

    reader.onerror = (e) => {
      reject(e)
    }
    reader.readAsArrayBuffer(f)
  })
}