<script setup>
import { useDockersStore } from '../stores/docker';
const { dockers } = storeToRefs(useDockersStore());
</script>

<template>
  <div class="w-full h-full flex flex-wrap justify-center gap-4 items-center">
    <template v-for="(docker, name) in dockers" :key="name">
      <ChooseDockerCard :docker="docker" :name="name" />
    </template>
  </div>
</template>