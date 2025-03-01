<script setup>

import { storeToRefs } from 'pinia'; // import storeToRefs helper hook from pinia
import { useAuthStore } from '../stores/auth'; // import the auth store we just created

const { Login } = useAuthStore(); // use authenticateUser action from  auth store

const { authenticated, responce } = storeToRefs(useAuthStore()); // make authenticated state reactive with storeToRefs

const user = ref({
  username: '',
  password: '',
});
const router = useRouter();

const signup = async () => {
  console.error("AUTHED?: ", authenticated.value)
  await Login(user.value);
  console.error("AUTHED?: ", authenticated.value)

  console.log("RESPONCE: ", responce.value)
  if (authenticated.value) {
    router.push('/');
  }
};

</script>

<template>

<div class=" w-full p-[4vw] justify-center align-middle items-center flex">
  <AuthSignupCard/>
</div>

</template>

