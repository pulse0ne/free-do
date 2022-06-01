<template>
  <div class="todo-creator">
    <div class="well col p4">
      <input class="title-input m0 mb4" placeholder="Task name">
      <textarea class="description-input m0" placeholder="Description" rows="2"></textarea>
      <div class="metadata-container row justify-space-between">
        <div>destination</div>
        <div>meta</div>
      </div>
    </div>
    <div class="button-container row">
      <freedo-button type="primary">Add task</freedo-button>
      <freedo-button type="secondary">Cancel</freedo-button>
    </div>
  </div>
</template>

<script>
import FreedoButton from './FreedoButton.vue';

export default {
  components: { FreedoButton }
};
</script>

<style lang="scss" scoped>
.todo-creator {
  .well {
    border-radius: 0.25rem;
    border: 1px solid rgba(0, 0, 0, 0.1);

    .title-input {
      border: none;
      outline: none;
      font-size: 1rem;
      font-weight: 500;
      width: 100%;
    }

    .description-input {
      border: none;
      outline: none;
      font-size: 0.9rem;
      font-weight: 300;
      resize: none;
    }
  }
}
</style>
