<template>
  <div class="side-nav-link px8 py4">
    <slot></slot>
  </div>
</template>

<style lang="scss" scoped>
.side-nav-link {
  color: unset;
  text-decoration: none;

  .router-link-active,
  &:hover {
    background-color: #efefef;
  }
}
</style>
