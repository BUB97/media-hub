<template>
  <div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100">
    <!-- 导航栏 -->
    <nav class="bg-white shadow-sm">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16">
          <div class="flex items-center">
            <h1 class="text-xl font-bold text-gray-900">Media Hub</h1>
          </div>
          <div class="flex items-center space-x-4">
            <template v-if="isAuthenticated">
              <router-link
                to="/dashboard"
                class="text-gray-700 hover:text-blue-600 px-3 py-2 rounded-md text-sm font-medium"
              >
                仪表板
              </router-link>
              <router-link
                to="/media"
                class="text-gray-700 hover:text-blue-600 px-3 py-2 rounded-md text-sm font-medium"
              >
                媒体库
              </router-link>
              <router-link
                to="/profile"
                class="text-gray-700 hover:text-blue-600 px-3 py-2 rounded-md text-sm font-medium"
              >
                个人资料
              </router-link>
              <button
                @click="handleLogout"
                class="bg-red-600 text-white px-4 py-2 rounded-md text-sm font-medium hover:bg-red-700"
              >
                登出
              </button>
            </template>
            <template v-else>
              <router-link
                to="/login"
                class="text-gray-700 hover:text-blue-600 px-3 py-2 rounded-md text-sm font-medium"
              >
                登录
              </router-link>
              <router-link
                to="/register"
                class="bg-blue-600 text-white px-4 py-2 rounded-md text-sm font-medium hover:bg-blue-700"
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
        <h1 class="text-4xl font-bold text-gray-900 sm:text-5xl md:text-6xl">
          欢迎来到 <span class="text-blue-600">Media Hub</span>
        </h1>
        <p class="mt-3 max-w-md mx-auto text-base text-gray-500 sm:text-lg md:mt-5 md:text-xl md:max-w-3xl">
          一个现代化的媒体管理平台，支持视频、音频和图片的上传、处理和管理。
        </p>
        <div class="mt-5 max-w-md mx-auto sm:flex sm:justify-center md:mt-8">
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
              class="w-full flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 md:py-4 md:text-lg md:px-10"
            >
              开始使用
            </div>
            <router-link
              v-else
              to="/dashboard"
              class="w-full flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 md:py-4 md:text-lg md:px-10"
            >
              进入仪表板
            </router-link>
          </div>
        </div>
      </div>

      <!-- 功能特性 -->
      <div class="mt-20">
        <div class="text-center">
          <h2 class="text-3xl font-bold text-gray-900">主要功能</h2>
          <p class="mt-4 text-lg text-gray-600">强大的媒体处理和管理功能</p>
        </div>
        <div class="mt-12 grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-3">
          <div class="bg-white rounded-lg shadow-md p-6">
            <div class="text-blue-600 mb-4">
              <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"></path>
              </svg>
            </div>
            <h3 class="text-lg font-semibold text-gray-900 mb-2">文件上传</h3>
            <p class="text-gray-600">支持多种格式的媒体文件上传，包括视频、音频和图片。</p>
          </div>
          <div class="bg-white rounded-lg shadow-md p-6">
            <div class="text-blue-600 mb-4">
              <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path>
              </svg>
            </div>
            <h3 class="text-lg font-semibold text-gray-900 mb-2">媒体处理</h3>
            <p class="text-gray-600">使用 WebAssembly 技术进行高效的媒体文件处理和转换。</p>
          </div>
          <div class="bg-white rounded-lg shadow-md p-6">
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
import { computed } from 'vue'
import { authUtils, authAPI } from '../api'

// 检查用户是否已登录
const isAuthenticated = computed(() => authUtils.isAuthenticatedSync())

// 处理登出
const handleLogout = async () => {
  try {
    await authAPI.logout()
    authUtils.clearAuthData()
    getApp().goTo('/')
  } catch (error) {
    console.error('登出失败:', error)
    // 即使 API 调用失败，也清除本地数据
    authUtils.clearAuthData()
    getApp().goTo('/')
  }
}

// 处理注册点击
const handleRegisterClick = () => {
  getApp().goTo('/register')
}
</script>