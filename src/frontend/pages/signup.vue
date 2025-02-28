<script setup>

import { storeToRefs } from 'pinia'; // import storeToRefs helper hook from pinia
import { useAuthStore } from '../stores/auth'; // import the auth store we just created

const { Login } = useAuthStore(); // use authenticateUser action from  auth store

const { authenticated, responce } = storeToRefs(useAuthStore()); // make authenticated state reactive with storeToRefs

const user = ref({
  username: 'kminchelle', 
  password: '0lelplR',
});
const router = useRouter();

const signup = async () => {
  await Login(user.value); // call authenticateUser and pass the user object
  // redirect to homepage if user is authenticated
  console.log("RESPONCE: ", responce.value)
  if (authenticated) {
    router.push('/');
  }
};

</script>

<template>
  <div>
    <h2>Login</h2>
    <form>
      <div>
        <label for="email">Email:</label>
        <input
          id="email"
          v-model="user.username"
          type="email"
          placeholder="Enter your email"
          required
        >
      </div>
      <div>
        <label for="password">Password:</label>
        <input
          id="password"
          v-model="user.password"
          type="password"
          placeholder="Enter your password"
          required
        >
      </div>

      <p>{{ data, status, error }}</p>

      <button @click="signup" type="button">
        log
      </button>
    </form>
    <p v-if="error" style="color: red;">
      {{ error }}
    </p>
  </div>
</template>

<style scoped>
div {
  max-width: 400px;
  margin: 20px auto;
}
label {
  display: block;
  margin-bottom: 5px;
}
input {
  width: 100%;
  padding: 8px;
  margin-bottom: 10px;
}
button {
  padding: 10px 20px;
  background-color: #007bff;
  color: white;
  border: none;
  cursor: pointer;
}
button:disabled {
  background-color: #cccccc;
}
</style>