import { createStore } from 'vuex';
import { invoke } from '@tauri-apps/api/tauri';

export default createStore({
  state: {
    todos: []
  },
  getters: {
    sortedTodos (state) {
      return state.todos.filter(t => !t.completed_at).sort((a, b) => {
        if (a.weight == b.weight) return b.created_at - a.created_at;
        if (a.weight > b.weight) return -1;
        return 1;
      });
    },
    sortedCompleted (state) {
      return state.todos.filter(t => t.completed_at).sort((a, b) => {
        return b.completed_at - a.completed_at;
      });
    }
  },
  mutations: {
    setTodos (state, todos) {
      console.log(todos);
      state.todos = todos;
    },
    completeTodo (state, todo) {
      const t = state.todos.find(t => t.id === todo.id);
      t.completed_at = Date.now();
    },
    uncompleteTodo (state, todo) {
      const t = state.todos.find(t => t.id === todo.id);
      t.completed_at = null;
    },
    modifyTodo (state, todo) {
      const ix = state.todos.findIndex(t => t.id === todo.id);
      state.todos[ix] = todo;
    }
  },
  actions: {
    getTodos ({ commit }) {
      invoke('get_todos').then(todos => commit('setTodos', todos));
    },
    saveTodos ({ state }) {
      // invoke('save_todos', state.todos);
      console.log('saving todos...', JSON.stringify(state.todos));
    },
    completeTodo ({ commit, dispatch }, todo) {
      commit('completeTodo', todo);
      dispatch('saveTodos');
    },
    uncompleteTodo ({ commit, dispatch }, todo) {
      commit('uncompleteTodo', todo);
      dispatch('saveTodos');
    },
    modifyTodo ({ commit, dispatch }, todo) {
      commit('modifyTodo', todo);
      dispatch('saveTodos');
    }
  }
});
