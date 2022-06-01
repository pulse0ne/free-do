<template>
  <div>
    <h1>Home</h1>
    <div v-for="t in overdueTodos" :key="t.id">
      {{ t.title }}
    </div>
    <todo-creator />
  </div>
</template>

<script>
import { mapGetters } from 'vuex';
import TodoCreator from '../components/TodoCreator.vue';

export default {
  components: { TodoCreator },
  computed: {
    ...mapGetters(['overdueTodos'])
  }
};
</script>
