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

      if (data.value) {
        const token = useCookie('token');
        this.responce = data.value
        token.value = data?.value?.data;
        this.authenticated = true;
      } else {
        this.responce = error.value
      }
    },
    async Login({ username, password }: LoginPayloadInterface) {
      const config = useRuntimeConfig();
      console.log("ENV: ", config.public);
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

      if (data.value) {
        const token = useCookie('token');
        this.responce = data.value
        token.value = data?.value?.data;
        this.authenticated = true;
      } else {
        this.responce = error.value.data
      }
    },
    Logout() {
      const token = useCookie('token');
      this.authenticated = false;
      token.value = null;
    },
  },
});