

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
        
      <SidebarElement icon_name="material-symbols:space-dashboard-rounded" text="dashboard" path="/" :folded/>

      <SidebarElement icon_name="mingcute:dashboard-2-fill" text="statistics" path="/statistics" :folded/>

      <SidebarElement icon_name="mingcute:user-security-fill" text="Authorization" path="/authorization" :folded/>

      <SidebarElement icon_name="lets-icons:database-fill" text="logs" path="/logs" :folded/>



      <div class="grow"></div>

      <SidebarElement icon_name="material-symbols:settings" text="settings" path="/settings" :folded/>

    </div>
</template>