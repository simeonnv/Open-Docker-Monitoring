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

let nz = ref(1)

</script>

<template>
  <div class="w-full h-full justify-center align-middle items-center flex">
    <Button @click="nz = (nz + 1) % 2">
      +++
    </Button>
    <Card class="mx-auto max-w-sm h-96 flex flex-col justify-center" v-auto-animate >
      <AuthServerOwnershipCardContents v-if="nz === 1"/>
      <AuthSignupCardContents v-else/>
    </Card>
  </div>
</template>

