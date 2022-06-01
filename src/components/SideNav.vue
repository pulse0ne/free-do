<template>
  <div id="side-nav" class="col px4 py8 spacey2">
    <router-link to="/" custom v-slot="{ navigate, isActive }">
      <side-nav-link @click="navigate" :isActive="isActive">
        <freedo-icon icon="check-filled-circle" /> Home
      </side-nav-link>
    </router-link>
    <router-link to="/today" custom v-slot="{ navigate, isActive }">
      <side-nav-link @click="navigate" :isActive="isActive">
        <freedo-icon icon="check-filled-circle" /> Today
      </side-nav-link>
    </router-link>
    <router-link to="/upcoming" custom v-slot="{ navigate, isActive }">
      <side-nav-link @click="navigate" :isActive="isActive">
        <freedo-icon icon="check-filled-circle" /> Upcoming
      </side-nav-link>
    </router-link>
    <router-link to="/group/test-group" custom v-slot="{ navigate, isActive }">
      <side-nav-link @click="navigate" :isActive="isActive">
        Test Group
      </side-nav-link>
    </router-link>
    <router-link v-for="group in todoGroups" :key="group.id" :to="'/group/' + group.id" custom v-slot="{ navigate, isActive }">
      <side-nav-link @click="navigate" :isActive="isActive">
        {{ group.name }}
      </side-nav-link>
    </router-link>
  </div>
</template>

<script>
import { mapGetters } from 'vuex';
import FreedoIcon from './FreedoIcon.vue';
import SideNavLink from './SideNavLink.vue';

export default {
  components: {
    FreedoIcon,
    SideNavLink
  },
  computed: {
    ...mapGetters(['todoGroups'])
  }
};
</script>

<style lang="scss" scoped>
#side-nav {
  min-width: 100px;
  background-color: #f9f9f9;
}
</style>
