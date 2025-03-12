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
  key: string
};

export const useAuthStore = defineStore('auth', {
  state: (): useAuthState => ({
    authenticated: false,
    responce: {
      status: "",
      data: ""
    },
    key: ""
  }),
  actions: {
    async AuthorizeAppOwnership({ username, password }: SignupPayloadInterface) {
      const config = useRuntimeConfig();
      console.log("ENV: ", config.public);
      console.log("ADDRESS: ", `${config.public.backendPublicAddress}:${config.public.backendPort}/auth/signup_key`)
      const { data, status }: any = await $fetch(`${config.public.backendPublicAddress}:${config.public.backendPort}/auth/signup_key`, {
        method: 'post',
        headers: { 'Content-Type': 'application/json' },
        body: {
          username,
          password
        },
      })
        .catch((error: any) => error.data);
      console.log({data, status});

      this.key = data

      return {data, status}
    },
    async Signup({ username, password }: SignupPayloadInterface) {
      const config = useRuntimeConfig();
      console.log("ENV: ", config.public);
      console.log("ADDRESS: ", `${config.public.backendPublicAddress}:${config.public.backendPort}/auth/signup`)

      if (!this.key)
        return { data: "", status: "invalid key!"}

      const { data, status }: any = await $fetch(`${config.public.backendPublicAddress}:${config.public.backendPort}/auth/signup`, {
        method: 'post',
        headers: { 'Content-Type': 'application/json' },
        body: {
          username,
          password,
          key: this.key,
        },
      })  
        .catch((error: any) => error.data);
      
      console.log(data);
      const token = useCookie('token');

      if (data) {
        this.responce = {data, status}
        token.value = data;
        // this.key = ""
        this.authenticated = true;
      } else {
        this.responce = {data, status}
        token.value = null
        // this.key = ""
        this.authenticated = false;
      }

      return {data, status}
    },
    async Login({ username, password }: LoginPayloadInterface) {
      const config = useRuntimeConfig();
      console.log("ENV: ", config.public);
      console.log("ADDRESS: ", `${config.public.backendPublicAddress}:${config.public.backendPort}/auth/login`)
      const { data, status }: any = await $fetch(`${config.public.backendPublicAddress}:${config.public.backendPort}/auth/login`, {
        method: 'post',
        headers: { 'Content-Type': 'application/json' },
        body: {
          username,
          password,
        },
      })
        .catch((error: any) => error.data);
      // console.log("DATA: ", data);
      // console.log("ERROR: ", error);
      // console.log("DATA: ", data.value);
      const token = useCookie('token');

      if (data) {
        this.responce = {data, status}
        token.value = data;
        this.authenticated = true;
      } else {
        token.value = null
        this.authenticated = false;
        this.responce = {data, status}
      }

      console.log("authenticated: ", this.authenticated)
      return {data, status}
    },
    Logout() {
      const token = useCookie('token');
      this.authenticated = false;
      token.value = null;
    },
    async ValidateAuth(): Promise<boolean> {
      const config = useRuntimeConfig();
      const token = useCookie('token');
    
      // Check if token exists before making the request
      if (!token.value) {
        this.Logout();
        return false;
      }
    
      try {
        const response = await $fetch(`${config.public.backendPublicAddress}:${config.public.backendPort}/auth/validate`, {
          method: 'get',
          headers: { 
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${token.value}` // Use .value explicitly
          },
          retry: 3,
          retryDelay: 200,
        })
          .catch((error: any) => error.data);

    
        const { data, status } = response as { data: any; status: string }; 
    
        if (!data || status !== 'success') {
          this.Logout();
          return false;
        }
    
        this.authenticated = true;
        return true;
      } catch (error) {
        console.error('ValidateAuth error:', error);
        this.Logout();
        return false;
      }
    },
    async MainAccountExists(): Promise<boolean> {
      const config = useRuntimeConfig();
      const { data }: { data: boolean } = await $fetch<Responce | undefined>(`${config.public.backendPublicAddress}:${config.public.backendPort}/auth/exists`, {
        method: 'GET',
        retry: 3,
        retryDelay: 50,
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