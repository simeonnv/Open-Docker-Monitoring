<script setup lang="ts">
import { ref } from 'vue'

const props = defineProps({
  text: {
    type: String,
    default: ''
  }
})

const folded = ref(true)
import { useTimeout } from '@vueuse/core'

const { ready, start } = useTimeout(300, { controls: true })




</script>

<template>

    <div class="gap-1" v-auto-animate>
        <Button @click="() => {if (ready) {folded = !folded; start()} }" variant="ghost" class="w-full flex-row flex p-0 m-0">
            
            <div class="flex flex-row w-full px-2">
                
                <p>{{ text }}</p>
                
                <div class="grow" />
                
                <Icon 
                name="material-symbols:arrow-back-2-rounded" 
                    :class="folded ? 'rotate-90' : '-rotate-90'"
                    class="h-6 w-6 transition-transform"
                />
                
            </div>
            
        </Button>
            
        <div v-if="!folded">
            <slot>

            </slot>
        </div>
            
    </div>

</template>