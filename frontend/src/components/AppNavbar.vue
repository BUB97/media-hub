<template>
  <nav class="bg-white/80 backdrop-blur-sm shadow-lg border-b border-white/20 sticky top-0 z-50 relative">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex justify-between h-16">
        <div class="flex items-center">
          <div class="flex items-center space-x-3">
            <div class="h-8 w-8 bg-gradient-to-r from-blue-500 to-purple-600 rounded-lg flex items-center justify-center">
              <svg class="h-5 w-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path>
              </svg>
            </div>
            <router-link to="/" class="text-xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">Media Hub</router-link>
          </div>
        </div>
        <!-- 桌面端导航链接 -->
        <div class="hidden md:flex items-center space-x-4">
          <router-link
            v-if="showNavLinks"
            to="/dashboard"
            :class="getNavLinkClass('dashboard')"
          >
            仪表板
          </router-link>
          <router-link
            v-if="showNavLinks"
            to="/media"
            :class="getNavLinkClass('media')"
          >
            媒体库
          </router-link>
          <router-link
            v-if="showNavLinks"
            to="/profile"
            :class="getNavLinkClass('profile')"
          >
            个人资料
          </router-link>
          <button
            v-if="showLogout"
            @click="handleLogout"
            class="bg-gradient-to-r from-red-500 to-red-600 hover:from-red-600 hover:to-red-700 text-white px-4 py-2 rounded-xl text-sm font-medium transition-all duration-200 shadow-lg hover:shadow-xl"
          >
            登出
          </button>
          <div v-if="showAuthLinks" class="flex items-center space-x-2">
            <router-link
              to="/login"
              class="text-gray-600 hover:text-blue-600 px-3 py-2 rounded-xl text-sm font-medium transition-colors duration-200 hover:bg-blue-50"
            >
              登录
            </router-link>
            <router-link
              to="/register"
              class="bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700 text-white px-4 py-2 rounded-xl text-sm font-medium transition-all duration-200 shadow-lg hover:shadow-xl"
            >
              注册
            </router-link>
          </div>
        </div>

        <!-- 移动端菜单开关 -->
        <div class="flex md:hidden items-center">
          <button
            @click="isMobileMenuOpen = !isMobileMenuOpen"
            class="inline-flex items-center justify-center rounded-xl p-2 text-gray-600 hover:text-blue-600 hover:bg-blue-50 focus:outline-none focus:ring-2 focus:ring-blue-300"
            :aria-expanded="isMobileMenuOpen"
            aria-controls="mobile-menu"
          >
            <svg v-if="!isMobileMenuOpen" class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
            </svg>
            <svg v-else class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
      </div>

      <!-- 移动端折叠菜单（绝对定位，移出正常文档流） -->
      <div
        id="mobile-menu"
        class="md:hidden absolute inset-x-0 top-16 z-40 overflow-hidden transition-all duration-300 bg-white/95 backdrop-blur-sm border-b border-white/20 shadow-xl"
        :style="isMobileMenuOpen ? 'max-height: 30rem; opacity: 1;' : 'max-height: 0; opacity: 0;'"
        aria-hidden="false"
      >
        <div class="max-w-7xl mx-auto px-4 pb-4 space-y-2">
          <div v-if="showNavLinks" class="grid grid-cols-1 gap-2">
            <router-link
              to="/dashboard"
              :class="getNavLinkClass('dashboard') + ' block'"
              @click="isMobileMenuOpen = false"
            >
              仪表板
            </router-link>
            <router-link
              to="/media"
              :class="getNavLinkClass('media') + ' block'"
              @click="isMobileMenuOpen = false"
            >
              媒体库
            </router-link>
            <router-link
              to="/profile"
              :class="getNavLinkClass('profile') + ' block'"
              @click="isMobileMenuOpen = false"
            >
              个人资料
            </router-link>
          </div>

          <div v-if="showAuthLinks" class="grid grid-cols-2 gap-2">
            <router-link
              to="/login"
              class="text-gray-600 hover:text-blue-600 px-3 py-2 rounded-xl text-sm font-medium transition-colors duration-200 hover:bg-blue-50 text-center"
              @click="isMobileMenuOpen = false"
            >
              登录
            </router-link>
            <router-link
              to="/register"
              class="bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700 text-white px-4 py-2 rounded-xl text-sm font-medium transition-all duration-200 shadow-lg hover:shadow-xl text-center"
              @click="isMobileMenuOpen = false"
            >
              注册
            </router-link>
          </div>

          <div v-if="showLogout" class="pt-2">
            <button
              @click="handleLogout"
              class="w-full bg-gradient-to-r from-red-500 to-red-600 hover:from-red-600 hover:to-red-700 text-white px-4 py-2 rounded-xl text-sm font-medium transition-all duration-200 shadow-lg hover:shadow-xl"
            >
              登出
            </button>
          </div>
        </div>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { authAPI, authUtils } from '../api';

interface Props {
  showNavLinks?: boolean
  showLogout?: boolean
  showAuthLinks?: boolean
  currentPage?: string
}

const props = withDefaults(defineProps<Props>(), {
    showNavLinks: true,
    showLogout: true,
    showAuthLinks: false,
    currentPage: ''
});

const isMobileMenuOpen = ref(false);

// 计算导航链接的样式类
const getNavLinkClass = (page: string) => {
  const baseClass = "px-3 py-2 rounded-xl text-sm font-medium transition-colors duration-200";
  const activeClass = "text-blue-600 bg-blue-50 border border-blue-200";
  const inactiveClass = "text-gray-600 hover:text-blue-600 hover:bg-blue-50";
  
  return props.currentPage === page 
    ? `${baseClass} ${activeClass}` 
    : `${baseClass} ${inactiveClass}`;
};

const handleLogout = async () => {
    try {
        await authAPI.logout();
        authUtils.clearAuthData();
    } catch (error) {
        console.error('登出失败:', error);
        authUtils.clearAuthData();
    }
    getApp().goTo({ path: '/login' });
    isMobileMenuOpen.value = false;
};
</script>