<script setup>

import { storeToRefs } from 'pinia'; // import storeToRefs helper hook from pinia
import { useAuthStore } from '../stores/auth'; // import the auth store we just created

const { authenticated } = storeToRefs(useAuthStore()); // make authenticated state reactive with storeToRefs



</script>



<template>
    <Sheet key="left">
        <SheetTrigger class="md:hidden" v-if="authenticated">
            <Button 
                class="flex !w-min !h-min items-center align-middle justify-center"
                variant="link"
            >
                <Icon
                    class="text-primary w-[30px] h-[30px]"
                    name="game-icons:hamburger-menu"
                />
            </Button>
        </SheetTrigger>
        <SheetContent class="!w-full" side="left">
            <div class="bg-red-500 w-full h-full">
                <SidebarContent />
            </div>
        </SheetContent>
    </Sheet>
</template>

<style scoped>
/* Target the SheetContent's rendered class if needed */
.sheet-content {
    max-width: 100% !important;
    width: 100vw !important;
}
</style>
