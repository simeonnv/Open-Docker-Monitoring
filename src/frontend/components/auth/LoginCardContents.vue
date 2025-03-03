<script setup>
import { storeToRefs } from 'pinia'; // import storeToRefs helper hook from pinia
import { useAuthStore } from '../stores/auth'; // import the auth store we just created
import { reactive, ref, nextTick } from 'vue'; // import nextTick for DOM updates

const { Login } = useAuthStore(); // use authenticateUser action from auth store

const InputedCredentials = reactive({
    username: "",
    password: ""
});

let error = ref("");

const SubmitLogin = async () => {
    const { status, data } = await Login(InputedCredentials);
    if (status !== "success") {
        error.value = "";
        await nextTick();
        error.value = status;
    } else {
        return navigateTo('/');
    }
};
</script>

<template>
    <Card class="mx-auto max-w-sm">
        <CardHeader>
            <CardTitle class="flex justify-center text-2xl">
                Login
            </CardTitle>
            <CardDescription class="text-center">
                Enter a username and password to enter in your Open Docker Monitoring Server!
            </CardDescription>
        </CardHeader>

        <CardContent>
            <div class="grid gap-4">
                <div class="grid gap-2">
                    <Label for="email">Username</Label>
                    <Input v-model="InputedCredentials.username" type="username" placeholder="Michael" required />
                </div>

                <div class="grid gap-2">
                    <div class="flex items-center">
                        <Label for="password">Password</Label>
                    </div>
                    <Input v-model="InputedCredentials.password" type="password" placeholder="Super secret password"
                        required />
                </div>

                <div class="h-6 flex">
                    <Transition name="slide">
                        <p v-if="error !== ''" class="text-red-500 text-sm">{{ error }}</p>
                    </Transition>
                </div>

                <Button @click="SubmitLogin" type="submit" class="w-full">
                    Login
                </Button>

                <Button variant="outline" class="w-full">
                    Login with Google
                </Button>
            </div>
        </CardContent>
    </Card>
</template>

<style scoped>
.slide-enter-active,
.slide-leave-active {
    transition: transform 0.6s ease, opacity 0.6s ease-in-out;
    opacity: 1;
}

.slide-enter-from,
.slide-leave-to {
    transition: transform 0.6s ease, opacity 0.6s ease-in-out;
    transform: translatex(-10px);
    opacity: 0;
}
</style>