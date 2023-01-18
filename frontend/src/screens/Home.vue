<template>
  <div>Home</div>
</template>

<script lang="ts">
import useAuthentication from '../composables/useAuthentication'
import { User, Languages } from 'lucide-vue-next'
import { useToast } from 'vue-toastification'
// add
export default {
  components: {
    User,
    Languages,
  },
  setup() {
    const toast = useToast()
    const { user } = useAuthentication()

    return {
      user,
    }
  },
}
</script>
