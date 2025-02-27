<script setup>
import { ref } from 'vue'

// Reactive variables for form inputs
const userForm = reactive({
    email: '',
    password: '',
})

const { data, status, error, execute, clear } = await useFetch('http://localhost:6004/login', {
    method: "POST",
    lazy: true,
    immediate: false,
    default: null,
    server: false,
    body: {
        username: userForm.email,
        password: userForm.password
    }
})

</script>

<template>
  <div>
    <h2>Login</h2>
    <form @submit.prevent="handleLogin">
      <div>
        <label for="email">Email:</label>
        <input
          id="email"
          v-model="userForm.email"
          type="email"
          placeholder="Enter your email"
          required
        >
      </div>
      <div>
        <label for="password">Password:</label>
        <input
          id="password"
          v-model="userForm.password"
          type="password"
          placeholder="Enter your password"
          required
        >
      </div>

      <p>{{ data, status, error }}</p>

      <button @click="execute" type="submit">
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