<script lang="ts" setup>
import { onMounted } from 'vue';
import { storeToRefs } from 'pinia';

const store = useContainersStore();
const { containers } = storeToRefs(store);
const { getContainers } = store;

console.log("STORE", store)

await getContainers();

</script>

<template>
  <div>
    <!-- Iterate over containers for meaningful display -->
    <ul>
      <li v-for="container in containers" :key="container.id">
        {{ container.id }} - {{ container.names.join(', ') }} ({{ container.status }})
      </li>
    </ul>
  </div>
</template>