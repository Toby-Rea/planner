// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  modules: [
    '@nuxtjs/eslint-module',
    '@nuxtjs/google-fonts',
    'nuxt-icon'
  ],
  googleFonts: {
    download: true,
    families: {
      Lora: '400..700'
    }
  }
})
