<template>
  <div class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100">
    <!-- 统一导航栏 -->
    <AppNavbar current-page="dashboard" />

    <!-- 主要内容 -->
    <main class="max-w-7xl mx-auto py-8 sm:px-6 lg:px-8">
      <!-- 欢迎区域 -->
      <div class="px-4 py-6 sm:px-0">
        <div class="bg-gradient-to-r from-blue-500 to-purple-600 rounded-2xl shadow-xl overflow-hidden">
          <div class="px-8 py-12 text-center text-white relative">
            <div class="absolute inset-0 bg-black/10"></div>
            <div class="relative z-10">
              <h1 class="text-3xl font-bold mb-4">
                欢迎回来，{{ user?.username }}！
              </h1>
              <p class="text-xl opacity-90 max-w-2xl mx-auto">
                这是您的媒体管理仪表板，您可以在这里管理您的所有媒体文件。开始您的创作之旅吧！
              </p>
              <div class="mt-8 flex justify-center space-x-4">
                <router-link
                  to="/upload"
                  class="bg-white/20 backdrop-blur-sm hover:bg-white/30 text-white px-6 py-3 rounded-xl font-medium transition-all duration-200 border border-white/30"
                >
                  立即上传
                </router-link>
                <router-link
                  to="/media"
                  class="bg-white text-blue-600 hover:bg-gray-50 px-6 py-3 rounded-xl font-medium transition-all duration-200"
                >
                  浏览媒体库
                </router-link>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 统计卡片 -->
      <div class="px-4 py-6 sm:px-0">
        <h3 class="text-2xl font-bold text-gray-900 mb-6">数据概览</h3>
        <div class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4">
          <div class="bg-white/80 backdrop-blur-sm overflow-hidden shadow-lg rounded-2xl border border-white/20 hover:shadow-xl transition-all duration-300 transform hover:scale-105 animate-fade-in-up" style="animation-delay: 0.1s;">
            <div class="p-6">
              <div class="flex items-center">
                <div class="flex-shrink-0">
                  <div class="h-12 w-12 bg-gradient-to-r from-blue-500 to-blue-600 rounded-xl flex items-center justify-center">
                    <svg class="h-6 w-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z"></path>
                    </svg>
                  </div>
                </div>
                <div class="ml-5 w-0 flex-1">
                  <dl>
                    <dt class="text-sm font-medium text-gray-500 truncate">总媒体数</dt>
                    <dd class="text-2xl font-bold text-gray-900">{{ stats.totalMedia }}</dd>
                    <dd class="text-xs text-gray-400 mt-1">所有类型文件</dd>
                  </dl>
                </div>
              </div>
            </div>
          </div>

          <div class="bg-white/80 backdrop-blur-sm overflow-hidden shadow-lg rounded-2xl border border-white/20 hover:shadow-xl transition-all duration-300 transform hover:scale-105 animate-fade-in-up" style="animation-delay: 0.2s;">
            <div class="p-6">
              <div class="flex items-center">
                <div class="flex-shrink-0">
                  <div class="h-12 w-12 bg-gradient-to-r from-purple-500 to-purple-600 rounded-xl flex items-center justify-center">
                    <svg class="h-6 w-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                    </svg>
                  </div>
                </div>
                <div class="ml-5 w-0 flex-1">
                  <dl>
                    <dt class="text-sm font-medium text-gray-500 truncate">视频文件</dt>
                    <dd class="text-2xl font-bold text-gray-900">{{ stats.videoCount }}</dd>
                    <dd class="text-xs text-gray-400 mt-1">MP4, AVI, MOV</dd>
                  </dl>
                </div>
              </div>
            </div>
          </div>

          <div class="bg-white/80 backdrop-blur-sm overflow-hidden shadow-lg rounded-2xl border border-white/20 hover:shadow-xl transition-all duration-300 transform hover:scale-105 animate-fade-in-up" style="animation-delay: 0.3s;">
            <div class="p-6">
              <div class="flex items-center">
                <div class="flex-shrink-0">
                  <div class="h-12 w-12 bg-gradient-to-r from-green-500 to-green-600 rounded-xl flex items-center justify-center">
                    <svg class="h-6 w-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3"></path>
                    </svg>
                  </div>
                </div>
                <div class="ml-5 w-0 flex-1">
                  <dl>
                    <dt class="text-sm font-medium text-gray-500 truncate">音频文件</dt>
                    <dd class="text-2xl font-bold text-gray-900">{{ stats.audioCount }}</dd>
                    <dd class="text-xs text-gray-400 mt-1">MP3, WAV, FLAC</dd>
                  </dl>
                </div>
              </div>
            </div>
          </div>

          <div class="bg-white/80 backdrop-blur-sm overflow-hidden shadow-lg rounded-2xl border border-white/20 hover:shadow-xl transition-all duration-300 transform hover:scale-105 animate-fade-in-up" style="animation-delay: 0.4s;">
            <div class="p-6">
              <div class="flex items-center">
                <div class="flex-shrink-0">
                  <div class="h-12 w-12 bg-gradient-to-r from-pink-500 to-pink-600 rounded-xl flex items-center justify-center">
                    <svg class="h-6 w-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
                    </svg>
                  </div>
                </div>
                <div class="ml-5 w-0 flex-1">
                  <dl>
                    <dt class="text-sm font-medium text-gray-500 truncate">图片文件</dt>
                    <dd class="text-2xl font-bold text-gray-900">{{ stats.imageCount }}</dd>
                    <dd class="text-xs text-gray-400 mt-1">JPG, PNG, GIF</dd>
                  </dl>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 快速操作 -->
      <div class="px-4 py-6 sm:px-0">
        <div class="bg-white/80 backdrop-blur-sm shadow-lg rounded-2xl border border-white/20">
          <div class="px-6 py-8">
            <h3 class="text-2xl font-bold text-gray-900 mb-6">快速操作</h3>
            <div class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
              <router-link
                to="/media"
                class="relative group bg-gradient-to-br from-blue-50 to-blue-100 p-8 rounded-2xl border-2 border-blue-200 hover:border-blue-300 transition-all duration-300 transform hover:scale-105 hover:shadow-xl"
              >
                <div>
                  <span class="rounded-2xl inline-flex p-4 bg-gradient-to-r from-blue-500 to-blue-600 text-white shadow-lg">
                    <svg class="h-8 w-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path>
                    </svg>
                  </span>
                </div>
                <div class="mt-6">
                  <h3 class="text-xl font-bold text-gray-900">
                    <span class="absolute inset-0" aria-hidden="true"></span>
                    浏览媒体库
                  </h3>
                  <p class="mt-3 text-gray-600">
                    查看和管理您的所有媒体文件
                  </p>
                </div>
              </router-link>

              <button
                @click="showUploadModal = true"
                class="relative group bg-gradient-to-br from-green-50 to-green-100 p-8 rounded-2xl border-2 border-green-200 hover:border-green-300 transition-all duration-300 transform hover:scale-105 hover:shadow-xl"
              >
                <div>
                  <span class="rounded-2xl inline-flex p-4 bg-gradient-to-r from-green-500 to-green-600 text-white shadow-lg">
                    <svg class="h-8 w-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"></path>
                    </svg>
                  </span>
                </div>
                <div class="mt-6">
                  <h3 class="text-xl font-bold text-gray-900">
                    上传新文件
                  </h3>
                  <p class="mt-3 text-gray-600">
                    添加新的媒体文件到您的库中
                  </p>
                </div>
              </button>

              <router-link
                to="/profile"
                class="relative group bg-gradient-to-br from-purple-50 to-purple-100 p-8 rounded-2xl border-2 border-purple-200 hover:border-purple-300 transition-all duration-300 transform hover:scale-105 hover:shadow-xl"
              >
                <div>
                  <span class="rounded-2xl inline-flex p-4 bg-gradient-to-r from-purple-500 to-purple-600 text-white shadow-lg">
                    <svg class="h-8 w-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
                    </svg>
                  </span>
                </div>
                <div class="mt-6">
                  <h3 class="text-xl font-bold text-gray-900">
                    <span class="absolute inset-0" aria-hidden="true"></span>
                    个人设置
                  </h3>
                  <p class="mt-3 text-gray-600">
                    管理您的账户和偏好设置
                  </p>
                </div>
              </router-link>
            </div>
          </div>
        </div>
      </div>

      <!-- 最近活动 -->
      <div class="px-4 py-6 sm:px-0">
        <div class="bg-white/80 backdrop-blur-sm shadow-lg rounded-2xl border border-white/20">
          <div class="px-6 py-8">
            <div class="flex items-center justify-between mb-6">
              <h3 class="text-2xl font-bold text-gray-900">最近活动</h3>
              <button class="text-blue-600 hover:text-blue-700 text-sm font-medium transition-colors duration-200">
                查看全部
              </button>
            </div>
            <div class="flow-root">
              <ul class="-mb-8">
                <li v-for="(activity, index) in recentActivities" :key="index" class="animate-fade-in-up" :style="`animation-delay: ${0.5 + index * 0.1}s;`">
                  <div class="relative pb-8">
                    <span v-if="index !== recentActivities.length - 1" class="absolute top-5 left-5 -ml-px h-full w-0.5 bg-gradient-to-b from-blue-200 to-purple-200" aria-hidden="true"></span>
                    <div class="relative flex space-x-4 hover:bg-gray-50/50 rounded-xl p-2 transition-all duration-200">
                      <div>
                        <span class="h-10 w-10 rounded-xl bg-gradient-to-r from-blue-500 to-purple-500 flex items-center justify-center shadow-lg">
                          <svg class="h-5 w-5 text-white" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
                          </svg>
                        </span>
                      </div>
                      <div class="min-w-0 flex-1 pt-2">
                        <div class="flex justify-between items-start">
                          <div class="flex-1">
                            <p class="text-gray-900 font-medium">{{ activity.description }}</p>
                            <p class="text-sm text-gray-500 mt-1">{{ activity.details }}</p>
                          </div>
                          <div class="text-right ml-4">
                            <time class="text-sm text-gray-500">{{ activity.time }}</time>
                            <div class="text-xs text-gray-400 mt-1">{{ activity.type }}</div>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                </li>
              </ul>
              <div v-if="!recentActivities.length" class="text-center py-12">
                <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"></path>
                </svg>
                <h3 class="mt-4 text-lg font-medium text-gray-900">暂无活动记录</h3>
                <p class="mt-2 text-gray-500">开始上传文件来查看您的活动历史</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </main>

    <!-- 上传模态框占位符 -->
    <div v-if="showUploadModal" class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-[60]">
      <div class="relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white">
        <div class="mt-3 text-center">
          <h3 class="text-lg font-medium text-gray-900">上传功能</h3>
          <div class="mt-2 px-7 py-3">
            <p class="text-sm text-gray-500">
              上传功能将在媒体列表页面中实现。
            </p>
          </div>
          <div class="items-center px-4 py-3">
            <button
              @click="showUploadModal = false"
              class="px-4 py-2 bg-blue-500 text-white text-base font-medium rounded-md w-full shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-300"
            >
              关闭
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
@keyframes fade-in-up {
  0% {
    opacity: 0;
    transform: translateY(20px);
  }
  100% {
    opacity: 1;
    transform: translateY(0);
  }
}

