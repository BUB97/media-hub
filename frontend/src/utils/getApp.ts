import type { Router } from 'vue-router'

// 全局路由实例
let globalRouter: Router | null = null

// 全局应用实例（带有路由方法）
const appInstance = {
  // 路由跳转方法
  goTo(to: string | { path: string; query?: any }, isReplace = false) {
    if (!globalRouter) {
      console.error('Router not initialized')
      return
    }
    if (typeof to === 'string') {
      to = { path: to }
    }
    if (isReplace) {
      globalRouter.replace(to)
    } else {
      globalRouter.push(to)
    }
  },

  // 路由回退
  back() {
    if (!globalRouter) {
      console.error('Router not initialized')
      return
    }
    globalRouter.back()
  },

  // 路由前进
  forward() {
    if (!globalRouter) {
      console.error('Router not initialized')
      return
    }
    globalRouter.forward()
  },

  // 获取当前路由
  get currentRoute() {
    return globalRouter?.currentRoute
  },

  // 获取路由实例
  get router() {
    return globalRouter
  }
}

// 全局方法：获取应用实例
const getApp = () => appInstance

// 初始化方法，在 main.ts 中调用
export const initApp = (router: Router) => {
  globalRouter = router
  // 挂载到全局 window 对象
  if (typeof window !== 'undefined') {
    window.getApp = getApp
  }
}