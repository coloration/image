<script lang="ts" setup>
import { ref } from 'vue'
import init, { grayscale } from 'frame-handler'

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

  let d = Date.now()
  imgs.value = fileBuffers.map(grayscale)

  console.log(Date.now() - d)
  console.log(fileBuffers[0].length)
}

</script>
<template>
<div class="home">

  <label for="file">
    <input type="file" id="file" multiple @change="handleFileChange">

    <div class="w-100 h-80 bg-blue-400">
      <div v-for="(img, i) in imgs" :key="i">
        <img :src="img" alt="">
      </div>
    </div>
  </label>
</div>
</template>
<style lang="postcss">
.home {
  @apply;
}
</style>