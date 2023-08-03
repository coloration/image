<script lang="ts" setup>
import { ref } from 'vue'

const selectedPic = ref('')

async function handleFileChange(e: any) {
  console.log('files', e.target.files)
  const files: File[] = Array.from(e.target.files)
  const reader = new FileReader()
  const fileBuffers = await files.reduce(async (last, f) => {
    const allUint8Data = await last
    console.log(f instanceof Blob)

    
    return new Promise((resolve) => {
      reader.onload = () => {
        const uint8Data = new Uint8Array(reader.result as ArrayBuffer)
        allUint8Data.push(uint8Data)
        resolve(allUint8Data)
      }
      reader.readAsArrayBuffer(f)

     
    })
  }, Promise.resolve([] as Uint8Array[]))


  console.log(fileBuffers)

  // selectedPic.value = e.target.value
}
</script>
<template>
<div class="home">

  <label for="file">
    <input type="file" id="file" multiple @change="handleFileChange">

    <div class="w-100 h-80 bg-blue-400">
    </div>
  </label>
</div>
</template>
<style lang="postcss">
.home {
  @apply;
}
</style>