<template>
  <i class="tydo-icon" :class="className"></i>
</template>

<script>
export default {
  name: 'Icon',
  props: ['icon'],
  computed: {
    className () {
      return `icon-${this.icon}`;
    }
  }
};
</script>

<style>
.tydo-icon {
  cursor: default;
}
</style>
