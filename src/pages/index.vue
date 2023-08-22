<script lang="ts" setup>
import { ref } from 'vue'
import { featureGroups, Pipe, FeatureType, featureNameGroup, readAsBuffer } from '../core'
import { download } from '@coloration/kit'


const imageResults = ref<string[]>([])
const needRerender = ref<boolean>(false)
const pipe = ref<Pipe>(new Pipe())

async function handleFileChange(e: any) {
  const files: File[] = Array.from(e.target.files)
  
  await pipe.value.addSource(files)
  needRerender.value = true

}

function deleteSource(index: number) {
  pipe.value.delSource(index)
  needRerender.value = true
}

function addFeature(type: FeatureType) {
  pipe.value.addFeature(type)
  needRerender.value = true
}

function removeFeature(index: number) {
  pipe.value.delFeature(index)
  needRerender.value = true
}

async function handleRun() {
  const responses = await pipe.value.handle()
  imageResults.value = responses
}

function handleDownload () {
  imageResults.value.map((res, i) => {
    
    // TODO replace when kit package fix
    const arr = res.split(',') as any
    const mime = arr[0].match(/:(.*?);/)[1]
    console.log(mime)
    const bstr = atob(arr[1])
    let n = bstr.length
    const u8arr = new Uint8Array(n)
    
    while(n--){
      u8arr[n] = bstr.charCodeAt(n)
    }
  
  const blob = new Blob([u8arr], { type: mime })
  const objUrl = URL.createObjectURL(blob)
    // const fileName = pipe.value.sources[i].name
    download(pipe.value.sources[i].name, objUrl)
  })
}
</script>

<template>
  <div class="main-view">
    <!-- -->
    <div class="flex flex-col gap-2 flex-1">
      <Board>
        <template #title>Input</template>


        <template #footer>
          <div>
            <label>
              <input
                type="file"
                class="hidden"
                accept="image/*"
                multiple
                @change="handleFileChange">
              <OptButton>Upload</OptButton>
            </label>
          </div>
        </template>

        <template #default>
          <div class="flex gap-2 min-h-40">

            <ImageBox
              closable
              @close="deleteSource(i)"
              v-for="(item, i) in pipe.displaySources"
              :key="i"
              :source="item" />
          </div>
        </template>
      </Board>
      <Board>
        <template #title>Output</template>


        <template #footer>
          <OptButton
            @click="handleDownload"
            :disabled="imageResults.length === 0"
          >Download</OptButton>
        </template>

        <template #default>
          <div class="flex gap-2  min-h-40">
            <ImageBox
              v-for="(item, i) in imageResults"
              :key="i"
              :source="item" />
          </div>
        </template>
      </Board>
    </div>

    <div class="flex-1 flex gap-6">
      <Board class="pipe-board">
        <template #title>Pipe</template>

      <template #footer>
          <OptButton
            @click="handleRun"
            :disabled="!pipe.isReady">
            Run
          </OptButton>
        </template>

        <template #default>
          <div class="flex flex-col gap-2">
            <FeatureButton
              closable
              @close="removeFeature(i)"
              size="lg"
              v-for="(item, i) in pipe.features">
              {{ featureNameGroup[item.type] }}
            </FeatureButton>
          </div>
        </template>

      </Board>
      <Board class="flex-1">
        <template #title>Features</template>
        <!-- <template #footer>
          <OptButton disabled>Setting</OptButton>
        </template> -->
        <template #default>
          <div
            v-for="item in featureGroups"
            :key="item.group"
            class="flex flex-col gap-1">
            <FeatureButton
              class="cursor-pointer"
              v-for="feat in item.features"
              @click="addFeature(feat.value)">
              {{ feat.name }}
            </FeatureButton>
          </div>
        </template>
      </Board>
    </div>
  </div>
</template>

<style lang="postcss">
.main-view {
  @apply pt-4 flex gap-10 items-stretch;
}

.pipe-board {
  flex: 2;
}
</style>
