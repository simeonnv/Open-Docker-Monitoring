

<script setup>
const props = defineProps({
  folded: {
    type: Boolean,
    default: false
  }
})
  const route = useRoute()
  const isActive = computed(() => route.path === props.path)
  console.log(route.path)
  console.log(isActive.value)

</script>



<template>
    <div class="h-full flex flex-col align-middle w-full items-center pt-4">
        
        <SidebarElement icon_name="material-symbols:settings" text="settings" :folded/>


    </div>
</template>