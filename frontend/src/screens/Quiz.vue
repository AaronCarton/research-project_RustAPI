<template>
  <div class="flex-col flex flex-1 items-center justify-center">
    <Suspense>
      <div class="mb-4 flex flex-col w-2/3" v-if="quiz">
        <div class="flex justify-between">
          <p
            :class="`text-2xl font-semibold ${
              currentQuestionIndex == quiz?.questions.length ? 'mx-auto mb-6' : ''
            } `"
          >
            {{ quiz.name }}
          </p>
          <template v-if="currentQuestionIndex < quiz?.questions.length">
            <p class="text-xl font-bold mb-4 ml-3">{{ currentQuestionIndex + 1 }}/8</p>
          </template>
        </div>
        <template v-if="currentQuestion">
          <Question
            v-if="currentQuestionIndex < quiz.questions.length"
            :key="currentQuestionIndex"
            :quiz-id="quiz.id"
            :question-index="currentQuestionIndex"
            :question="currentQuestion"
            :next-question="goToNextQuestion"
          />
        </template>
        <QuizResult v-else :quiz="quiz" />
      </div>
    </Suspense>
  </div>
</template>

<script lang="ts">
import { ref, computed, watch } from 'vue'
import Quiz from '../interfaces/quiz.interface'
import QuestionType from '../interfaces/question.interface'
import Question from './Question.vue'
import QuizResult from './QuizResult.vue'
import { useAxios } from '@vueuse/integrations/useAxios'
import { useToast } from 'vue-toastification'
import { useRoute } from 'vue-router'

export default {
  components: {
    Question,
    QuizResult,
  },
  setup() {
    const { params } = useRoute()
    const { data, isFinished } = useAxios(`${window['env']['API_URL']}/quiz/${params.id}`)
    const quiz = computed(() => data.value as Quiz | undefined)
    const currentQuestionIndex = ref(0)
    const currentQuestion = ref<QuestionType>()

    function goToNextQuestion() {
      currentQuestionIndex.value += 1
    }

    watch(isFinished, () => {
      if (quiz.value) {
        console.log('quiz loaded', quiz.value)

        currentQuestion.value = quiz.value.questions[currentQuestionIndex.value]
      }
    })
    watch(currentQuestionIndex, () => {
      if (quiz.value) {
        currentQuestion.value = quiz.value.questions[currentQuestionIndex.value]
        console.log('next question')
      }
    })

    return {
      quiz,
      currentQuestion,
      currentQuestionIndex,

      goToNextQuestion,
    }
  },
}
</script>
