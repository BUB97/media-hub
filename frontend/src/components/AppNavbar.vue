<template>
  <nav class="bg-white/80 backdrop-blur-sm shadow-lg border-b border-white/20 sticky top-0 z-50">
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
        <div class="flex items-center space-x-4">
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
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import { authAPI, authUtils } from '../api';
import { computed } from 'vue';

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

const router = useRouter();

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
    router.push('/login');
};
</script>