<script lang="ts" setup>
withDefaults(
  defineProps<{
    title?: string,
    open?: boolean,
  }>(),
  {}
)
</script>
<template>
  <li class="sidebar-link-subgroup">
    <a
      class="sidebar-sub-link"
      href="#0"
      @click.prevent="open = !open"
      aria-haspopup="true"
      :aria-expanded="open"
    >
      <!-- arrow -->
      <i-mingcute-right-fill class="-ml-1" :class="{ 'rotate-90': open }" />
      <!-- <svg class="fill-slate-400 shrink-0 ml-2" :class="{ 'rotate-90': open }" width="8" height="10" xmlns="http://www.w3.org/2000/svg">
        <path d="M1 2 2.414.586 6.828 5 2.414 9.414 1 8l3-3z" />
      </svg> -->
      <span class="ml-0">{{title}}</span>
    </a>
    <!-- 3rd level -->
    <ul class="sidebar-lsg-son-list" :class="{ 'hidden': !open }">
      <slot />
    </ul>
  </li>
</template>
<style lang="postcss">
.sidebar-link-subgroup {
  @apply mt-3;
}

.sidebar-lsg-son-list {
  @apply mb-3 ml-1 pl-4 border-l border-slate-200 dark:border-slate-800;
}

.sidebar-sub-link {
  @apply flex items-center space-x-2 text-slate-800 font-medium dark:text-slate-200;
}
</style>