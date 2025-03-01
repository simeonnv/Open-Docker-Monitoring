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
  
        <div class="grid gap-4">
  
          <div class="grid gap-2">
            <Label for="email">Username</Label>
            <Input type="username" placeholder="env username" required />
          </div>
          
          <div class="grid gap-2">
            <div class="flex items-center">
              <Label for="password">Password</Label>
            </div>
            <Input type="password" placeholder="env password" required />
          </div>
          
          <Button type="submit" class="w-full">
            Confirm
          </Button>
  
        </div>
  
      </CardContent>
  </template>