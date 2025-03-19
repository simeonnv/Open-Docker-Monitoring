<script setup>
import { useDockersStore } from '@/stores/docker';
const { dockers } = storeToRefs(useDockersStore());
</script>

<template>
  <div class="w-full h-full flex flex-col" v-auto-animate>
    
    <div class="text-primary font-bold text-2xl text-center p-5 flex flex-row items-center justify-center gap-2">
      <p><span class="text-blue-300">All</span> Docker CLI connections</p>
      <DashboardDashboardConnectionsNew/>
    </div>

    <ScrollArea v-if="dockers && Object.keys(dockers).length > 0" class="h-full w-full rounded-md border py-5 gap-4">
      <div v-for="(docker, name) in dockers" :key="name" class="grid grid-cols-1 gap-6 px-8 w-full h-full items-start justify-center align-middle">
        <DashboardDashboardConnectionsCard :docker="docker" :name="name" />
      </div>
    </ScrollArea>

    <div v-else class="w-full h-full flex flex-col justify-center items-center text-primary text-2xl">
      <p>
        There are <span class="text-blue-300">no</span> docker connections j-j 
      
      </p>
    </div>

  </div>
</template>