<template>
  <div class="bg-gray-200 p-4 flex flex-column">
    <div class="flex justify-between">
      <h1 class="text-xl font-bold mb-4">Quiz</h1>
      <h1 class="text-lg font-bold mb-4 ml-3">
        {{
          currentQuestionIndex < quiz.questions.length
            ? currentQuestionIndex + 1
            : quiz.questions.length
        }}/8
      </h1>
    </div>
    <Suspense>
      <div class="mb-4 flex flex-col" v-if="quiz">
        <p class="text-lg ml-3 font-semibold">{{ quiz.name }}</p>
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
    const { data, isFinished } = useAxios(`http://127.0.0.1:8000/api/quiz/${params.id}`)
    const quiz = computed(() => data.value as Quiz | undefined)
    const currentQuestionIndex = ref(0)
    const currentQuestion = ref<QuestionType>()

    function goToNextQuestion() {
      currentQuestionIndex.value += 1
    }

    watch(isFinished, () => {
      if (quiz.value) {
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
