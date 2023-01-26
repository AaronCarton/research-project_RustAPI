<template>
  <div class="bg-gray-200 p-4">
    <div class="mb-4">
      <p class="mb-2">{{ question.question }}</p>
      <div class="grid grid-cols-2 grid-rows-2 gap-4">
        <div
          v-for="(answer, index) in question.answers"
          :key="index"
          @click="selectAnswer(index)"
          :class="{
            'border-2 border-gray-800': selectedAnswer === index,
            'bg-red-500': !answer.is_correct && showResults,
            'bg-green-500': answer.is_correct && showResults,
          }"
          class="mb-2 cursor-pointer rounded p-2 border-2 border-gray-300 shadow-md"
        >
          {{ answer.answer }}
        </div>
      </div>
    </div>

    <template v-if="!showResults">
      <button class="bg-red-500 text-white p-2 rounded" @click="answerQuestion">
        Show Results
      </button>
    </template>
    <template v-else>
      <button class="bg-red-500 text-white p-2 rounded" @click="nextQuestion()">Next</button>
    </template>
  </div>
</template>

<script lang="ts">
import { useAxios } from '@vueuse/integrations/useAxios'
import { defineComponent, ref, watch } from 'vue'
import useAuthentication from '../composables/useAuthentication'
import Question from '../interfaces/question.interface'

export default {
  props: {
    quizId: {
      type: Number,
      required: true,
    },
    question: {
      type: Object as () => Question,
      required: true,
    },
    nextQuestion: {
      type: Function,
      required: true,
    },
  },
  async setup({ question, quizId }) {
    const { user } = useAuthentication()
    const selectedAnswer = ref(null)
    const showResults = ref(false)

    function selectAnswer(index) {
      selectedAnswer.value = index
    }

    async function answerQuestion() {
      // send answer to http://0.0.0.0:8000/api/quiz/1/answer/5 with useAxios
      const { data, isFinished } = useAxios(
        `${window['env']['API_URL']}/quiz/${quizId}/answer/${question.id}`,
        {
          method: 'POST',
          headers: {
            Authorization: `Bearer ${await user.value?.getIdToken()}`,
          },
          data: {
            answer: question.answers[selectedAnswer.value].answer,
          },
        },
      )
      showResults.value = true
    }

    // function showResults() {
    //   // question.answers = question.answers.map((answer, i) => ({
    //   //   ...answer,
    //   //   isSelected: i === selectedAnswer.value,
    //   // }))
    //   // answered.value = true

    // }

    watch(question, (value) => {
      console.log('selectedAnswer', value)
    })

    return {
      question,
      selectedAnswer,
      showResults,

      selectAnswer,
      answerQuestion,
    }
  },
}
</script>
