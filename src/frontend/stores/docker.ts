import { defineStore } from 'pinia';

export interface DockerInfo {
  id?: string | null;
  server_version?: string | null;
  containers?: number | null;
  containers_running?: number | null;
  containers_paused?: number | null;
  containers_stopped?: number | null;
  images?: number | null;
  kernel_version?: string | null;
  os_type?: string | null;
  architecture?: string | null;
  ncpu?: number | null;
  mem_total?: number | null;
}

export interface GetDockerRes {
  status: string;
  data: Record<string, DockerInfo>; 
}

export const useDockersStore = defineStore('dockers', {
  state: () => ({
    dockers: {} as Record<string, DockerInfo>,
    selectedDocker: null as DockerInfo | null, 
    loading: false,       
    error: null,
  }),

  actions: {
    async fetchDockers() {
      this.loading = true;

      const config = useRuntimeConfig();
      const token = useCookie('token');
      
      const response = await $fetch(`${config.public.backendPublicAddress}:${config.public.backendPort}/docker`, {
        method: 'get',
        headers: { 
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token.value}` // Use .value explicitly
        },
        retry: 3,
        retryDelay: 200,
      })
        .catch((error: any) => error.data);

      const { data, status } = response as GetDockerRes;

      this.dockers = data

    },

    selectDocker(name: string) {
      this.selectedDocker = this.dockers[name] || null;
    },

    clearSelectedDocker() {
      this.selectedDocker = null;
    },
  },

  getters: {
    getDockers: (state) => state.dockers,
    getSelectedDocker: (state) => state.selectedDocker,
    isLoading: (state) => state.loading,
  },
});