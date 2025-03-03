<template>
    <NuxtLink :to="props.path" :class="isActive ? 'bg-red-500' : ''">
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