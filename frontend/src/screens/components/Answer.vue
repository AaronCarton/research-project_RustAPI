<template>
  <div>
    <label :class="`block ${getColor}`">
      <input type="radio" :value="answer" name="quiz" />
      {{ answer }}
    </label>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, watch } from 'vue'

export default {
  props: {
    answer: {
      type: String,
      required: true,
    },
    correct: {
      type: Boolean,
      required: true,
    },
    selectedAnswer: {
      type: Boolean,
      required: true,
    },
    checkAnswer: {
      type: Boolean,
      required: true,
    },
  },
  setup({ selectedAnswer, checkAnswer, correct }) {
    watch(
      () => selectedAnswer,
      (value) => {
        console.log('selectedAnswer', value)
      },
    )
    const getColor = () => {
      console.log('selectedAnswer', selectedAnswer)
      console.log('checkAnswer', checkAnswer)
      console.log('correct', correct)

      if (checkAnswer) {
        if (correct) {
          return 'text-green-500'
        } else {
          return 'text-red-500'
        }
      }
    }
    return {
      getColor,
    }
  },
}
</script>
