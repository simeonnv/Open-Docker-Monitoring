            
<script setup>
    
    const route = useRoute()
    
    const isAuthRoute = computed(() => {
        return route.path === '/login' || route.path === '/signup'
    })
    
    const isHydrated = ref(false);
    
    onMounted(() => {
        isHydrated.value = true;
    });

</script>

<template>
    <div
        class="h-12 ring-1 ring-gray-200 dark:ring-gray-800 divide-gray-200 dark:divide-gray-800 flex flex-row gap-4 bg-gray shadow p-4 py-8">
        <div
            class="flex flex-row gap-4 text-primary font-black font-sans text-xl justify-center align-middle text-center items-center">
            <Icon name="catppuccin:docker" class="w-[40px] h-[40px]" />
            <p>
                <span class="hidden md:inline"><span class="text-blue-300">Open</span> Docker Monitoring</span>
                <span class="md:hidden">ODM</span>
            </p>
        </div>

        <div class="grow"></div>

        <div v-auto-animate class="flex flex-col justify-center align-middle items-center">
            <div v-if="isHydrated && !isAuthRoute">
                <SidebarPhone class="md:hidden" />
            </div>
        </div>

        <div class="flex flex-col justify-center align-middle items-center">
            <Button class="flex !w-min !h-min items-center align-middle justify-center" variant="link">
                <Icon href="https://github.com/simeonnv/Open-Docker-Monitoring" class="text-primary w-[30px] h-[30px]"
                    name="mdi:github" />
            </Button>
        </div>
        <BaseChangeThemeButton size="26px" />
    </div>
</template>