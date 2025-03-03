<script setup>
const folded = ref(true)
</script>

<template>
  <div :class="folded ? 'w-2' : 'w-36'" class="ring-1 ring-gray-200 dark:ring-gray-800 divide-gray-200 dark:divide-gray-800
   transition-all duration-200 ease-in-out bg-background m-[1px] flex-col min-w-fit h-full hidden md:flex">
    
    <SidebarContent :folded="folded"/>
    
    <div class="w-full flex flex-row justify-center">
      <Button class="flex transition-all w-full duration-200 ease-in-out" variant="ghost" @click="folded = !folded">
        <Icon :class="folded ? 'rotate-0' : 'rotate-180'" 
              name="material-symbols:arrow-back-2-rounded" 
              class="w-6 h-6 transition-transform duration-300" />
      </Button>
    </div>
  </div>
</template>