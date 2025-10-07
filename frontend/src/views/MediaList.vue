<template>
  <div class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100">
    <!-- 统一导航栏 -->
    <AppNavbar current-page="media" />

    <!-- 主要内容 -->
    <main class="max-w-7xl mx-auto py-8 sm:px-6 lg:px-8">
      <!-- 页面标题和操作 -->
      <div class="px-4 py-6 sm:px-0">
        <div class="flex flex-col sm:flex-row sm:justify-between sm:items-center gap-4">
          <div>
            <h1 class="text-3xl font-bold text-gray-900">媒体库</h1>
            <p class="mt-2 text-gray-600">管理和浏览您的所有媒体文件</p>
          </div>
          <button
            @click="$router.push('/upload')"
            class="bg-blue-600 hover:from-primary-700 hover:to-secondary-700 text-white px-6 py-3 rounded-xl text-sm font-medium transition-all duration-200 shadow-lg hover:shadow-xl transform hover:scale-105 flex items-center"
          >
            <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
            </svg>
            添加媒体
          </button>
        </div>
      </div>

      <!-- 筛选器 -->
      <div class="px-4 py-4 sm:px-0">
        <div class="bg-white/80 backdrop-blur-sm shadow-lg rounded-2xl border border-white/20 p-4 sm:p-6 mobile-card">
          <div class="flex flex-col lg:flex-row gap-6 items-start lg:items-end">
            <div class="w-full lg:w-auto">
              <label class="block text-sm font-semibold text-gray-700 mb-2 mobile-text-sm">类型筛选</label>
              <div class="flex flex-wrap gap-2">
                <button
                  @click="filter.mediaType = ''; handleFilterChange()"
                  :class="[
                    'px-3 sm:px-4 py-2 rounded-xl text-sm font-medium transition-all duration-200 touch-target mobile-button mobile-text-sm',
                    filter.mediaType === '' 
                      ? 'bg-blue-600 text-white shadow-lg' 
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  ]"
                >
                  全部
                </button>
                <button
                  @click="filter.mediaType = 'video'; handleFilterChange()"
                  :class="[
                    'px-3 sm:px-4 py-2 rounded-xl text-sm font-medium transition-all duration-200 flex items-center touch-target mobile-button mobile-text-sm',
                    filter.mediaType === 'video' 
                      ? 'bg-gradient-to-r from-purple-500 to-purple-600 text-white shadow-lg' 
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  ]"
                >
                  <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                  </svg>
                  视频
                </button>
                <button
                  @click="filter.mediaType = 'audio'; handleFilterChange()"
                  :class="[
                    'px-3 sm:px-4 py-2 rounded-xl text-sm font-medium transition-all duration-200 flex items-center touch-target mobile-button mobile-text-sm',
                    filter.mediaType === 'audio' 
                      ? 'bg-gradient-to-r from-green-500 to-green-600 text-white shadow-lg' 
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  ]"
                >
                  <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3"></path>
                  </svg>
                  音频
                </button>
                <button
                  @click="filter.mediaType = 'image'; handleFilterChange()"
                  :class="[
                    'px-3 sm:px-4 py-2 rounded-xl text-sm font-medium transition-all duration-200 flex items-center touch-target mobile-button mobile-text-sm',
                    filter.mediaType === 'image' 
                      ? 'bg-gradient-to-r from-pink-500 to-pink-600 text-white shadow-lg' 
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  ]"
                >
                  <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
                  </svg>
                  图片
                </button>
              </div>
            </div>
            <div class="flex-1 w-full lg:w-auto">
              <label class="block text-sm font-semibold text-gray-700 mb-2">搜索</label>
              <div class="relative">
                <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                  <svg class="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
                  </svg>
                </div>
                <input
                  v-model="filter.search"
                  @input="debounceSearch"
                  type="text"
                  placeholder="搜索标题或描述..."
                  class="block w-full pl-10 pr-3 py-3 border border-gray-200 rounded-xl shadow-sm focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500 text-sm bg-white/50 backdrop-blur-sm"
                />
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 媒体列表 -->
      <div class="px-4 py-6 sm:px-0">
        <div v-if="loading" class="text-center py-8">
          <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
          <p class="mt-2 text-gray-600">加载中...</p>
        </div>

        <div v-else-if="error" class="text-center py-8">
          <p class="text-red-600">{{ error }}</p>
          <button
            @click="loadMediaList"
            class="mt-2 bg-blue-600 text-white px-4 py-2 rounded-md text-sm font-medium hover:bg-blue-700"
          >
            重试
          </button>
        </div>

        <div v-else-if="filteredMediaList.length === 0" class="text-center py-16">
          <div class="bg-white/80 backdrop-blur-sm rounded-2xl shadow-lg border border-white/20 p-12 max-w-md mx-auto">
            <div class="h-16 w-16 bg-gradient-to-r from-gray-400 to-gray-500 rounded-2xl flex items-center justify-center mx-auto mb-6">
              <svg class="h-8 w-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path>
              </svg>
            </div>
            <h3 class="text-xl font-bold text-gray-900 mb-2">暂无媒体文件</h3>
            <p class="text-gray-600">开始添加一些媒体文件到您的库中，让您的创作之旅开始吧！</p>
          </div>
        </div>

        <div v-else class="grid grid-cols-1 gap-4 sm:gap-6 md:gap-8 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 mobile-padding">
          <div
            v-for="(media, index) in filteredMediaList"
            :key="media.id"
            class="group bg-white/80 backdrop-blur-sm overflow-hidden shadow-lg rounded-2xl border border-white/20 hover:shadow-2xl transition-all duration-300 cursor-pointer transform hover:scale-105 animate-scale-in hover-lift mobile-card touch-target"
            :style="`animation-delay: ${index * 0.1}s; animation-fill-mode: both;`"
            @click="$router.push(`/media/${media.id}`)"
          >
            <!-- 媒体预览区域 -->
            <div class="relative h-48 bg-gradient-to-br from-gray-100 to-gray-200 overflow-hidden">
              <div class="absolute inset-0 flex items-center justify-center">
                <div :class="getMediaIconBg(media.media_type)" class="h-16 w-16 rounded-2xl flex items-center justify-center shadow-lg">
                  <svg :class="getMediaIconColor(media.media_type)" class="h-8 w-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path v-if="media.media_type === 'video'" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                    <path v-else-if="media.media_type === 'audio'" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3"></path>
                    <path v-else-if="media.media_type === 'image'" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
                    <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z"></path>
                  </svg>
                </div>
              </div>
              <!-- 类型标签 -->
              <div class="absolute top-4 left-4">
                <span :class="getMediaTypeBadge(media.media_type)" class="px-3 py-1 rounded-full text-xs font-medium">
                  {{ getMediaTypeLabel(media.media_type) }}
                </span>
              </div>
              <!-- 操作按钮 -->
              <div class="absolute top-4 right-4 opacity-0 group-hover:opacity-100 transition-opacity duration-200">
                <div class="flex space-x-2">
                  <button
                    @click.stop="downloadMedia(media)"
                    class="bg-white/90 backdrop-blur-sm text-gray-700 hover:text-green-600 p-2 rounded-xl shadow-lg hover:shadow-xl transition-all duration-200"
                  >
                    <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                    </svg>
                  </button>
                  <button
                    @click.stop="editMedia(media)"
                    class="bg-white/90 backdrop-blur-sm text-gray-700 hover:text-primary-600 p-2 rounded-xl shadow-lg hover:shadow-xl transition-all duration-200"
                  >
                    <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                    </svg>
                  </button>
                  <button
                    @click.stop="deleteMedia(media, $event)"
                    class="bg-white/90 backdrop-blur-sm text-gray-700 hover:text-red-600 p-2 rounded-xl shadow-lg hover:shadow-xl transition-all duration-200"
                  >
                    <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
                    </svg>
                  </button>
                </div>
              </div>
            </div>
            
            <!-- 媒体信息区域 -->
            <div class="p-6">
              <div class="mb-3">
                <h3 class="text-lg font-bold text-gray-900 truncate group-hover:text-primary-600 transition-colors duration-200">{{ media.title }}</h3>
                <p class="text-sm text-gray-500 mt-1">{{ media.description || '暂无描述' }}</p>
              </div>
              
              <!-- 媒体元数据 -->
              <div class="flex items-center justify-between text-xs text-gray-500">
                <div class="flex items-center space-x-4">
                  <span class="flex items-center">
                    <svg class="h-3 w-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                    </svg>
                    {{ formatDate(media.created_at) }}
                  </span>
                  <span v-if="media.file_size" class="flex items-center">
                    <svg class="h-3 w-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z"></path>
                    </svg>
                    {{ formatFileSize(media.file_size) }}
                  </span>
                </div>
                <div class="flex items-center space-x-1">
                  <div class="h-2 w-2 bg-green-400 rounded-full"></div>
                  <span>可用</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- 分页组件 -->
        <div v-if="totalPages > 1" class="px-4 py-6 sm:px-0">
          <div class="bg-white/80 backdrop-blur-sm shadow-lg rounded-2xl border border-white/20 p-4 sm:p-6">
            <div class="flex items-center justify-between">
              <div class="text-sm text-gray-700">
                显示第 {{ (currentPage - 1) * pageSize + 1 }} - {{ Math.min(currentPage * pageSize, totalCount) }} 条，共 {{ totalCount }} 条
              </div>
              <div class="flex items-center space-x-2">
                <button
                  @click="handlePageChange(currentPage - 1)"
                  :disabled="currentPage === 1"
                  class="px-3 py-2 text-sm font-medium text-gray-500 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 hover:text-gray-700 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  上一页
                </button>
                
                <div class="flex items-center space-x-1">
                  <template v-for="page in getPageNumbers()" :key="page">
                     <button
                       v-if="typeof page === 'number'"
                       @click="handlePageChange(page)"
                       :class="[
                         'px-3 py-2 text-sm font-medium rounded-lg transition-colors duration-200',
                         page === currentPage
                           ? 'bg-gradient-to-r from-primary-500 to-secondary-500 text-white shadow-lg'
                           : 'text-gray-700 bg-white border border-gray-300 hover:bg-gray-50 hover:text-gray-900'
                       ]"
                     >
                       {{ page }}
                     </button>
                     <span v-else class="px-2 py-2 text-sm text-gray-500">{{ page }}</span>
                   </template>
                </div>
                
                <button
                  @click="handlePageChange(currentPage + 1)"
                  :disabled="currentPage === totalPages"
                  class="px-3 py-2 text-sm font-medium text-gray-500 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 hover:text-gray-700 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  下一页
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </main>


  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { mediaAPI, type Media, type MediaListResponse, type MediaQueryParams } from '../api';
