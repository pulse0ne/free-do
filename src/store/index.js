import { createStore } from 'vuex';
import { invoke } from '@tauri-apps/api/tauri';
import moment from 'moment';

const getData = () => invoke('get_data');
// const saveData = () => invoke('save_data');

export default createStore({
  state: {
    todoData: {},
    now: moment()
  },
  getters: {
    // sortedTodos (state) {
    //   return state.todos.filter(t => !t.completed_at).sort((a, b) => {
    //     if (a.weight == b.weight) return b.created_at - a.created_at;
    //     if (a.weight > b.weight) return -1;
    //     return 1;
    //   });
    // },
    // sortedCompleted (state) {
    //   return state.todos.filter(t => t.completed_at).sort((a, b) => {
    //     return b.completed_at - a.completed_at;
    //   });
    // },
    todoGroups (state) {
      return state.todoData.groups;
    },
    allTodos (state) {
      return state.todoData.items;
    },
    todayTodos (state) {
      const now = state.now.clone();
      const [ startOfDay, endOfDay ] = [ now.startOf('day'), now.endOf('day') ];
      return state.todoData.items.filter(i => moment(i).isBetween(startOfDay, endOfDay));
    },
    upcomingTodos (state) {
      const today = state.now.clone().startOf('day');
      const twoWeeksFromNow = today.clone().add(14, 'days');
      return state.todoData.items.filter(i => moment(i).isBetween(today, twoWeeksFromNow));
    },
    overdueTodos (state) {
      const today = state.now.clone().startOf('day');
      return state.todoData.items.filter(i => moment(i).isBefore(today) && !i.completed_at);
    }
  },
  mutations: {
    // completeTodo (state, todo) {
    //   const t = state.todos.find(t => t.id === todo.id);
    //   t.completed_at = Date.now();
    // },
    // uncompleteTodo (state, todo) {
    //   const t = state.todos.find(t => t.id === todo.id);
    //   t.completed_at = null;
    // },
    // modifyTodo (state, todo) {
    //   const ix = state.todos.findIndex(t => t.id === todo.id);
    //   state.todos[ix] = todo;
    // }
    updateTime (state) {
      console.log('updating time');
      state.now = moment();
    },
    setTodoData (state, todoData) {
      console.log(todoData);
      state.todoData = todoData;
    }
  },
  actions: {
    // getTodos ({ commit }) {
    //   invoke('get_todos').then(todos => commit('setTodos', todos));
    // },
    // saveTodos ({ state }) {
    //   // invoke('save_todos', state.todos);
    //   console.log('saving todos...', JSON.stringify(state.todos));
    // },
    // completeTodo ({ commit, dispatch }, todo) {
    //   commit('completeTodo', todo);
    //   dispatch('saveTodos');
    // },
    // uncompleteTodo ({ commit, dispatch }, todo) {
    //   commit('uncompleteTodo', todo);
    //   dispatch('saveTodos');
    // },
    // modifyTodo ({ commit, dispatch }, todo) {
    //   commit('modifyTodo', todo);
    //   dispatch('saveTodos');
    // }
    getTodoData ({ commit }) {
      getData().then(data => commit('setTodoData', data));
    }
  }
});
