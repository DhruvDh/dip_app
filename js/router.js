import Vue from 'vue';
import VueRouter from 'vue-router';
import Home from './views/Home.vue';
import ViewerPage from './views/ViewerPage.vue';
import Ass1 from './views/Ass1.vue';
import Ass2 from './views/Ass2.vue';
import Ass3 from './views/Ass3.vue';

Vue.use(VueRouter);

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home,
  },
  {
    path: '/viewer',
    name: 'Viewer',
    component: ViewerPage,
  },
  {
    path: '/ass_1',
    name: 'Assignment 1',
    component: Ass1,
  },
  {
    path: '/ass_2',
    name: 'Assignment 2',
    component: Ass2,
  },
  {
    path: '/ass_3',
    name: 'Assignment 3',
    component: Ass3,
  },
];

/* eslint-disable */
const router = new VueRouter({
  base: __webpack_public_path__,
  routes,
});
/* eslint-enable */

export default router;
