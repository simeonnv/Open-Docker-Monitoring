<script setup lang="ts">

  const { fetchDockers } = useDockersStore();

  await fetchDockers();

</script>

<template>
  
  <div class="w-full h-full">
    
    <ChooseDocker/>

  </div>

</template>