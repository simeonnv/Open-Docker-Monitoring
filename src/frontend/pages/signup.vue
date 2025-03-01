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
  <div class="w-full h-full aling-middle justify-center flex items-center">
    <UCard class="w-full max-w-md p-6">
      <UForm class="flex flex-col gap-4">
        <div>
          <label for="username">Username: </label>
          <UInput v-model="user.username" type="text" placeholder="Enter your username" required />
        </div>

        <div>
          <label for="password">Password: </label>
          <UInput id="password" v-model="user.password" type="password" placeholder="Enter your password" required />
        </div>

        <UButton @click="signup" type="button" class="flex justify-center items-center">
          Log In
        </UButton>
      </UForm>
    </UCard>
  </div>
</template>
