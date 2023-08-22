<script lang="ts" setup>
import { getCurrentInstance } from 'vue'

const instance = getCurrentInstance()


</script>

<template>
  <div class="board">
    <!-- -->
    <div class="board-header">
      <div>
        <slot name="title"></slot>
      </div>
      <div>
        <slot name="title-rest"></slot>
      </div>
    </div>


    <div class="board-container">
      <slot></slot>
    </div>

    <div class="board-footer" v-if="instance?.slots?.footer">
      <slot name="footer"></slot>
    </div>

  </div>
</template>

<style lang="postcss">
.board {
  @apply bg-white bg-opacity-50 p-2 flex flex-col;
  border-radius: 10px 0 0 10px;
}

.board-header {
  @apply bg-white py-2 font-700 flex items-center text-lg px-6 -ml-4;
  border-radius: 10px 0;
}

.board-footer {
  @apply bg-gray-200 py-1 -ml-4 pl-6 pr-2 flex items-center flex-row-reverse;
  border-radius: 0px 10px;
}

.board-container {
  @apply py-2 flex-1;
}
</style>
