import Aos from 'aos'
import { onMounted } from 'vue'

export function useAos() {
  onMounted(() => {
    Aos.init({
      once: true,
      duration: 600,
      easing: 'ease-out-sine',
    })
  })
}