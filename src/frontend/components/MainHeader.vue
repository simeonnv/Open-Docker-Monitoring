
<template>
    <UContainer class="h-12 ring-1 ring-gray-200 dark:ring-gray-800 divide-gray-200 dark:divide-gray-800 flex flex-row gap-4 bg-gray shadow p-4">
        <div class="flex flex-row gap-4 text-primary font-black font-sans text-xl justify-center align-middle text-center items-center">
            <Icon name="catppuccin:docker" class="w-[40px] h-[40px]  "/>
            <p><span class="text-blue-300">Open</span> Docker Monitoring</p>
        </div>
        <div class="grow"></div>
        <ULink
            to="https://github.com/simeonnv/Open-Docker-Monitoring"
            class="flex items-center align-middle justify-center"
            active-class="text-primary"
            inactive-class="text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200"
        >
            <Icon
                class="text-primary p-1.5 w-[30px] h-[30px]"
                name="mdi:github"
            />
        </ULink>
        <BaseChangeThemeButton size="26px"/>
    </UContainer>
</template>