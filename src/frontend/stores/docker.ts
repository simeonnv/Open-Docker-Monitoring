import { defineStore } from 'pinia';

// Interface for Docker connection
export interface DockerRealtimeConnection {
    name: string;
    host: string;
    protocol: string;
    connected: boolean;
    connection_error: string | null;
}

// Response type for GET /docker
export interface GetDockerRes {
    status: string;
    data: Record<string, DockerRealtimeConnection>;
}

// Response type for POST /docker
export interface AddDockerRes {
    status: string;
    data: string; // Assuming the API returns a string (e.g., a message or ID)
}

export const useDockersStore = defineStore('dockers', {
    state: () => ({
        dockers: {} as Record<string, DockerRealtimeConnection>,
        selectedDocker: null as DockerRealtimeConnection | null,
        error: null,
    }),

    actions: {
        async fetchDockers() {
            const config = useRuntimeConfig();
            const token = useCookie('token');

            const response = await $fetch<GetDockerRes>(`${config.public.backendPublicAddress}:${config.public.backendPort}/docker`, {
                method: 'get',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${token.value}`,
                },
                retry: 3,
                retryDelay: 200,
            }).catch((error: any) => {
                console.error('Fetch dockers failed:', error);
                return { status: 'error', data: {} } as GetDockerRes; // Fallback
            });

            const { data, status } = response;

            if (status === 'success') {
                this.dockers = data;
            }
        },

        async addDocker(name: string, host: string, protocol: string) {
            const config = useRuntimeConfig();
            const token = useCookie('token');
        
            const response = await $fetch<AddDockerRes>(`${config.public.backendPublicAddress}:${config.public.backendPort}/docker`, {
                method: 'post',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${token.value}`,
                },
                body: {
                    name,
                    host,
                    protocol,
                },
            }).catch((error: any) => {
                console.error('Add docker failed:', error);
                // Use ?? to provide a fallback string, ensuring data is always a string
                return {status: error.data.status ?? 'failed to add a new docker', data: ""} as AddDockerRes;
            });
        
            const { data, status } = response;
        
            if (status === 'success') {
                await this.fetchDockers(); // Refresh the list
            }
        
            return { data, status } as AddDockerRes;
        },

        selectDocker(name: string) {
            this.selectedDocker = this.dockers[name] || null;
        },

        clearSelectedDocker() {
            this.selectedDocker = null;
        },
    },

    getters: {
        getDockers: (state): Record<string, DockerRealtimeConnection> => state.dockers,
        getSelectedDocker: (state): DockerRealtimeConnection | null => state.selectedDocker,
        hasOnlyOneDocker: (state): boolean => Object.keys(state.dockers).length === 1,
        isADockerSelected: (state): boolean => state.selectedDocker !== null,
    },
});