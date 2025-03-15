<script setup lang="ts">
  const { fetchDockers } = useDockersStore();
  await fetchDockers();
</script>

<template>
  <div class="flex flex-col md:flex-row w-full h-full">
    <LazyDashboardDashboardSidebar hydrate-on-interaction="mouseover" />
    <div class="flex w-full h-full">
      <DashboardDashboardBody />
    </div>
  </div>
</template>