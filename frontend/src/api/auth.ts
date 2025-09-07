import apiClient from './client'

// 用户接口类型定义
export interface User {
  id: string
  username: string
  email: string
  created_at: string
  last_login: string
}

export interface LoginRequest {
  username: string
  password: string
}

export interface RegisterRequest {
  username: string
  email: string
  password: string
}

export interface AuthResponse {
  user: User
  expires_at: string
}

// 认证 API 函数
export const authAPI = {
  // 用户注册
  register: async (data: RegisterRequest): Promise<AuthResponse> => {
    const response = await apiClient.post('/auth/register', data)
    return response.data
  },

  // 用户登录
  login: async (data: LoginRequest): Promise<AuthResponse> => {
    const response = await apiClient.post('/auth/login', data)
    return response.data
  },

  // 获取当前用户信息
  getCurrentUser: async (): Promise<User> => {
    const response = await apiClient.get('/auth/me')
    return response.data
  },

  // 用户登出
  logout: async (): Promise<{ message: string }> => {
    const response = await apiClient.post('/auth/logout')
    return response.data
  },
}

// 认证工具函数 (使用 HttpOnly Cookie)
export const authUtils = {
  // 保存认证信息到本地存储 (仅保存用户信息，token 由 Cookie 管理)
  saveAuthData: (authResponse: AuthResponse) => {
    localStorage.setItem('user_info', JSON.stringify(authResponse.user))
    localStorage.setItem('token_expires_at', authResponse.expires_at)
  },

  // 清除认证信息
  clearAuthData: () => {
    localStorage.removeItem('user_info')
    localStorage.removeItem('token_expires_at')
    // Cookie 会由服务器端清除
  },

  // 获取本地存储的用户信息
  getStoredUser: (): User | null => {
    const userInfo = localStorage.getItem('user_info')
    return userInfo ? JSON.parse(userInfo) : null
  },

  // 检查是否已登录 (通过尝试获取用户信息)
  isAuthenticated: async (): Promise<boolean> => {
    try {
      // 尝试调用需要认证的接口来验证 Cookie
      await authAPI.getCurrentUser()
      return true
    } catch (error) {
      // 如果请求失败，清除本地存储的用户信息
      authUtils.clearAuthData()
      return false
    }
  },

  // 同步版本的认证检查 (基于本地存储的过期时间)
  isAuthenticatedSync: (): boolean => {
    const expiresAt = localStorage.getItem('token_expires_at')
    const userInfo = localStorage.getItem('user_info')
    
    if (!expiresAt || !userInfo) {
      return false
    }
    
    // 检查 token 是否过期
    const now = new Date().getTime()
    const expiry = new Date(expiresAt).getTime()
    
    if (now >= expiry) {
      authUtils.clearAuthData()
      return false
    }
    
    return true
  },
}