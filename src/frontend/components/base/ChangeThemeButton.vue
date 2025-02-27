<template>
    <div>
      <UButton @click="toggleTheme" class="relative" color="gray" variant="ghost" square>
        <transition name="fade">
          <Icon
            v-if="$colorMode.preference === 'light'"
            class="absolute text-primary"
            :size
            name="material-symbols:clear-day-rounded"
          />
          <Icon
            v-else
            class="absolute text-primary"
            :size
            name="material-symbols:nightlight-rounded"
          />
        </transition>
        <!-- Invisible spacer to set button size -->
      <Icon class="invisible" :size name="material-symbols:clear-day-rounded" />
      </UButton>
    </div>
  </template>
  
  <script>
  export default {
    props: {
      size: {
        type: String,
        default: '32px', // Default size if none provided
      },
    },
    methods: {
      toggleTheme() {
        // Toggle between 'light' and 'dark' modes
        this.$colorMode.preference = this.$colorMode.preference === 'light' ? 'dark' : 'light';
      },
    },
  };
  </script>
  
  <style>
  .fade-enter-active,
  .fade-leave-active {
    transition: all 0.5s ease;
  }
  .fade-enter-from {
    opacity: 0;
  }
  .fade-leave-to {
    opacity: 0;
  }
  .fade-enter-active,
  .fade-leave-active {
    position: absolute;
  }
  </style>