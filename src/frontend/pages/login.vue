<script setup>

import { storeToRefs } from 'pinia'; // import storeToRefs helper hook from pinia
import { useAuthStore } from '../stores/auth'; // import the auth store we just created

const { key } = storeToRefs(useAuthStore()); // make authenticated state reactive with storeToRefs

</script>

<template>
  <div class="w-full h-full justify-center align-middle items-center flex">
    <Card class="mx-auto max-w-sm h-96 flex flex-col justify-center" v-auto-animate >
      <AuthLoginCardContents/>
    </Card>
  </div>
</template>

