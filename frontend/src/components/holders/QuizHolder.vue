<template>
  <div class="bg-[#5347F3] min-h-screen flex flex-col text-neutral-100">
    <RouterLink to="/">
      <ArrowLeft class="absolute top-3 left-3 w-8 h-8 text-white" />
    </RouterLink>
    <h1 class="text-3xl font-bold pt-4 text-center">Quiz</h1>
    <router-view class="overflow-y-auto"> </router-view>
  </div>
</template>

<script lang="ts">
import { ArrowLeft } from 'lucide-vue-next'
export default {
  components: {
    ArrowLeft,
  },
  setup() {
    return {}
  },
}
</script>
