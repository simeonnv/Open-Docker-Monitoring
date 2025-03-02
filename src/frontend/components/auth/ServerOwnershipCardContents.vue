<script setup>

  import { storeToRefs } from 'pinia'; // import storeToRefs helper hook from pinia
  import { useAuthStore } from '../stores/auth'; // import the auth store we just created

  const { Login, MainAccountExists,AuthorizeAppOwnership } = useAuthStore(); // use authenticateUser action from  auth store


  const InputedCredentials = reactive({
    username: "",
    password: ""
  })

  let error = ref("")

  const SubmitAuth = async () => {
    const {status, data} = await AuthorizeAppOwnership(InputedCredentials);
    if (status !== "success")
      error.value = status
  }

</script>



<template>
      <CardHeader>
  
        <CardTitle class="flex justify-center text-2xl">
          Confirm Server Ownership
        </CardTitle>
  
        <CardDescription class="text-center">
          Enter enviroment credentials to create a main account
        </CardDescription>
  
      </CardHeader>
  
      <CardContent>
  
        <div class="grid gap-4" v-auto-animate>
  
          <div class="grid gap-2">
            <Label for="email">Username</Label>
            <Input v-model="InputedCredentials.username" type="username" placeholder="env username" required />
          </div>
          
          <div class="grid gap-2">
            <div class="flex items-center">
              <Label for="password">Password</Label>
            </div>
            <Input v-model="InputedCredentials.password" type="password" placeholder="env password" required />
          </div>
          
          <div class="h-8" v-if="error !== ''">
            <p class="text-red-500">{{ error }}</p>
          </div>

          <Button @Click="SubmitAuth" type="submit" class="w-full">
            Confirm
          </Button>
  
        </div>
  
      </CardContent>
  </template>