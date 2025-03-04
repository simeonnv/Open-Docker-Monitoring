<script setup>
import { storeToRefs } from 'pinia';
import { useAuthStore } from '../stores/auth';
import { ref, onMounted } from 'vue';

const { authenticated } = storeToRefs(useAuthStore());
const isHydrated = ref(false); // Track hydration state

// Set isHydrated to true after the component mounts
onMounted(() => {
  isHydrated.value = true;
});
</script>

<template>
  <Sheet key="left">
    <SheetTrigger class="md:hidden" v-if="isHydrated && authenticated">
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
      <SheetDescription class="w-full h-full">
        <div
          class="w-full h-full rounded-2xl ring-1 ring-gray-200 dark:ring-gray-800 divide-gray-200 dark:divide-gray-800"
        >
          <SidebarContent />
        </div>
      </SheetDescription>
    </SheetContent>
  </Sheet>
</template>

<style scoped>
.sheet-content {
  max-width: 100% !important;
  width: 100vw !important;
}
</style>