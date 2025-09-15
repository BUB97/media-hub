<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-50 via-blue-50 to-indigo-100">
    <!-- 现代化导航栏 -->
    <nav class="bg-white/80 backdrop-blur-md shadow-lg border-b border-gray-200/50 sticky top-0 z-50">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between items-center h-16">
          <!-- Logo区域 -->
          <div class="flex items-center space-x-3">
            <div class="w-8 h-8 bg-gradient-to-br from-blue-600 to-purple-600 rounded-lg flex items-center justify-center">
              <svg class="w-5 h-5 text-white" fill="currentColor" viewBox="0 0 20 20">
                <path d="M4 3a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V5a2 2 0 00-2-2H4zm12 12H4l4-8 3 6 2-4 3 6z"/>
              </svg>
            </div>
            <h1 class="text-xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
              Media Hub
            </h1>
          </div>

          <!-- 桌面端导航菜单 -->
          <div class="hidden md:flex items-center space-x-1">
            <template v-if="isAuthenticated">
              <router-link
                to="/dashboard"
                class="nav-link flex items-center space-x-2 px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200 hover:bg-blue-50 hover:text-blue-600"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>
                </svg>
                <span>仪表板</span>
              </router-link>
              <router-link
                to="/media"
                class="nav-link flex items-center space-x-2 px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200 hover:bg-blue-50 hover:text-blue-600"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/>
                </svg>
                <span>媒体库</span>
              </router-link>
              <router-link
                to="/upload"
                class="nav-link flex items-center space-x-2 px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200 hover:bg-green-50 hover:text-green-600"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"/>
                </svg>
                <span>上传</span>
              </router-link>
              
              <!-- 用户菜单 -->
              <div class="relative ml-4">
                <button
                  @click="toggleUserMenu"
                  class="flex items-center space-x-2 px-3 py-2 rounded-lg text-sm font-medium bg-gray-100 hover:bg-gray-200 transition-all duration-200"
                >
                  <div class="w-6 h-6 bg-gradient-to-br from-blue-500 to-purple-600 rounded-full flex items-center justify-center text-white text-xs font-bold">
                    {{ user?.username?.charAt(0).toUpperCase() || 'U' }}
                  </div>
                  <span class="text-gray-700">{{ user?.username || '用户' }}</span>
                  <svg class="w-4 h-4 text-gray-500 transition-transform duration-200" :class="{ 'rotate-180': showUserMenu }" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                  </svg>
                </button>
                
                <!-- 用户下拉菜单 -->
                <div v-if="showUserMenu" class="absolute right-0 mt-2 w-48 bg-white rounded-lg shadow-lg border border-gray-200 py-1 z-[60] animate-fade-in">
                  <router-link
                    to="/profile"
                    class="flex items-center space-x-2 px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 transition-colors duration-150"
                    @click="showUserMenu = false"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
                    </svg>
                    <span>个人资料</span>
                  </router-link>
                  <hr class="my-1 border-gray-200">
                  <button
                    @click="handleLogout"
                    class="flex items-center space-x-2 w-full px-4 py-2 text-sm text-red-600 hover:bg-red-50 transition-colors duration-150"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"/>
                    </svg>
                    <span>登出</span>
                  </button>
                </div>
              </div>
            </template>
            
            <!-- 未登录状态 -->
            <template v-else>
              <router-link
                to="/login"
                class="px-4 py-2 text-sm font-medium text-gray-700 hover:text-blue-600 transition-colors duration-200"
              >
                登录
              </router-link>
              <router-link
                to="/register"
                class="btn btn-primary px-6 py-2 text-sm font-medium"
              >
                注册
              </router-link>
            </template>
          </div>

          <!-- 移动端菜单按钮 -->
          <div class="md:hidden">
            <button
              @click="toggleMobileMenu"
              class="p-3 rounded-xl text-gray-600 hover:text-gray-900 hover:bg-gray-100 transition-all duration-200 touch-target mobile-button"
              :class="{ 'bg-gray-100': showMobileMenu }"
            >
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path v-if="!showMobileMenu" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
                <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
              </svg>
            </button>
          </div>
        </div>

        <!-- 移动端菜单 -->
        <div v-if="showMobileMenu" class="md:hidden border-t border-gray-200 mobile-padding animate-slide-in">
          <div class="space-y-1 mobile-margin">
            <template v-if="isAuthenticated">
              <router-link
                to="/dashboard"
                class="flex items-center space-x-3 px-4 py-4 text-base font-medium text-gray-700 hover:text-blue-600 hover:bg-blue-50 rounded-xl transition-all duration-200 touch-target mobile-text-base"
                @click="showMobileMenu = false"
              >
                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>
                </svg>
                <span>仪表板</span>
              </router-link>
              <router-link
                to="/media"
                class="flex items-center space-x-3 px-4 py-4 text-base font-medium text-gray-700 hover:text-blue-600 hover:bg-blue-50 rounded-xl transition-all duration-200 touch-target mobile-text-base"
                @click="showMobileMenu = false"
              >
                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/>
                </svg>
                <span>媒体库</span>
              </router-link>
              <router-link
                to="/upload"
                class="flex items-center space-x-3 px-4 py-4 text-base font-medium text-gray-700 hover:text-green-600 hover:bg-green-50 rounded-xl transition-all duration-200 touch-target mobile-text-base"
                @click="showMobileMenu = false"
              >
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"/>
                </svg>
                <span>上传文件</span>
              </router-link>
              <router-link
                to="/profile"
                class="flex items-center space-x-3 px-4 py-3 text-base font-medium text-gray-700 hover:text-blue-600 hover:bg-blue-50 rounded-lg transition-all duration-200"
                @click="showMobileMenu = false"
              >
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
                </svg>
                <span>个人资料</span>
              </router-link>
              <hr class="my-2 border-gray-200">
              <button
                @click="handleLogout"
                class="flex items-center space-x-3 w-full px-4 py-3 text-base font-medium text-red-600 hover:bg-red-50 rounded-lg transition-all duration-200"
              >
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"/>
                </svg>
                <span>登出</span>
              </button>
            </template>
            <template v-else>
              <router-link
                to="/login"
                class="block px-4 py-3 text-base font-medium text-gray-700 hover:text-blue-600 hover:bg-blue-50 rounded-lg transition-all duration-200"
                @click="showMobileMenu = false"
              >
                登录
              </router-link>
              <router-link
                to="/register"
                class="block px-4 py-3 text-base font-medium text-white bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 rounded-lg transition-all duration-200"
                @click="showMobileMenu = false"
              >
                注册
              </router-link>
            </template>
          </div>
        </div>
      </div>
    </nav>

    <!-- 主要内容 -->
    <main class="max-w-7xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
      <!-- 英雄区域 -->
      <div class="text-center">
        <h1 class="text-4xl font-bold text-gray-900 sm:text-5xl md:text-6xl animate-fade-in">
          欢迎来到 <span class="text-blue-600">Media Hub</span>
        </h1>
        <p class="mt-3 max-w-md mx-auto text-base text-gray-500 sm:text-lg md:mt-5 md:text-xl md:max-w-3xl animate-slide-in-left" style="animation-delay: 0.2s; animation-fill-mode: both;">
          一个现代化的媒体管理平台，支持视频、音频和图片的上传、处理和管理。
        </p>
        <div class="mt-5 max-w-md mx-auto sm:flex sm:justify-center md:mt-8 animate-slide-in-right" style="animation-delay: 0.4s; animation-fill-mode: both;">
          <div class="rounded-md shadow">
            <!-- <router-link
              v-if="!isAuthenticated"
              to="/register"
              class="w-full flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 md:py-4 md:text-lg md:px-10"
            >
            </router-link> -->
            <div 
              v-if="!isAuthenticated"
              @click="handleRegisterClick"
              class="w-full flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 md:py-4 md:text-lg md:px-10 hover-lift"
            >
              开始使用
            </div>
            <router-link
              v-else
              to="/dashboard"
              class="w-full flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 md:py-4 md:text-lg md:px-10 hover-lift"
            >
              进入仪表板
            </router-link>
          </div>
        </div>
      </div>

      <!-- 功能特性 -->
      <div class="mt-20">
        <div class="text-center animate-fade-in" style="animation-delay: 0.6s; animation-fill-mode: both;">
          <h2 class="text-3xl font-bold text-gray-900">主要功能</h2>
          <p class="mt-4 text-lg text-gray-600">强大的媒体处理和管理功能</p>
        </div>
        <div class="mt-12 grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-3">
          <div class="bg-white rounded-lg shadow-md p-6 animate-slide-in-up hover-lift" style="animation-delay: 0.8s; animation-fill-mode: both;">
            <div class="text-blue-600 mb-4">
              <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"></path>
              </svg>
            </div>
            <h3 class="text-lg font-semibold text-gray-900 mb-2">文件上传</h3>
            <p class="text-gray-600">支持多种格式的媒体文件上传，包括视频、音频和图片。</p>
          </div>
          <div class="bg-white rounded-lg shadow-md p-6 animate-slide-in-up hover-lift" style="animation-delay: 1.0s; animation-fill-mode: both;">
            <div class="text-blue-600 mb-4">
              <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path>
              </svg>
            </div>
            <h3 class="text-lg font-semibold text-gray-900 mb-2">媒体处理</h3>
            <p class="text-gray-600">使用 WebAssembly 技术进行高效的媒体文件处理和转换。</p>
          </div>
          <div class="bg-white rounded-lg shadow-md p-6 animate-slide-in-up hover-lift" style="animation-delay: 1.2s; animation-fill-mode: both;">
            <div class="text-blue-600 mb-4">
              <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path>
              </svg>
            </div>
            <h3 class="text-lg font-semibold text-gray-900 mb-2">数据管理</h3>
            <p class="text-gray-600">完整的媒体库管理功能，支持分类、搜索和批量操作。</p>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, onUnmounted } from 'vue'
