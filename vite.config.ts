import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import pages from 'vite-plugin-pages'
import layouts from 'vite-plugin-vue-layouts'
import unocss from 'unocss/vite'
import { presetUno } from 'unocss'
import transformerDirectives from '@unocss/transformer-directives'
import markdown from 'vite-plugin-md'
import autoprefixer from 'autoprefixer'
import postcssNesting from 'postcss-nesting'
// import inspect from 'vite-plugin-inspect'
import components from 'unplugin-vue-components/vite'
import icons from 'unplugin-icons/vite'
import iconsResolver from 'unplugin-icons/resolver'

import markdownPrism from 'markdown-it-prism'
// @ts-expect-error missing types
import markdownLinkAttributes from 'markdown-it-link-attributes'
import markdownAnchor from 'markdown-it-anchor'
import markdownToc from 'markdown-it-toc-done-right'

// https://vitejs.dev/config/
export default defineConfig({
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    }
  },
  plugins: [
    vue({
      include: [/\.vue$/, /\.md/],
    }), 
    vueJsx(),
    pages({   // https://github.com/hannoeru/vite-plugin-pages
      extensions: ['vue', 'md'],
    }),
    layouts({ // https://github.com/JohnCampionJr/vite-plugin-vue-layouts

    }),
    unocss({  // https://github.com/unocss/unocss
      presets: [
        presetUno(),
        // ...custom presets
      ],
      transformers: [
        transformerDirectives()
      ],
    }),
    icons({ // https://github.com/antfu/unplugin-icons
      autoInstall: true, // change `package.json`
    }),
    components({  // https://github.com/antfu/unplugin-vue-components#configuration
      extensions: ['vue', 'md'],
      include: [/\.vue$/, /\.vue\?vue/, /\.md$/],
      dts: true,

      // auto import icons
      resolvers: [
        iconsResolver({ // https://github.com/antfu/unplugin-icons#auto-importing
        }),
      ],
    }),
    markdown({ // https://github.com/antfu/vite-plugin-md
      wrapperClasses: 'mk',
      headEnabled: true,
      markdownItSetup(md) {
        // https://prismjs.com/
        md.use(markdownPrism)
        // md.use(PreHandlePlugin)
        md.use(markdownLinkAttributes, {
          pattern: /^https?:\/\//,
          attrs: {
            target: '_blank',
            rel: 'noopener',
          },
        })
        md.use(markdownAnchor)
        md.use(markdownToc, { level: 2 })
      }
    }),
    // inspect()
  ],
  css: {
    postcss: {
      plugins: [
        autoprefixer({}), // add options if needed
        postcssNesting({}),
      ],
    }
  }
})
