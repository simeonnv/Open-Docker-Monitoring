<script setup lang="ts">

import { useDockersStore } from '../stores/docker';
const { fetchDockers } = useDockersStore();
const { dockers, hasOnlyOneDocker } = storeToRefs(useDockersStore());

await fetchDockers();

</script>

<template>

  <div class="w-full h-full">
    
    {{ dockers }}

    {{ hasOnlyOneDocker }}

  </div>

</template>