<script setup lang="ts">
import { defineProps } from 'vue'
import type { PropType } from 'vue'  // Import PropType
import { useDashboardSidebarStore } from '@/stores/dashboard_sidebar'
import type { DashboardSidebar } from '@/stores/dashboard_sidebar'

const props = defineProps({
  icon_name: {
    type: String,
    required: true
  },
  text: {
    type: String,
    default: ""
  },
  element: {
    type: String as PropType<DashboardSidebar>,  // Use PropType for the custom type
    required: true
  }
})

const { selectElement } = useDashboardSidebarStore()
const { selectedElement } = storeToRefs(useDashboardSidebarStore())

</script>

<template>
  <div class="p-1 rounded-md m-4 transition-all duration-500 ease-in-out align-middle flex flex-row items-center justify-center">
    <Button :class="selectedElement === element ? 'bg-muted' : ''" variant="ghost" @click="selectElement(element)" class="hover:transition-colors rounded-xl p-2 flex items-center gap-2 justify-center align-middle transition-all duration-500">
      <Icon :name="icon_name" class="w-6 h-6 text-primary"/>
      <p class="text-primary font-bold">{{ text.charAt(0).toUpperCase() + text.slice(1) }}</p>
    </Button>
  </div>
</template>
