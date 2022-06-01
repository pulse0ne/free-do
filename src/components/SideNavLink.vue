<template>
  <div class="side-nav-link px8 py4" :class="activeClass">
    <slot></slot>
  </div>
</template>

<script>
export default {
  props: ['isActive'],
  computed: {
    activeClass () {
      return { 'router-link-active': this.isActive };
    }
  }
};
</script>

<style lang="scss" scoped>
.side-nav-link {
  cursor: default;
  color: unset;
  text-decoration: none;
  border-radius: 0.25rem;

  &.router-link-active,
  &:hover {
    background-color: #eaeaea;
  }
}
</style>
