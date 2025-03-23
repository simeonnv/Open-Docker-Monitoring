import { defineStore } from 'pinia';

// Interface definitions
export interface ContainerRefinedSummary {
    id: string;
    names: Array<string>; // Use lowercase "string" for TypeScript consistency
    image_id: string;
    command: string;
    created: string;
    ports: Array<string>;
    state: string;
    status: string;
    network: string;
}

export interface GetDockerContainersRes {
    status: string;
    data: Array<ContainerRefinedSummary> | string;
}

export const useContainersStore = defineStore('containers', { // Fixed typo: "Contianers" -> "Containers"
    state: () => ({
        containers: [] as Array<ContainerRefinedSummary>, // Correctly typed as an array of ContainerRefinedSummary
    }),

    actions: {
        async getContainers(): Promise<GetDockerContainersRes> {
            const config = useRuntimeConfig();
            const token = useCookie('token');

            const { data, status }: any = await $fetch(
                `${config.public.backendPublicAddress}:${config.public.backendPort}/docker/containers`, {
                    method: 'get',
                    headers: {
                        'Content-Type': 'application/json',
                        'Authorization': `Bearer ${token.value}`,
                    },
                }
            ).catch((error: any) => {
                return { status: 'error', data: 'Failed to fetch containers' } as GetDockerContainersRes;
            });

            if (data && status === 'success') {
                this.containers = data; // Assign data directly (assuming data is Array<ContainerRefinedSummary>)
            }

            return { data, status };
        },
    },
});