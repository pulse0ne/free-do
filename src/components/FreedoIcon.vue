<template>
  <i class="freedo-icon" :class="className"></i>
</template>

<script>
export default {
  name: 'FreedoIcon',
  props: ['icon'],
  computed: {
    className () {
      return `icon-${this.icon}`;
    }
  }
};
</script>

<style>
.freedo-icon {
  cursor: default;
}
</style>
