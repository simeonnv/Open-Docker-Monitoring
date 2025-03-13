import { defineStore } from 'pinia';

export type DashboardSidebar = "containers" | "connections" | "networks" | "volumes" | "webhooks" | "images"

export const useDashboardSidebarStore = defineStore('dashboard_sidebar', {
    state: () => ({
        selectedElement: "containers" as DashboardSidebar
    }),

    actions: {
        async selectElement(name: DashboardSidebar) {
            this.selectedElement = name;
        }
    }
});