import { createRouter, createWebHashHistory } from 'vue-router';
import HomeView from './HomeView.vue';
import TodayView from './TodayView.vue';

const history = createWebHashHistory();

const routes = [
  { path: '/', component: HomeView },
  { path: '/today', component: TodayView }
];

export default createRouter({ routes, history });
