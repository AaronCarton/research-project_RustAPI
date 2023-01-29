<template>
  <div>
    <div class="mb-2">
      <p class="mb-6 text-lg font-semibold">{{ question.question }}</p>
      <div class="grid grid-cols-2 grid-rows-2 gap-4">
        <div
          v-for="(answer, index) in question.answers"
          :key="index"
          @click="selectAnswer(index)"
          :class="{
            'bg-neutral-400 !border-neutral-200': selectedAnswer === index,
            '!bg-red-500 !text-neutral-100': !answer.is_correct && showResults,
            '!bg-green-500 !text-neutral-100': answer.is_correct && showResults,
          }"
          class="mb-2 cursor-pointer capitalize text-lg border-5 border-transparent bg-neutral-100 text-center font-bold rounded-xl px-2 py-3 text-neutral-800 shadow-md"
        >
          {{ answer.answer }}
        </div>
      </div>
    </div>

    <div class="flex justify-end mt-8">
      <template v-if="!showResults">
        <button
          class="bg-red-500 font-semibold text-white w-1/4 py-3 rounded"
          @click="answerQuestion"
        >
          Show Results
        </button>
      </template>
      <template v-else>
        <button
          class="bg-red-500 font-semibold text-white py-3 w-1/4 rounded"
          @click="nextQuestion()"
        >
          Next
        </button>
      </template>
    </div>
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
      if (showResults.value) return
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
