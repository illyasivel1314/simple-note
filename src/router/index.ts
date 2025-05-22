import SplashScreen from "/src/views/SplashScreen.vue";
import Calendar from "/src/views/Calendar.vue";
import { createRouter, createWebHistory } from 'vue-router';

const routes = [
  {
    path: "/splashscreen",
    name: "SplashScreen",
    component: SplashScreen
  },
  {
    path: "/calendar",
    name: "Calendar",
    component: Calendar
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;