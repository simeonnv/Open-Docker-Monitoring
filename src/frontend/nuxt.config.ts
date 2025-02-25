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

  ssr: true,
  
  devServer: {
    port: 7004 // Set the port for development
  },



  modules: [
    "@nuxt/ui",
    "@nuxtjs/tailwindcss",
    "@nuxt/devtools",
    "@nuxtjs/color-mode",
    "@nuxt/image",
    "@nuxt/icon",
    "@nuxt/eslint",
    // "@nuxt/content",
    "@nuxt/fonts",
    "@vueuse/nuxt"
  ],

  compatibilityDate: "2025-02-25"
})