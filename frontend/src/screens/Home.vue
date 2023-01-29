<template>
  <div class="flex">
    <div class="flex flex-col mx-auto mt-10 text-center text-lg font-semibold">
      <XCircle
        @click="resetUserData"
        class="absolute top-3 right-5 text-red-500 h-9 w-9 cursor-pointer active:text-red-600"
      />
      <p class="text-3xl mb-3">
        Hi there,
        <span v-if="userData" class="capitalize">{{ userData?.username }}</span>
        <span
          @click="$router.push('/auth/login')"
          v-else
          class="text-red-600 cursor-pointer hover:text-red-700"
          >please login</span
        >
        !
      </p>
      <p class="text-1xl">Your current total score is:</p>
      <p class="text-4xl">{{ userData?.score }}</p>
    </div>
  </div>
  <div class="container mx-auto mt-20 flex flex-wrap justify-center">
    <QuizCard
      v-for="quiz in quizzes"
      class="flex gap-4 m-6 w-1/3"
      :key="quiz.id"
      :name="quiz.name"
      :description="quiz.description"
      :id="quiz.id"
    />
  </div>
</template>

<script lang="ts">
import QuizCard from './components/QuizCard.vue'
import { useToast } from 'vue-toastification'
import axios from 'axios'
import Quiz from '../interfaces/quiz.interface'
import User from '../interfaces/user.interface'
import { ref } from 'vue'
import { useAxios } from '@vueuse/integrations/useAxios'
import useAuthentication from '../composables/useAuthentication'
import { XCircle } from 'lucide-vue-next'
export default {
  components: {
    XCircle,
    QuizCard,
  },
  async setup() {
    const toast = useToast()
    const quizzes = ref<Quiz[]>([])
    const { user } = useAuthentication()
    const token = await user.value?.getIdToken()
    const userData = ref<User>()

    const getUserData = () => {
      if (!token) return
      useAxios(`${window['env']['API_URL']}/users/self`, {
        headers: {
          Accept: 'application/json',
          Authorization: `Bearer ${token}`,
        },
      }).then(({ data }) => (userData.value = data.value as User))
    }
    getUserData()

    const resetUserData = () => {
      useAxios(`${window['env']['API_URL']}/users/self/reset`, {
        method: 'DELETE',
        headers: {
          Accept: 'application/json',
          Authorization: `Bearer ${token}`,
        },
      }).then(({ data }) => (userData.value = data.value as User))
    }

    const allQuizzes = () => {
      axios
        .get(`${window['env']['API_URL']}/quiz`, {
          headers: {
            Accept: 'application/json',
          },
        })
        .then(({ data, status }) => {
          if (status !== 200) {
            toast.error('Something went wrong')
            console.log(data)
            return null
          }
          console.log(data)

          quizzes.value = data as Quiz[]
        })
    }

    allQuizzes()

    return {
      quizzes,
      userData,
      resetUserData,
    }
  },
}
</script>