import AppNavbar from '../components/AppNavbar.vue';

// 状态管理
const mediaList = ref<Media[]>([]);
const loading = ref(false);
const error = ref('');
const totalCount = ref(0);
const currentPage = ref(1);
const pageSize = ref(12);

// 筛选器
const filter = ref({
  mediaType: '',
  search: '',
});

// 计算属性 - 筛选后的媒体列表（现在由后端处理筛选）
const filteredMediaList = computed(() => {
  return mediaList.value;
});

// 计算总页数
const totalPages = computed(() => {
  return Math.ceil(totalCount.value / pageSize.value);
});

// 获取媒体类型图标
// const getMediaIcon = (_type: string) => {
//   return 'svg'; // 简化处理，实际应该返回对应的 SVG 组件
// };

// 获取媒体类型标签
const getMediaTypeLabel = (type: string) => {
  const labels = {
    video: '视频',
    audio: '音频',
    image: '图片',
  };
  return labels[type as keyof typeof labels] || type;
};

// 获取媒体图标背景
const getMediaIconBg = (type: string) => {
  const backgrounds = {
    video: 'bg-gradient-to-r from-red-500 to-pink-500',
    audio: 'bg-gradient-to-r from-purple-500 to-indigo-500',
    image: 'bg-gradient-to-r from-green-500 to-teal-500',
    document: 'bg-gradient-to-r from-blue-500 to-cyan-500'
  };
  return backgrounds[type as keyof typeof backgrounds] || 'bg-gradient-to-r from-gray-500 to-gray-600';
};

