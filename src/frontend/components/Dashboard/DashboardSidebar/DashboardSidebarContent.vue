<template>
    <div class="h-full flex flex-col align-middle w-full items-center pt-4">

        <DashboardDashboardSidebarElement icon_name="lucide:container" element="containers" text="containers" />

        <div class="grow"></div>

        <DashboardDashboardSidebarElement icon_name="mdi:connection" element="connections" text="connections" />

        <div class="grow"></div>

        <DashboardDashboardSidebarElement icon_name="material-symbols:network-node" element="networks" text="networks" />

        <div class="grow"></div>

        <DashboardDashboardSidebarElement icon_name="mdi:harddisk" element="volumes" text="volumes" />

        <div class="grow"></div>

        <DashboardDashboardSidebarElement icon_name="ri:webhook-fill" element="webhooks" text="webhooks" />

        <div class="grow"></div>

        <DashboardDashboardSidebarElement icon_name="ph:blueprint-fill" element="images" text="images" />

    </div>
</template>