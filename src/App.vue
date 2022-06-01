<template>
  <div class="full-height col">
    <div id="top-nav"></div>
    <div class="row flex-grow">
      <side-nav></side-nav>
      <div id="content" class="flex-grow px16 py8">
        <router-view />
      </div>
    </div>
  </div>
</template>

<script>
import { mapActions, mapMutations } from 'vuex';
import SideNav from './components/SideNav.vue';

let timerId;

export default {
  name: 'App',
  components: { SideNav },
  mounted() {
    this.getTodoData();
    this.updateTime();
    timerId = setInterval(this.updateTime, 5 * 60 * 1000);
  },
  unmounted() {
    clearInterval(timerId);
  },
  methods: {
    ...mapMutations(['updateTime']),
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
