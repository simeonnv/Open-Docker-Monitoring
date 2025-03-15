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
import Foldable from '@/components/ui/Foldable/Foldable.vue';

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
        <Card>
          
            <CardHeader class="px-8 py-6 flex flex-col gap-4">
                
                <CardTitle class="flex flex-row text-primary gap-2 justify-center align-middle items-center"> 
                    <span>{{ props.name }}</span>
                    <span>{{ docker.server_version || "unknown version!" }}</span> 
                </CardTitle>

                <CardDescription class="text-text text-md">

                    <p>ID: {{ docker.id }}</p>
                
                    <p>Protocol: {{docker_connection.protocol}}</p>

                    <p>Host address: {{docker_connection.host}}</p>

                </CardDescription>

            </CardHeader>


          <CardContent class="gap-1 flex flex-col">
            <Foldable text="Show more information" class="text-muted-foreground">

            <div v-if="docker.containers !== undefined" class="flex flex-col">
                <p v-if="docker.containers" >Contianers: {{ docker.containers }}</p>
                <p v-if="docker.containers_running !== undefined" class="text-muted-foreground"> - Containers Running: {{ docker.containers_running }}</p>
                <p v-if="docker.containers_paused !== undefined" class="text-muted-foreground"> - Containers Paused: {{ docker.containers_paused }}</p>
                <p v-if="docker.containers_stopped !== undefined" class="text-muted-foreground"> - Containers Stopped: {{ docker.containers_stopped }}</p>
            </div>

            <p v-if="docker.images" >Images: {{ docker.images }}</p>
            
            <p v-if="docker.os_type" >OS: {{ docker.os_type }}</p>
            
            <p v-if="docker.kernel_version" >Kernel Version: {{ docker.kernel_version }}</p>

            <p v-if="docker.architecture" >Architecture: {{ docker.architecture }}</p>

            <p v-if="docker.ncpu" >CPU Cores: {{ docker.ncpu }}</p>

            <p v-if="docker.mem_total" >Memory: {{ Math.round(docker.mem_total / Math.pow(1024, 3) * 100) / 100 }}GB</p>

            </Foldable>
          </CardContent>
        </Card>
</template>