<script lang="ts" setup>
import { computed, ref } from 'vue'
import { readAsBase64, featureGroups, Pipe, FeatureType, featureNameGroup, readAsBuffer } from '../core'
import init, { grayscale, greet } from 'frame-handler'


const imageSources = ref<string[]>([])
const imageResults = ref<string[]>([])
const needRerender = ref<boolean>(false)
const pipe = ref<Pipe>(new Pipe())


const runValidStatus = computed(() => {

})


async function handleFileChange(e: any) {
  console.log('files', e.target.files)
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
  const bufs = await Promise.all(pipe.value.sources.map(readAsBuffer))
  await init()
  greet('Davidd');
  const responses = bufs.map((buf) => {
    console.log(buf.byteLength)
    console.log(new Uint8Array(buf).byteOffset)
    return grayscale(new Uint8Array(buf))
  })
  console.log(bufs, responses)

  imageResults.value.push(...responses)

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
          <OptButton disabled>Download</OptButton>
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
            {{ pipe.isReady }}
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
        <template #footer>
          <OptButton disabled>Setting</OptButton>
        </template>
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