import { authUtils, authAPI } from '../api'

// 响应式数据
const showUserMenu = ref(false)
const showMobileMenu = ref(false)
const user = ref<any>(null)

// 检查用户是否已登录
const isAuthenticated = computed(() => authUtils.isAuthenticatedSync())

// 获取用户信息
const loadUserInfo = async () => {
  if (isAuthenticated.value) {
    try {
      const userData = authUtils.getStoredUser()
      user.value = userData
    } catch (error) {
      console.error('获取用户信息失败:', error)
    }
  }
}

// 切换用户菜单
const toggleUserMenu = () => {
  showUserMenu.value = !showUserMenu.value
  if (showUserMenu.value) {
    showMobileMenu.value = false
  }
}

// 切换移动端菜单
const toggleMobileMenu = () => {
  showMobileMenu.value = !showMobileMenu.value
  if (showMobileMenu.value) {
    showUserMenu.value = false
  }
}

// 处理点击外部关闭菜单
const handleClickOutside = (event: Event) => {
  const target = event.target as Element
  if (!target.closest('.relative')) {
    showUserMenu.value = false
  }
  if (!target.closest('.md\\:hidden')) {
    showMobileMenu.value = false
  }
}

// 处理登出
const handleLogout = async () => {
  try {
    await authAPI.logout()
    authUtils.clearAuthData()
    user.value = null
    showUserMenu.value = false
    showMobileMenu.value = false
    getApp().goTo('/')
  } catch (error) {
    console.error('登出失败:', error)
    // 即使 API 调用失败，也清除本地数据
    authUtils.clearAuthData()
    user.value = null
    showUserMenu.value = false
    showMobileMenu.value = false
    getApp().goTo('/')
  }
}

// 处理注册点击
const handleRegisterClick = () => {
  getApp().goTo('/register')
}

// 生命周期钩子
onMounted(() => {
  loadUserInfo()
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>