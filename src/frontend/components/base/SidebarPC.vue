<script setup>

const folded = ref(true)

</script>



<template>
  <div :class="folded ? 'w-2' : 'w-32'" class="transition-all bg-red-500 flex-col min-w-fit h-full hidden md:flex">
    <BaseSidebarContent/>
    <div class="grow"></div>
    <Button @click="folded = !folded">
      <
    </Button>
  </div>
</template>