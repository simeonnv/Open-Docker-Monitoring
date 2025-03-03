<script setup>

  import { storeToRefs } from 'pinia'; // import storeToRefs helper hook from pinia
  import { useAuthStore } from '../stores/auth'; // import the auth store we just created

  const { Signup } = useAuthStore(); // use authenticateUser action from  auth store


  const InputedCredentials = reactive({
    username: "",
    password: ""
  })

  let error = ref("")

  const SubmitSignup = async () => {
    const {status, data} = await Signup(InputedCredentials);
    if (status !== "success")
      error.value = status
    else
      return navigateTo('/');
  }

</script>



<template>
  <Card class="mx-auto max-w-sm" >
    <CardHeader>

      <CardTitle class="flex justify-center text-2xl">
        Signup
      </CardTitle>

      <CardDescription class="text-center">
        Enter a username and password to claim your Open Docker Monitoring Server!
      </CardDescription>

    </CardHeader>

    <CardContent>

      <div class="grid gap-4"v-auto-animate>

        <div class="grid gap-2">
          <Label for="email">Username</Label>
          <Input v-model="InputedCredentials.username" type="username" placeholder="Michael" required />
        </div>
        
        <div class="grid gap-2">
          <div class="flex items-center">
            <Label for="password">Password</Label>
          </div>
          <Input v-model="InputedCredentials.password" type="password" placeholder="Super secret password" required />
        </div>
        
        <div class="h-12" v-if="error !== ''">
          <p class="text-red-500">{{ error }}</p>
        </div>

        <Button @click="SubmitSignup" type="submit" class="w-full">
          Login
        </Button>
        
        <Button variant="outline" class="w-full">
          Login with Google
        </Button>

      </div>

    </CardContent>
  </Card>
</template>