<template>
  <div class="!transition-all !ease-in-out !duration-300 bg-background h-screen max-h-screen min-h-screen w-full flex flex-col">
    <BaseMainHeader />
    <div class="flex-1 overflow-auto flex flex-row">
      <div v-auto-animate>
        <LazySidebarPC v-if="!isAuthRoute" hydrate-on-interaction="mouseover"/>
      </div>
      <div class="flex-1 overflow-auto">
        <NuxtPage />
      </div>
    </div>
  </div>
</template>

<script setup>
const route = useRoute()

const isAuthRoute = computed(() => {
  return route.path === '/login' || route.path === '/signup'
})
</script>