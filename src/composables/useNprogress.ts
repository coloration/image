import NProgress from 'nprogress'
import { useRouter } from 'vue-router'
import { useStyleTag } from '@vueuse/core'

export function useNprogress() {

  useStyleTag(`
    #nprogress {
      position: fixed;
      width: 100%;
      top: 0;
      left: 0;
      height: .25rem;
    }

    #nprogress .bar {
      height: 100%;
      background-color: var(--color-primary);
    }
  `)
  const router = useRouter();
  router.beforeEach(() => {
    NProgress.start()
  })

  router.afterEach(() => {
    NProgress.done()
  })

  return NProgress
}