<script setup>
const props = defineProps({
  folded: {
    type: Boolean,
    default: false
  },
  icon_name: {
    type: String,
    required: true
  },
  text: {
    type: String,
    default: ""
  }
})
</script>

<template>
  <div class="transition-transform duration-300 ease-in-out align-middle flex flex-row items-center" :class="folded ? 'gap-0' : 'gap-4'">
    <button path="settings" class="flex align-middle">
      <Icon :name="icon_name" class="w-6 h-6"/>
    </button>
    <div :class="folded ? 'w-0' : 'w-20'" class="w-min-fit w-max-fit text-ml overflow-hidden transition-all duration-500 ease-in-out">
      <p> {{ text }} </p>
    </div>
  </div>
</template>