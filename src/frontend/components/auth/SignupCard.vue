<template>
  <Card class="mx-auto max-w-sm">
    <CardHeader>

      <CardTitle class="flex justify-center text-2xl">
        Signup
      </CardTitle>
      <CardDescription class="text-center">
        Enter a username and password to claim your Open Docker Monitoring Server!
      </CardDescription>
    </CardHeader>
    <CardContent>
      <div class="grid gap-4">
        <div class="grid gap-2">
          <Label for="email">Username</Label>
          <Input
            type="username"
            placeholder="Michael"
            required
          />
        </div>
        <div class="grid gap-2">
          <div class="flex items-center">
            <Label for="password">Password</Label>
          </div>
          <Input id="password" type="password" required />
        </div>
        <Button type="submit" class="w-full">
          Login
        </Button>
        <Button variant="outline" class="w-full">
          Login with Google
        </Button>
      </div>
      <div class="mt-4 text-center text-sm">
        Don't have an account?
        <a href="#" class="underline">
          Sign up
        </a>
      </div>
    </CardContent>
  </Card>
</template>