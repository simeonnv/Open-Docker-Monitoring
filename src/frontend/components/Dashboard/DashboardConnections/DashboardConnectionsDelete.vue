<script setup lang="ts">
import { reactive, watch } from 'vue';
const { removeDocker } = useDockersStore();

const props = defineProps({
    name: {
        type: String,
        required: true
    }
})

const wait = (ms: number) => new Promise(resolve => {
    useTimeoutFn(resolve, ms, { immediate: true });
});

const error = ref('')
const open = ref(false)

const submit = async () => {
    const { data, status } = await removeDocker(props.name)
    if (status !== "success") {
        if (error.value === "") {
            error.value = status
            return
        } else {
            error.value = ""
            await wait(310)
            error.value = status
            return
        }

    }
}

</script>


<template>
    <Dialog v-model:open="open">

        <DialogTrigger as-child>
            <DashboardDashboardConnectionsDeleteTrigger />
        </DialogTrigger>

        <DialogContent class="sm:max-w-92">
            <DialogHeader class="gap-4 pt-4">
                <DialogTitle class="text-center">Are you sure you want to delete this docker connection?</DialogTitle>
                <DialogDescription class="text-center">
                    This cannot be undone. Make sure you a certain.
                </DialogDescription>
            </DialogHeader>

            <div v-auto-animate class="h-6 min-h-fit transition-all duration-500 ">
                <p v-if="error" class="text-rose-700 text-sm">Error: {{ error }}!</p>
            </div>

            <DialogFooter class="font-bold">

                <Button @Click="open = !open" variant="outline" class="bg-teal-500 hover:bg-teal-600 text-white">
                    Back
                </Button>

                <div class="grow"></div>

                <Button variant="destructive" @Click="submit">
                    Confirm
                </Button>

            </DialogFooter>
        </DialogContent>
    </Dialog>
</template>