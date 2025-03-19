<script setup lang="ts">
    import {
        Card,
        CardContent,
        CardDescription,
        CardFooter,
        CardHeader,
        CardTitle,
    } from '@/components/ui/card'
    import type { DockerRealtimeConnection } from '@/stores/docker';
    import { defineProps } from 'vue'; 
    import type { PropType } from 'vue'; 
    import Foldable from '@/components/ui/Foldable/Foldable.vue';

    const props = defineProps({
        docker: {
            type: Object as PropType<DockerRealtimeConnection>,
            required: true
        },
        name: {
            type: String,
            required: true
        }
    })


</script>


<template>
    <Card>
      <CardHeader class="px-8 py-6 flex flex-col gap-4 relative"> <!-- Added relative to CardHeader -->
        <CardTitle class="flex flex-row justify-center items-center gap-4 align-middle text-center">
          <span>{{ props.name }}</span>
          <span :class="docker.connected ? 'text-teal-400' : 'text-rose-400'">{{ docker.connected ? "Online" : "Offline" }}</span>
        </CardTitle>
  
        <div class="absolute top-6 right-8">
          <DashboardDashboardConnectionsDelete :name/>
        </div>
  
        <!-- Error message below -->
        <p class="text-rose-400/60 text-sm">{{ docker.connection_error }}</p>
  
        <CardDescription class="text-text text-xl">
          <p>Protocol: {{docker.protocol}}</p>
          <p>Host address: {{docker.host}}</p>
        </CardDescription>
      </CardHeader>
    </Card>
  </template>