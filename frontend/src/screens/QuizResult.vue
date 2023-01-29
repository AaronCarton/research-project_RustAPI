<template>
  <div>
    <div class="flex flex-col items-center">
      <h2 class="text-2xl font-semibold">Your score is:</h2>
      <h2 class="text-3xl font-semibold">{{ score }} / {{ quiz.questions.length }}</h2>
      <RouterLink :to="`/`">
        <button class="bg-red-500 mt-8 text-lg font-semibold text-white py-2 px-6 rounded">
          Home
        </button>
      </RouterLink>
    </div>
  </div>
</template>

<script lang="ts">
import Quiz from '../interfaces/quiz.interface'
import { ref, computed } from 'vue'
import { useRoute } from 'vue-router'
import { useAxios } from '@vueuse/integrations/useAxios'
import useAuthentication from '../composables/useAuthentication'

export default {
  props: {
    quiz: {
      type: Object as () => Quiz,
      required: true,
    },
  },
  async setup() {
    const { params } = useRoute()
    const { user } = useAuthentication()

    const getScore = async () => {
      return useAxios(`${window['env']['API_URL']}/quiz/${params.id}/score`, {
        method: 'GET',
        headers: {
          Authorization: `Bearer ${await user.value?.getIdToken()}`,
        },
      }).then(({ data }) => data.value as number)
    }
    return { score: await getScore() }
  },
}
</script>
