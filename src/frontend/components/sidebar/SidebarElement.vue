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
  },
  path: {
    type: String,
    default: ""
  }
})
</script>

<template>
  <div class="m-2  transition-all duration-500 ease-in-out align-middle flex flex-row items-center justify-center">
    <SidebarPathButton :path="path" class="flex items-center justify-center align-middle transition-all duration-500" :class="folded ? ' gap-0 ' : ' gap-4 '">
      <Icon :name="icon_name" class="w-6 h-6 text-primary"/>
      <div :class="folded ? 'w-0 duration-500' : 'w-28 duration-200'" class="text-primary font-bold w-min-fit w-max-fit text-md text-center overflow-hidden transition-all ease-in-out">
        <p> {{ text.charAt(0).toUpperCase() + text.slice(1) }} </p>
      </div>
    </SidebarPathButton>
  </div>
</template>