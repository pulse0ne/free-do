<template>
  <div class="full-height col">
    <div id="top-nav"></div>
    <div class="row flex-grow">
      <side-nav></side-nav>
      <div id="content" class="px16 py8">
        <router-view />
      </div>
    </div>
  </div>
</template>

<script>
import { mapActions } from 'vuex';
import SideNav from './components/SideNav.vue';

export default {
  name: 'App',
  components: { SideNav },
  mounted() {
    this.getTodoData();
  },
  methods: {
    ...mapActions(['getTodoData'])
  }
};
</script>

<style lang="scss">
@import './styles/styles.scss';

#app {
  height: 100%;
}
</style>
