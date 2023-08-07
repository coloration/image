<script lang="ts" setup>
import { ref } from 'vue'
import init, { grayscale } from 'frame-handler'

const canvasRef = ref<any>(null)

const imgs = ref<string[]>([])

async function handleFileChange(e: any) {
  console.log('files', e.target.files)
  const files: File[] = Array.from(e.target.files)
  const reader = new FileReader()
  const fileBuffers = await files.reduce(async (last, f) => {
    const allUint8Data = await last
    console.log(f)

    
    return new Promise((resolve) => {
      reader.onload = () => {
        const uint8Data = new Uint8ClampedArray(reader.result as ArrayBuffer)
        allUint8Data.push(uint8Data)
        resolve(allUint8Data)
      }
      reader.readAsArrayBuffer(f)

     
    })
  }, Promise.resolve([] as Uint8ClampedArray[]))


  console.log(fileBuffers)

  await init()

  imgs.value = fileBuffers.map((b) => grayscale(b as any))



  const ctx = canvasRef.value.getContext('2d')

  console.log(ctx)

  const img = new Image()
  img.onload = () => {
    ctx.drawImage(img, 0, 0)
  }
  const base64Reader = new FileReader()
  base64Reader.onload = () => {
    img.src = base64Reader.result as string
  }
  base64Reader.readAsDataURL(files[0])
  
  

  const photon = await import('@silvia-odwyer/photon')
  
  let newImage = photon.open_image(canvasRef.value, ctx);
  photon.filter(newImage, "radio");

  photon.putImageData(canvasRef.value, ctx, newImage);
}

</script>
<template>
<div class="home">

  <label for="file">
    <input type="file" id="file" accept = 'image/*' multiple @change="handleFileChange">

    <div class="w-100 h-80 bg-blue-400">
      <div v-for="(img, i) in imgs" :key="i">
        <img :src="img" alt="">
      </div>
    </div>


    <canvas ref="canvasRef" class="h-100px w-100px"></canvas>
  </label>
</div>
</template>
<style lang="postcss">
.home {
  @apply;
}
</style>