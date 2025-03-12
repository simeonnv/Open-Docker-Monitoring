<script setup lang="ts">
    import {
        Card,
        CardContent,
        CardDescription,
        CardFooter,
        CardHeader,
        CardTitle,
    } from '@/components/ui/card'
    import type { DockerInfo } from '../../stores/docker';
    import { defineProps } from 'vue'; 
    import type { PropType } from 'vue'; 

    const props = defineProps({
        docker: {
            type: Object as PropType<DockerInfo>,
            required: true
        },
        name: {
            type: String,
            required: true
        }
    })
</script>


<template>
    <div class="flex-none basis-[calc(50%-0.5rem)] p-4">
        <Card class="hover:bg-muted transition-colors duration-500">
          <CardHeader class="flex flex-col gap-4 justify-center align-middle items-center">
            <CardTitle> <span>{{ props.name }}</span> <span>{{ props.docker.server_version }}</span> </CardTitle>
            <CardDescription><span>ID:</span> {{ props.docker.id }}</CardDescription>
          </CardHeader>
          <CardContent class="gap-2 flex flex-col">

            <div v-if="props.docker.containers">
                <p><span>Contianers:</span> {{ props.docker.containers }}</p>
                <p class="text-muted-foreground"> - <span>Containers Running:</span> {{ props.docker.containers_running }}</p>
                <p class="text-muted-foreground"> - <span>Containers Paused:</span> {{ props.docker.containers_paused }}</p>
                <p class="text-muted-foreground"> - <span>Containers Stopped:</span> {{ props.docker.containers_stopped }}</p>
            </div>

            <p><span>Images:</span> {{ props.docker.images }}</p>
            
            <p><span>OS:</span> {{ props.docker.os_type }}</p>
            
            <p><span>Kernel Version:</span> {{ props.docker.kernel_version }}</p>

            <p><span>Architecture:</span> {{ props.docker.architecture }}</p>

            <p><span>CPU Cores:</span> {{ props.docker.ncpu }}</p>

            <p v-if="props.docker.mem_total" ><span>Memory:</span> {{ Math.round(props.docker.mem_total / Math.pow(1024, 3) * 100) / 100 }}GB</p>

          </CardContent>
        </Card>
    </div>
</template>