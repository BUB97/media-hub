import { createRouter, createWebHistory } from 'vue-router'
import { authUtils } from '../api/auth'

// 路由组件懒加载
const Home = () => import('../views/Home.vue')
const Login = () => import('../views/Login.vue')
const Register = () => import('../views/Register.vue')
const Dashboard = () => import('../views/Dashboard.vue')
const MediaList = () => import('../views/MediaList.vue')
const MediaDetail = () => import('../views/MediaDetail.vue')
const Profile = () => import('../views/Profile.vue')

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home,
  },
  {
    path: '/login',
    name: 'Login',
    component: Login,
    meta: { requiresGuest: true }, // 只允许未登录用户访问
  },
  {
    path: '/register',
    name: 'Register',
    component: Register,
    meta: { requiresGuest: true },
  },
  {
    path: '/dashboard',
    name: 'Dashboard',
    component: Dashboard,
    meta: { requiresAuth: true }, // 需要登录
  },
  {
    path: '/media',
    name: 'MediaList',
    component: MediaList,
    meta: { requiresAuth: true },
  },
  {
    path: '/media/:id',
    name: 'MediaDetail',
    component: MediaDetail,
    meta: { requiresAuth: true },
    props: true,
  },
  {
    path: '/profile',
    name: 'Profile',
    component: Profile,
    meta: { requiresAuth: true },
  },
  {
    path: '/:pathMatch(.*)*',
    name: 'NotFound',
    redirect: '/',
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

// 路由守卫
router.beforeEach((to, _from, next) => {
  const isAuthenticated = authUtils.isAuthenticatedSync()
  
  // 需要登录的页面
  if (to.meta.requiresAuth && !isAuthenticated) {
    next({ name: 'Login' })
    return
  }
  
  // 只允许未登录用户访问的页面（如登录、注册）
  if (to.meta.requiresGuest && isAuthenticated) {
    next({ name: 'Dashboard' })
    return
  }
  
  next()
})

export default router