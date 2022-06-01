import { createRouter, createWebHashHistory } from 'vue-router';
import HomeView from './HomeView.vue';
import TodayView from './TodayView.vue';
import UpcomingView from './UpcomingView.vue';
import GroupView from './GroupView.vue';

const history = createWebHashHistory();

const routes = [
  { path: '/', component: HomeView },
  { path: '/today', component: TodayView },
  { path: '/upcoming', component: UpcomingView },
  { path: '/group/:id', component: GroupView }
];

export default createRouter({ routes, history });