// 获取媒体图标颜色
const getMediaIconColor = (type: string) => {
  console.log('type', type);
  return 'text-white';
};

// 获取媒体类型徽章样式
const getMediaTypeBadge = (type: string) => {
  const badges = {
    video: 'bg-red-100 text-red-800',
    audio: 'bg-purple-100 text-purple-800',
    image: 'bg-green-100 text-green-800',
    document: 'bg-blue-100 text-blue-800'
  };
  return badges[type as keyof typeof badges] || 'bg-gray-100 text-gray-800';
};

// 格式化日期
const formatDate = (dateString: string | undefined) => {
  if (!dateString) { return '未知时间'; }
  const date = new Date(dateString);
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  });
};

// 格式化文件大小
const formatFileSize = (bytes: number) => {
  if (bytes === 0) { return '0 B'; }
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

// 防抖搜索
let searchTimeout: number;
const debounceSearch = () => {
  clearTimeout(searchTimeout);
  searchTimeout = setTimeout(() => {
    currentPage.value = 1; // 重置到第一页
    loadMediaList();
  }, 300);
};

// 筛选器变化处理
const handleFilterChange = () => {
  currentPage.value = 1; // 重置到第一页
  loadMediaList();
};

// 分页处理
const handlePageChange = (page: number) => {
  currentPage.value = page;
  loadMediaList();
};

// 下载媒体文件
const downloadMedia = async (media: Media) => {
  try {
    const downloadUrl = mediaAPI.getMediaDownloadUrl(media);
    const link = document.createElement('a');
    link.href = downloadUrl;
    link.download = media.title;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  } catch (err: any) {
    console.error('下载失败:', err);
    alert('下载失败，请稍后重试');
  }
};

// 加载媒体列表
const loadMediaList = async () => {
  loading.value = true;
  error.value = '';
  
  try {
    const params: MediaQueryParams = {
      page: currentPage.value,
      per_page: pageSize.value,
      media_type: filter.value.mediaType || undefined,
      q: filter.value.search || undefined
    };
    
    const response: MediaListResponse = await mediaAPI.getMediaList(params);
    mediaList.value = response.items;
    totalCount.value = response.total;
  } catch (err: any) {
    console.error('加载媒体列表失败:', err);
    error.value = '加载媒体列表失败，请稍后重试';
  } finally {
    loading.value = false;
  }
};

// 编辑媒体
const editMedia = (media: Media) => {
  // 使用 Vue Router 进行导航
  getApp().goTo(`/media/${media.id}`);
};

// 删除媒体
const deleteMedia = async (media: Media, event: Event) => {
  event.stopPropagation(); // 阻止事件冒泡
  
  if (!confirm(`确定要删除 "${media.title}" 吗？`)) {
    return;
  }
  
  try {
    await mediaAPI.deleteMedia(media.id);
    // 重新加载当前页
    loadMediaList();
  } catch (err: any) {
    console.error('删除媒体失败:', err);
    alert('删除媒体失败，请稍后重试');
  }
};



// 获取分页页码数组
const getPageNumbers = () => {
  const pages = [];
  const total = totalPages.value;
  const current = currentPage.value;
  
  if (total <= 7) {
    // 如果总页数小于等于7，显示所有页码
    for (let i = 1; i <= total; i++) {
      pages.push(i);
    }
  } else {
    // 总是显示第一页
    pages.push(1);
    
    if (current <= 4) {
      // 当前页在前面时
      for (let i = 2; i <= 5; i++) {
        pages.push(i);
      }
      pages.push('...');
      pages.push(total);
    } else if (current >= total - 3) {
      // 当前页在后面时
      pages.push('...');
      for (let i = total - 4; i <= total; i++) {
        pages.push(i);
      }
    } else {
      // 当前页在中间时
      pages.push('...');
      for (let i = current - 1; i <= current + 1; i++) {
        pages.push(i);
      }
      pages.push('...');
      pages.push(total);
    }
  }
  
  return pages;
};

// 组件挂载时加载数据
onMounted(() => {
  loadMediaList();
});
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>