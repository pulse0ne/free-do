<template>
  <button class="free-do-button" :class="classList">
    <slot></slot>
  </button>
</template>

<script>
export default {
  props: {
    size: {
      default: 'medium'
    },
    type: {
      default: 'primary'
    }
  },
  computed: {
    classList () {
      return [...this.size.split(' '), this.type];
    }
  }
};
</script>

<style lang="scss" scoped>
.free-do-button {
  border: none;
  border-radius: 0.5rem;
  padding: 0.5em 1em;

  &.primary {
    background-color: #585858;
    color: white;
  }

  &.secondary {
    background-color: #e0e0e0;
    color: black;
  }
}
</style>
