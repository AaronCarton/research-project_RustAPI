import {
  createRouter,
  createWebHistory,
  RouteLocationNormalized,
  Router,
  RouteRecordRaw,
} from 'vue-router'
import useAuthentication from '../composables/useAuthentication'
import useUser from '../composables/useUser'

const { user } = useAuthentication()
const { Role, user: dbUser } = useUser()

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: () => import('../components/holders/AppHolder.vue'),
    children: [
      {
        path: '', // Eigenlijk zal de / altijd hiernaar resolven
        component: () => import('../screens/Home.vue'),
      },
    ],
  },

  {
    path: '/auth',
    redirect: '/auth/login',
    component: () => import('../components/holders/AuthHolder.vue'),
    children: [
      {
        path: 'login',
        component: () => import('../components/auth/Login.vue'),
        meta: {
          cantAuthenticate: true,
        },
      },

      {
        path: 'register',
        component: () => import('../components/auth/Register.vue'),
        meta: {
          cantAuthenticate: true,
        },
      },

      {
        path: 'forgot-password',
        component: () => import('../components/auth/ForgotPassword.vue'),
      },
    ],
  },

  {
    path: '/:pathMatch(.*)*',
    name: 'ClientError',
    component: () => import('../screens/generic/ClientError.vue'),
  },
]

const router: Router = createRouter({
  history: createWebHistory(),
  routes,
})

router.beforeEach(async (to: RouteLocationNormalized, from: RouteLocationNormalized) => {
  if (to.meta.needsAuthentication && !user.value)
    return { path: '/auth/login', query: { redirect: to.fullPath } }
  if (to.meta.cantAuthenticate && user.value) return '/'
  if (to.meta.needsAdmin && dbUser.value?.role !== Role.ADMIN) return to.fullPath
  if (
    to.meta.needsDriver &&
    dbUser.value?.role !== Role.DRIVER &&
    dbUser.value?.role !== Role.ADMIN
  )
    return to.fullPath
})

export default router
