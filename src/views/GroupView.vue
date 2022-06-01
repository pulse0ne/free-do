<template>
  <div>
    <h1>Group {{ groupId }}</h1>
  </div>
</template>

<script>
export default {
  data () {
    return {
      groupId: null
    };
  },
  created() {
    console.log('here');
    this.groupId = this.$route.params.id;
    this.$watch(
      () => this.$route.params,
      (params) => {
        this.groupId = params.id;
      }
    )
  }
};
</script>
