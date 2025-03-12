<script setup>

    import { useDockersStore } from '../stores/docker';
    const { dockers, hasOnlyOneDocker, isADockerSelected  } = storeToRefs(useDockersStore());

</script>

<template>
  
  <div class="w-full h-full grid grid-cols-2">
    
    <div v-for="docker in dockers">
      <ChooseDockerCard/>
    </div>

  </div>

</template>