.animate-fade-in-up {
  animation: fade-in-up 0.6s ease-out forwards;
  opacity: 0;
}
</style>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { authUtils, mediaAPI } from '../api';
import AppNavbar from '../components/AppNavbar.vue';

// 状态管理
const user = ref(authUtils.getStoredUser());
const showUploadModal = ref(false);
const stats = ref({
    totalMedia: 0,
    videoCount: 0,
    audioCount: 0,
    imageCount: 0,
});
const recentActivities = ref([
    {
        description: '上传了新的视频文件',
        details: 'sample_video.mp4',
        time: '2小时前',
        type: '上传'
    },
    {
        description: '删除了图片文件',
        details: 'old_image.jpg',
        time: '1天前',
        type: '删除'
    },
    {
        description: '更新了个人资料',
        details: '修改了用户名和头像',
        time: '3天前',
        type: '更新'
    }
]);

// 加载统计数据
const loadStats = async () => {
    try {
        const mediaList = await mediaAPI.getMediaList();
        stats.value.totalMedia = mediaList.total;
        stats.value.videoCount = mediaList.items?.filter(m => m.media_type === 'video').length;
        stats.value.audioCount = mediaList.items?.filter(m => m.media_type === 'audio').length;
        stats.value.imageCount = mediaList.items?.filter(m => m.media_type === 'image').length;
    } catch (error) {
        console.error('加载统计数据失败:', error);
    }
};



// 组件挂载时加载数据
onMounted(() => {
    loadStats();
});
</script>