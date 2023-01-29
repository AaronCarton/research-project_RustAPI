<template>
  <div class="p-4 flex flex-col border-3 border-[#FDFFFC] items-center card">
    <div>
      <h1 class="text-xl font-semibold mb-1 text-center">{{ name }}</h1>
      <p class="text-lg mb-auto">{{ description }}</p>
    </div>
    <RouterLink :to="`/quiz/${id}`" class="mt-auto">
      <button class="bg-red-500 hover:bg-red-600 text-white py-2 px-4 rounded">Start Quiz</button>
    </RouterLink>
  </div>
</template>

<script lang="ts">
import Quiz from '../../interfaces/quiz.interface'

export default {
  props: {
    name: {
      type: String,
      required: true,
    },
    description: {
      type: String,
      required: true,
    },
    id: {
      type: Number,
      required: true,
    },
  },
  setup() {
    return {}
  },
}
</script>

<style>
.card {
  border-radius: 20px;
  background: #4f42ff;
  box-shadow: 7px 7px 16px #4338d9, -7px -7px 16px #5b4cff;
}
</style>
