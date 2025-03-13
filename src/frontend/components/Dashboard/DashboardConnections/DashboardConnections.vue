<script setup>
import { useDockersStore } from '@/stores/docker';
const { dockers } = storeToRefs(useDockersStore());
</script>

<template>
  <div class="w-full h-full flex flex-col">
    
    <p class="text-primary font-bold text-2xl text-center p-5"><span class="text-blue-300">All</span> Docker CLI connections</p>

    <div v-for="(docker, name) in dockers" :key="name" class="w-full">
      <DashboardDashboardConnectionsCard :docker="docker" :name="name" />
    </div>
  </div>
</template>