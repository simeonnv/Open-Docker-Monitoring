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

  // sourcemap: {
  //   server: true,
  //   client: true
  // },

  vue: {
    compilerOptions: {
      isCustomElement: (tag) => tag === 'u-header' // Adjust tag name if needed
    }
  },

  // ssr: true,
  
  devServer: {
    port: process.env.FRONTEND_PORT !== undefined ? parseInt(process.env.FRONTEND_PORT) : 7004 // Set the port for development
  },

  colorMode: {
    preference: 'system', // default
    fallback: 'light',
    classSuffix: '',
  },

  runtimeConfig: {
    public: {
      // postgresUser: process.env.POSTGRES_USER,
      // postgresPassword: process.env.POSTGRES_PASSWORD,
      // postgresDatabase: process.env.POSTGRES_DATABASE,
      // postgresAddress: process.env.POSTGRES_ADDRESS,
      backendPort: process.env.BACKEND_PORT,
      backendAddress: process.env.BACKEND_ADDRESS,
      backendPublicAddress: process.env.BACKEND_PUBLIC_ADDRESS,
      frontendPort: process.env.FRONTEND_PORT,
      frontendAddress: process.env.FRONTEND_ADDRESS,
    }
  },

  modules: [
    "@nuxtjs/tailwindcss",
    "@nuxt/devtools",
    "@nuxtjs/color-mode",
    "@nuxt/image",
    "@nuxt/icon",
    "@nuxt/eslint",
    "@nuxt/content",
    "@nuxt/fonts",
    "@vueuse/nuxt",
    '@pinia/nuxt',
    "shadcn-nuxt",
    '@formkit/auto-animate/nuxt'
  ],
  app: {
    pageTransition: {
      name: 'layout', // Name of the transition
      mode: 'out-in', // Wait for leaving transition to finish before entering
      duration: 300 // Optional: transition duration in ms
    }
  },
  
  
  
  shadcn: {
    prefix: '',
    /**
     * Directory that the component lives in.
     * @default "./components/ui"
     */
    componentDir: './components/ui'
  },

  compatibilityDate: "2025-02-25"
})