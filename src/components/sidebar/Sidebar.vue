<script lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'

import SidebarLinkGroup from './SidebarLinkGroup.vue'
import SidebarLinkSubgroup from './SidebarLinkSubgroup.vue'

export default {
  name: 'Sidebar',
  props: ['sidebarOpen', 'navs'],
  components: {
    SidebarLinkGroup,
    SidebarLinkSubgroup,
  },
  setup(props, { emit }) {

    const sidebar = ref(null)

    const currentRoute = useRouter().currentRoute.value

    // close on click outside
    const clickHandler = ({ target }) => {
      if (!sidebar.value) return
      if (
        !props.sidebarOpen ||
        sidebar.value.contains(target)
      ) return
      emit('close-sidebar')
    }

    // close if the esc key is pressed
    const keyHandler = ({ keyCode }) => {
      if (!props.sidebarOpen || keyCode !== 27) return
      emit('close-sidebar')
    } 

    onMounted(() => {
      document.addEventListener('click', clickHandler)
      document.addEventListener('keydown', keyHandler)
    })

    onUnmounted(() => {
      document.removeEventListener('click', clickHandler)
      document.removeEventListener('keydown', keyHandler)
    })
    
    return {
      currentRoute,
      sidebar,
    }
  },  
}
</script>
<template>
  <div>
    <transition
      enter-active-class="transition ease-out duration-200"
      enter-from-class="opacity-0"
      enter-to-class="opacity-100"
      leave-active-class="transition ease-out duration-100"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0"
    >            
      <div v-show="sidebarOpen" class="sidebar-mobile-shadow" aria-hidden="true"></div>
    </transition>
  
    <transition
      enter-active-class="transition ease-out duration-200 transform"
      enter-from-class="opacity-0 -translate-x-full"
      enter-to-class="opacity-100 translate-x-0"
      leave-active-class="transition ease-out duration-200"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0"
    >
      <aside
        id="sidebar"
        class="sidebar"
        ref="sidebar"
        v-show="sidebarOpen"
      >
        <!-- Gradient bg displaying on light layout only -->
        <div class="absolute inset-0 -left-[9999px] bg-gradient-to-b from-slate-50 to-white pointer-events-none -z-10 dark:hidden" aria-hidden="true"></div>
  
        <div class="fixed top-0 bottom-0 w-64 px-4 sm:px-6 md:pl-0 md:pr-8 overflow-y-auto no-scrollbar">
          <div class="pt-24 md:pt-28 pb-8">
  
            <!-- Docs nav -->
            <nav class="md:block">
              <ul class="text-sm">
                  
                <!-- 1st level -->
                <SidebarLinkGroup 
                  v-for="(nav, i) in navs"
                  :key="i"
                  v-slot="parentLink" 
                  :activeCondition="currentRoute.fullPath.includes('guides')">
                  <a
                    class="sidebar-root-link"
                    :class="{ 'before:hidden': !currentRoute.fullPath.includes('guides') }"
                    href="#0"
                    @click.prevent="parentLink.handleClick()"
                  >
                    <svg class="mr-3 shrink-0" width="24" height="24" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                      <path class="fill-purple-400" d="M19.888 7.804a.88.88 0 0 0-.314-.328l-7.11-4.346a.889.889 0 0 0-.927 0L4.426 7.476a.88.88 0 0 0-.314.328L12 12.624l7.888-4.82Z" />
                      <path class="fill-white dark:fill-slate-800" d="M4.112 7.804a.889.889 0 0 0-.112.43v7.892c0 .31.161.597.426.758l7.11 4.346c.14.085.3.13.464.13v-8.736l-7.888-4.82Z" />
                      <path class="fill-purple-600" d="M19.888 7.804c.073.132.112.28.112.43v7.892c0 .31-.161.597-.426.758l-7.11 4.346c-.14.085-.3.13-.464.13v-8.736l7.888-4.82Z" />
                    </svg>
                    
                    <span>{{ nav.name }}</span>
                  </a>
                  <ul class="mb-3 ml-4 pl-6 border-l border-slate-200 dark:border-slate-800" :class="{ 'hidden': !parentLink.expanded }">                    
                    <!-- <li class="mt-3" v-for="(navChild, j) in nav.children" :key="j">
                      <router-link to="/guides/marketing-api" custom v-slot="{ href, navigate, isExactActive }">
                        <a class="flex items-center space-x-3 font-medium" :class="isExactActive ? 'text-blue-600' : 'text-slate-800 dark:text-slate-200'" :href="href" @click="navigate">{{ navChild.name }}</a>
                      </router-link>
                    </li> -->
                    <SidebarLinkSubgroup v-for="(navChild, j) in nav.children" :key="j" :title="navChild.name" :open="currentRoute.fullPath.includes('alternative-scheme')">
                      <li class="mt-3" v-for="(navSon, k) in navChild.children" :key="k">
                        <router-link :to="navSon.value" custom v-slot="{ href, navigate, isExactActive }">
                          <a
                            class="sidebar-son-link"
                            :class="{ active: isExactActive}"
                            :href="href"
                            @click="navigate"
                          >{{ navSon.name }}</a>
                        </router-link>
                      </li>
                    </SidebarLinkSubgroup>
                  </ul>
                </SidebarLinkGroup>
              </ul>
            </nav>
          </div>
        </div>
  
      </aside>
    </transition>
  </div>
</template>
<style lang="postcss">
.sidebar-mobile-shadow {
  @apply 
    md:hidden 
    fixed inset-0 z-10 
    bg-slate-900 bg-opacity-20 transition-opacity;
}

.sidebar {
  @apply 
    fixed left-0 top-0 bottom-0 box-border
    w-64 h-screen border-r border-slate-200 
    
    md:left-auto md:shrink-0 z-10 md:!opacity-100 md:!block 
    dark:border-slate-800 dark:bg-slate-900;
}

.sidebar-root-link {
  @apply relative flex items-center font-[650] text-slate-800 p-1 before:absolute before:inset-0 before:rounded before:bg-gradient-to-tr before:from-blue-400 before:to-purple-500 before:opacity-20 before:-z-10 before:pointer-events-none dark:text-slate-200;
}

.sidebar-son-link {
  @apply flex items-center space-x-3 font-normal text-slate-600 dark:text-slate-400;

  &.active {
    @apply text-blue-600;
  }
}
</style>
