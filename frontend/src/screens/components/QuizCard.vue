<template>
  <div class="p-4 flex flex-col border-3 border-[#FDFFFC] items-center card">
    <h1 class="text-xl font-semibold">{{ name }}</h1>
    <p class="text-lg">{{ description }}</p>
    <RouterLink :to="`/quiz/${id}`">
      <button class="bg-red-500 mt-4 text-white py-2 px-4 rounded">Start Quiz</button>
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
