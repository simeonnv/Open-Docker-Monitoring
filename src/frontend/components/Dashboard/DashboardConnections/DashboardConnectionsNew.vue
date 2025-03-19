<script setup lang="ts">
import { reactive, watch } from 'vue';
const { addDocker } = useDockersStore();


// Define the reactive form object
const form = reactive({
  name: "",
  host: "",
  protocol: ""
});

// Watch the protocol property and clear host when it changes to "local"
watch(() => form.protocol, (newProtocol) => {
  if (newProtocol === 'local') {
    form.host = 'localhost';
  }

});

const error = ref("");
const open = ref(false)

const wait = (ms: number) => new Promise(resolve => {
  useTimeoutFn(resolve, ms, { immediate: true });
});

const submit = async () => {
    const { data, status } = await addDocker(form.name, form.host, form.protocol)
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

    } else {
        open.value = !open
    }
}

</script>


<template>
    <Dialog v-model:open="open">
        <DialogTrigger as-child>
            <DashboardDashboardConnectionsNewTrigger />
        </DialogTrigger>
        <DialogContent class="sm:max-w-[425px]">
            <DialogHeader>
                <DialogTitle>Create a new Docker connection</DialogTitle>
                <DialogDescription>
                    Make changes to your profile here. Click save when you're done.
                </DialogDescription>
            </DialogHeader>
            <div class="grid gap-4 py-4">

                <div class="grid grid-cols-4 items-center gap-4">
                    <Label for="name" class="text-right">
                        Name
                    </Label>
                    <Input v-model="form.name" id="name" class="col-span-3" />
                </div>

                <div class="grid grid-cols-4 items-center gap-4">
                    <Label for="host" class="text-right">
                        Host
                    </Label>
                    <Input :disabled="form.protocol === 'local'" v-model="form.host" id="host" class="col-span-3" />
                </div>

                <div class="grid grid-cols-4 items-center gap-4 flex-row">
                    <Label for="protocol" class="text-right col-span-1">
                        Protocol
                    </Label>
                    <Select v-model="form.protocol" id="protocol">
                        <SelectTrigger class="col-span-3">
                            <SelectValue placeholder="Select a connection protocol" />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectGroup>
                                <SelectItem value="local">
                                    localhost
                                </SelectItem>
                                <SelectItem value="http">
                                    http
                                </SelectItem>
                            </SelectGroup>
                        </SelectContent>
                    </Select>
                </div>

            </div>

            <div v-auto-animate class="h-6 min-h-fit transition-all duration-500 ">
                <p v-if="error" class="text-rose-700 text-sm">Error: {{ error }}!</p>
            </div>

            <DialogFooter>


                <Button @click="submit">
                    Add new Docker
                </Button>

            </DialogFooter>
        </DialogContent>
    </Dialog>
</template>