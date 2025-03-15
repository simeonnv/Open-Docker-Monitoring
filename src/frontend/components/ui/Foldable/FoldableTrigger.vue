<script setup lang="ts">
defineProps({
  folded: {
    type: Boolean,
    default: false
  }
})

// Define emit for the parent to handle
const emit = defineEmits(['update:folded'])
</script>

<template>
  <Button @click="emit('update:folded', !folded)" variant="ghost" class="p-0 m-0">
    <Icon 
      name="material-symbols:arrow-back-2-rounded" 
      :class="folded ? 'rotate-90' : '-rotate-90'"
      class="h-6 w-6 transition-transform"
    />
  </Button>
</template>