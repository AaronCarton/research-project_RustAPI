<template>
  <div>Home</div>
  <div v-for="quiz in quizzes" class="flex gap-4 m-6">
    <QuizCard :name="quiz.name" :description="quiz.description" :id="quiz.id" />
  </div>
</template>

<script lang="ts">
import QuizCard from './components/QuizCard.vue'
import { useToast } from 'vue-toastification'
import axios from 'axios'
import Quiz from '../interfaces/quiz.interface'
import { watch, ref } from 'vue'
export default {
  components: {
    QuizCard,
  },
  setup() {
    const toast = useToast()
    const quizzes = ref<Quiz[]>([])
    // const { user } = useAuthentication()

    // const question = ref('What is the capital of France?')
    // const answers = ref([
    //   { text: 'Paris', isCorrect: true },
    //   { text: 'Rome', isCorrect: false },
    //   { text: 'Madrid', isCorrect: false },
    //   { text: 'Berlin', isCorrect: false },
    // ])

    // function nextQuestion() {
    //   toast.success('Correct!')
    // }

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

    // const getQuiz = async () => {
    //   const { data, status } = await axios.get('http://127.0.0.1:8000/api/quiz/1', {
    //     headers: {
    //       Accept: 'application/json',
    //     },
    //   })

    //   if (status !== 200) {
    //     toast.error('Something went wrong')
    //     console.log(data)
    //     return null
    //   }

    //   return data as Quiz
    // }

    return {
      quizzes,
    }
  },
}
</script>
