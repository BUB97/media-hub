// 全局类型声明
declare global {
  interface Window {
    getApp: () => {
      goTo: (to: string | { path: string; query?: any }, isReplace?: boolean) => void
      back: () => void
      forward: () => void
      currentRoute: any
      router: any
    }
  }

  // 全局方法声明
  function getApp(): {
    goTo: (to: string | { path: string; query?: any }, isReplace?: boolean) => void
    back: () => void
    forward: () => void
    currentRoute: any
    router: any
  }
}

export {}