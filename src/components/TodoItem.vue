<template>
  <div class="todo-item" :title="description">
    <div class="main">
      <tydo-icon class="check-icon" :icon="iconName" :size="24" @click="handleCheck"/>
      <span v-if="!isEditing" class="todo-title">{{ item.title }}</span>
      <input v-else v-model="editableTitle">
      <div class="weight-wrapper">
        <tydo-icon class="move-up" @click="addWeight(todo)" />
        <tydo-icon class="move-down" @click="removeWeight(todo)" />
      </div>
      <tydo-icon v-if="!item.completed_at" :class="editIconClass" icon="edit" @click="handleEdit" />
    </div>
    <textarea v-if="isEditing" v-model="editableDescription"></textarea>
  </div>
</template>

<script>
import TydoIcon from './TydoIcon.vue';

export default {
  name: 'TodoItem',
  props: ['item'],
  emits: ['completed', 'uncompleted', 'modified'],
  components: { 'tydo-icon': TydoIcon },
  data () {
    return {
      isEditing: false,
      editableTitle: null,
      editableDescription: null
    };
  },
  mounted () {
    this.editableTitle = this.item.title;
    this.editableDescription = this.item.description;
  },
  methods: {
    handleCheck () {
      if (this.item.completed_at) this.$emit('uncompleted');
      else this.$emit('completed');
    },
    handleEdit () {
      if (this.isEditing) {
        this.$emit('modified', { title: this.editableTitle, description: this.editableDescription });
        this.isEditing = false;
      } else {
        this.isEditing = true;
      }
    }
  },
  computed: {
    iconName () {
      if (this.item.completed_at) {
        return 'check-filled-circle';
      } else {
        return 'check-empty-circle';
      }
    },
    editIconClass () {
      const classes = ['edit-icon'];
      if (this.isEditing) classes.push('editing');
      return classes;
    },
    description () {
      // return this.item.description;
      return JSON.stringify(this.item);
    }
  }
};
</script>

<style scoped>
.todo-item {
  display: flex;
  flex-direction: column;
}

.todo-item .main {
  display: flex;
  flex-direction: row;
  align-items: center;
}

.todo-title {
  cursor: default;
}

.check-icon {
  font-size: 22px;
  color: rgba(0, 0, 0, 0.8);
}

.check-icon:hover {
  color: rgba(0, 0, 0, 1);
}

.edit-icon {
  font-size: 16px;
}

.edit-icon.editing {
  color: dodgerblue;
}
</style>
