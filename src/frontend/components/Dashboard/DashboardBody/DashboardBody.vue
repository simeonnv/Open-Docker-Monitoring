<script setup lang="ts">
import { useDashboardSidebarStore } from '../../../stores/dashboard_sidebar'
import { storeToRefs } from 'pinia'

const { selectElement } = useDashboardSidebarStore()
const { selectedElement } = storeToRefs(useDashboardSidebarStore())

// Log to see when the transition happens
onMounted(() => console.log('Mounted with selectedElement:', selectedElement.value))
watch(selectedElement, (newVal) => console.log('selectedElement changed to:', newVal))
</script>

<template>
  <div class="w-full h-full flex relative">
    <Transition name="fade" mode="out-in">
      <div v-if="selectedElement === 'containers'" class="w-full h-full flex">
        GUAH
      </div>
      <div v-else-if="selectedElement === 'connections'" class="w-full h-full flex overflow-hidden">
        <DashboardDashboardConnections />
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>