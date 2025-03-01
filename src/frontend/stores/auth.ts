import { defineStore } from 'pinia';

interface SignupPayloadInterface {
  username: string;
  password: string;
  key: string;
}

interface LoginPayloadInterface {
  username: string;
  password: string;
}

interface Responce {
  status: string;
  data: string;
}

type useAuthState = {
  authenticated: boolean;
  responce: Responce; // Updated to allow empty object
  loading: boolean;
};

export const useAuthStore = defineStore('auth', {
  state: (): useAuthState => ({
    authenticated: false,
    responce: {
      status: "",
      data: ""
    },
    loading: false,
  }),
  actions: {
    async Signup({ username, password, key }: SignupPayloadInterface) {
      const config = useRuntimeConfig();
      console.log("ENV: ", config.public);
      console.log("ADDRESS: ", `${config.public.backendPublicAddress}:${config.public.backendPort}/auth/signup`)
      const { data, error, pending }: any = await useFetch(`${config.public.backendPublicAddress}:${config.public.backendPort}/auth/signup`, {
        method: 'post',
        headers: { 'Content-Type': 'application/json' },
        body: {
          username,
          password,
          key,
        },
      });
      this.loading = pending;
      console.log(data);
      const token = useCookie('token');

      if (data.value) {
        this.responce = data.value
        token.value = data?.value?.data;
        this.authenticated = true;
      } else {
        token.value = null
        this.authenticated = false;
        this.responce = error.value.data
      }
    },
    async Login({ username, password }: LoginPayloadInterface) {
      const config = useRuntimeConfig();
      console.log("ENV: ", config.public);
      console.log("ADDRESS: ", `${config.public.backendPublicAddress}:${config.public.backendPort}/auth/signup`)
      const { data, pending, error }: any = await useFetch(`${config.public.backendPublicAddress}:${config.public.backendPort}/auth/login`, {
        method: 'post',
        headers: { 'Content-Type': 'application/json' },
        body: {
          username,
          password,
        },
      });
      this.loading = pending;
      // console.log("DATA: ", data);
      // console.log("ERROR: ", error);
      // console.log("DATA: ", data.value);
      const token = useCookie('token');

      if (data.value) {
        this.responce = data.value
        token.value = data?.value?.data;
        this.authenticated = true;
      } else {
        token.value = null
        this.authenticated = false;
        this.responce = error.value.data
      }
    },
    Logout() {
      const token = useCookie('token');
      this.authenticated = false;
      token.value = null;
    },
    async ValidateAuth(): Promise<boolean> {
      const config = useRuntimeConfig();
      const token = useCookie('token');
      const { data, pending, error }: any = await useFetch(`${config.public.backendPublicAddress}:${config.public.backendPort}/auth/validate`, {
        method: 'get',
        headers: { 
          'Content-Type': 'application/json',
          'Authorization': `Bearer: ${token}` 
        }
      });
      if (error.value) {
        this.Logout()
        return false
      }

      return true
    },
    async MainAccountExists(): Promise<boolean> {
      const config = useRuntimeConfig();
      const { data }: { data: boolean } = await $fetch<Responce | undefined>(`${config.public.backendPublicAddress}:${config.public.backendPort}/auth/exists`, {
        method: 'GET'
      })
        .catch((error: any) => error.data);;

      if (data === undefined) {
        this.Logout()
        return true
      }

      return data
    },

  },
});