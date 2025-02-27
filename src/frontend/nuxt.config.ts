// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: {
    enabled: true,

    timeline: {
      enabled: true
    },

    vscode: {
      port: 3090
    }
  },

  sourcemap: {
    server: true,
    client: true
  },

  vue: {
    compilerOptions: {
      isCustomElement: (tag) => tag === 'u-header' // Adjust tag name if needed
    }
  },

  ssr: true,
  
  devServer: {
    port: process.env.FRONTEND_PORT !== undefined ? parseInt(process.env.FRONTEND_PORT) : 7004 // Set the port for development
  },

  colorMode: {
    preference: 'system', // default
    fallback: 'light',
    classSuffix: '',
  },


  modules: [
    "@nuxt/ui",
    "@nuxtjs/tailwindcss",
    "@nuxt/devtools",
    "@nuxtjs/color-mode",
    "@nuxt/image",
    "@nuxt/icon",
    "@nuxt/eslint",
    "@nuxt/content",
    "@nuxt/fonts",
    "@vueuse/nuxt",
    '@pinia/nuxt'
  ],
  
  compatibilityDate: "2025-02-25"
})