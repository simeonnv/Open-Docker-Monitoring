
<template>
    <UContainer class=" divide-gray-200 dark:divide-gray-800 flex flex-row gap-4 bg-gray shadow p-4">
        <p class="flex text-primary font-bold text-xl justify-center align-middle text-center items-center">Open Docker Monitoring</p>
        <div class="grow"></div>
        <ULink
            to="https://github.com/simeonnv/Open-Docker-Monitoring"
            class="flex items-center align-middle justify-center"
            active-class="text-primary"
            inactive-class="text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200"
        >
            <Icon
                class="text-primary w-[30px] h-[30px]"
                name="mdi:github"
            />
        </ULink>
        <BaseChangeThemeButton size="26px"/>
    </UContainer>
</template>