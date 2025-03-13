<script setup lang="ts">
    import {
        Card,
        CardContent,
        CardDescription,
        CardFooter,
        CardHeader,
        CardTitle,
    } from '@/components/ui/card'
    import type { DockerInfo } from '@/stores/docker';
    import { defineProps } from 'vue'; 
    import type { PropType } from 'vue'; 

    const props = defineProps({
        docker: {
            type: Object as PropType<[DockerConnection, DockerInfo]>,
            required: true
        },
        name: {
            type: String,
            required: true
        }
    })

    let [docker_connection, docker] = props.docker

</script>


<template>
    <div class="p-4 w-fit">
        {{ docker }}
        {{ docker_connection }}
        <Card class="hover:bg-muted transition-colors duration-500">
          
            <CardHeader class="flex flex-col gap-4 justify-center align-middle items-center">
            <CardTitle class="flex flex-row gap-2"> 
                <span>{{ props.name }}</span>
                <span>{{ docker.server_version || "unknown version!" }}</span> 
            </CardTitle>
            <CardDescription><span>ID:</span> {{ docker.id }}</CardDescription>
          </CardHeader>

          <CardContent class="gap-2 flex flex-col">

            <div v-if="docker.containers">
                <p v-if="docker.containers" ><span>Contianers:</span> {{ docker.containers }}</p>
                <p v-if="docker.containers_running" class="text-muted-foreground"> - <span>Containers Running:</span> {{ docker.containers_running }}</p>
                <p v-if="docker.containers_paused" class="text-muted-foreground"> - <span>Containers Paused:</span> {{ docker.containers_paused }}</p>
                <p v-if="docker.containers_stopped" class="text-muted-foreground"> - <span>Containers Stopped:</span> {{ docker.containers_stopped }}</p>
            </div>

            <p v-if="docker.images" ><span>Images:</span> {{ docker.images }}</p>
            
            <p v-if="docker.os_type" ><span>OS:</span> {{ docker.os_type }}</p>
            
            <p v-if="docker.kernel_version" ><span>Kernel Version:</span> {{ docker.kernel_version }}</p>

            <p v-if="docker.architecture" ><span>Architecture:</span> {{ docker.architecture }}</p>

            <p v-if="docker.ncpu" ><span>CPU Cores:</span> {{ docker.ncpu }}</p>

            <p v-if="docker.mem_total" ><span>Memory:</span> {{ Math.round(docker.mem_total / Math.pow(1024, 3) * 100) / 100 }}GB</p>

          </CardContent>
        </Card>
    </div>
</template>