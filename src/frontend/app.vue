<template>
  <div class="divide-gray-200 dark:divide-gray-800 bg-white dark:bg-gray-900 min-h-screen">
    <MainHeader />
    <UContainer>
      <UCard class="mt-10">
        <template #header>
          <NuxtPage />
        </template>
      </UCard>
    </UContainer>
  </div>
</template>

<style>

html {
  transition: background-color 0.3s ease, color 0.3s ease, filter 0.3s ease;
  transition: text-color 0s ease, color 0s ease, filter 0s ease;
}
</style>