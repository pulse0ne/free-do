<template>
  <div class="full-height col">
    <div id="top-nav"></div>
    <div class="row flex-grow">
      <div id="side-nav" class="col p8">
        <router-link to="/" custom v-slot="{ navigate }">
          <side-nav-link @click="navigate">
            Home
          </side-nav-link>
        </router-link>
        <router-link to="/today" custom v-slot="{ navigate }">
          <side-nav-link @click="navigate">
            Today
          </side-nav-link>
        </router-link>
      </div>
      <div id="content" class="px16 py8">
        <router-view />
      </div>
    </div>
  </div>
</template>

<script>
import SideNavLink from './components/SideNavLink.vue';

export default {
  name: 'App',
  components: {
    SideNavLink
  }
};
</script>

<style lang="scss">
@import './styles/styles.scss';

#app {
  height: 100%;
}

#side-nav {
  min-width: 100px;
  background-color: #f9f9f9;
}
</style>
