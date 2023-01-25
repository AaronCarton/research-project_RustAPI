<template>
  <div class="bg-gray-200 p-4 shadow-md flex flex-col items-center rounded-lg">
    <h1 class="text-xl font-semibold">{{ name }}</h1>
    <p class="text-lg">{{ description }}</p>
    <RouterLink :to="`/quiz/${id}`">
      <button class="bg-red-500 mt-4 text-white p-2 rounded">Start Quiz</button>
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
