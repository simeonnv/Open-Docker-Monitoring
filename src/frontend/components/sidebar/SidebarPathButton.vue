<template>
    <NuxtLink :to="props.path" class="hover:transition-colors duration-500 rounded-xl p-2 hover:bg-muted/60" :class="isActive ? 'bg-muted/60' : ''">
        <slot>

        </slot>
    </NuxtLink>
</template>

<script setup>
const props = defineProps({
    path: {
        type: String,
        required: true
    }
})
const route = useRoute()
const isActive = computed(() => route.path === props.path)
console.log(route.path)
console.log(isActive.value)

</script>