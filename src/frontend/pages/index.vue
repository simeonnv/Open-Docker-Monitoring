<script setup lang="ts">




  const { fetchDockers } = useDockersStore();

  await fetchDockers();

</script>

<template>
  
  <div class="flex flex-row w-full h-full">
    
    <DashboardDashboardSidebar/>

    <div class="w-full h-full flex">
      <DashboardDashboardBody/>
    </div>

  </div>

</template>