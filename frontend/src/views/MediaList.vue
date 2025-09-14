<template>
  <div class="min-h-screen bg-gray-50">
    <!-- 导航栏 -->
    <nav class="bg-white shadow">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16">
          <div class="flex items-center">
            <router-link to="/" class="text-xl font-bold text-gray-900">Media Hub</router-link>
          </div>
          <div class="flex items-center space-x-4">
            <router-link
              to="/dashboard"
              class="text-gray-700 hover:text-blue-600 px-3 py-2 rounded-md text-sm font-medium"
            >
              仪表板
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
          </div>
        </div>
      </div>
    </nav>

    <!-- 主要内容 -->
    <main class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
      <!-- 页面标题和操作 -->
      <div class="px-4 py-6 sm:px-0">
        <div class="flex justify-between items-center">
          <h1 class="text-2xl font-bold text-gray-900">媒体库</h1>
          <button
            @click="showCreateModal = true"
            class="bg-blue-600 text-white px-4 py-2 rounded-md text-sm font-medium hover:bg-blue-700"
          >
            添加媒体
          </button>
        </div>
      </div>

      <!-- 筛选器 -->
      <div class="px-4 py-4 sm:px-0">
        <div class="bg-white shadow rounded-lg p-4">
          <div class="flex flex-wrap gap-4 items-center">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">类型筛选</label>
              <select
                v-model="filter.mediaType"
                @change="loadMediaList"
                class="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
              >
                <option value="">所有类型</option>
                <option value="video">视频</option>
                <option value="audio">音频</option>
                <option value="image">图片</option>
              </select>
            </div>
            <div class="flex-1">
              <label class="block text-sm font-medium text-gray-700 mb-1">搜索</label>
              <input
                v-model="filter.search"
                @input="debounceSearch"
                type="text"
                placeholder="搜索标题或描述..."
                class="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
              />
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

        <div v-else-if="filteredMediaList.length === 0" class="text-center py-8">
          <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path>
          </svg>
          <h3 class="mt-2 text-sm font-medium text-gray-900">暂无媒体文件</h3>
          <p class="mt-1 text-sm text-gray-500">开始添加一些媒体文件到您的库中。</p>
          <div class="mt-6">
            <button
              @click="showCreateModal = true"
              class="bg-blue-600 text-white px-4 py-2 rounded-md text-sm font-medium hover:bg-blue-700"
            >
              添加第一个媒体文件
            </button>
          </div>
        </div>

        <div v-else class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
          <div
            v-for="media in filteredMediaList"
            :key="media.id"
            class="bg-white overflow-hidden shadow rounded-lg hover:shadow-md transition-shadow cursor-pointer"
            @click="$router.push(`/media/${media.id}`)"
          >
            <div class="p-6">
              <div class="flex items-center justify-between">
                <div class="flex items-center">
                  <div class="flex-shrink-0">
                    <component :is="getMediaIcon(media.media_type)" class="h-8 w-8 text-gray-400" />
                  </div>
                  <div class="ml-4">
                    <h3 class="text-lg font-medium text-gray-900 truncate">{{ media.title }}</h3>
                    <p class="text-sm text-gray-500">{{ getMediaTypeLabel(media.media_type) }}</p>
                  </div>
                </div>
                <div class="flex space-x-2">
                  <button
                    @click.stop="editMedia(media)"
                    class="text-blue-600 hover:text-blue-800"
                  >
                    <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                    </svg>
                  </button>
                  <button
                    @click.stop="deleteMedia(media)"
                    class="text-red-600 hover:text-red-800"
                  >
                    <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
                    </svg>
                  </button>
                </div>
              </div>
              <div class="mt-4">
                <p class="text-sm text-gray-600 line-clamp-2">{{ media.description }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </main>

    <!-- 创建媒体模态框 -->
    <div v-if="showCreateModal" class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
      <div class="relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white">
        <div class="mt-3">
          <h3 class="text-lg font-medium text-gray-900 text-center">添加新媒体</h3>
          <form @submit.prevent="handleCreateMedia" class="mt-4 space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700">标题</label>
              <input
                v-model="createForm.title"
                type="text"
                required
                class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700">描述</label>
              <textarea
                v-model="createForm.description"
                rows="3"
                class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
              ></textarea>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700">类型</label>
              <select
                v-model="createForm.media_type"
                required
                class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
              >
                <option value="">选择类型</option>
                <option value="video">视频</option>
                <option value="audio">音频</option>
                <option value="image">图片</option>
              </select>
            </div>
            <div class="flex justify-end space-x-3 pt-4">
              <button
                type="button"
                @click="showCreateModal = false"
                class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50"
              >
                取消
              </button>
              <button
                type="submit"
                :disabled="createLoading"
                class="px-4 py-2 bg-blue-600 text-white rounded-md text-sm font-medium hover:bg-blue-700 disabled:opacity-50"
              >
                {{ createLoading ? '创建中...' : '创建' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { authUtils, authAPI, mediaAPI, type Media } from '../api'

// 状态管理
const mediaList = ref<Media[]>([])
const loading = ref(false)
const error = ref('')
const showCreateModal = ref(false)
const createLoading = ref(false)

// 筛选器
const filter = ref({
  mediaType: '',
  search: '',
})

// 创建表单
const createForm = ref({
  title: '',
  description: '',
  media_type: '' as 'video' | 'audio' | 'image' | '',
})

// 计算属性 - 筛选后的媒体列表
const filteredMediaList = computed(() => {
  let filtered = mediaList.value

  if (filter.value.mediaType) {
    filtered = filtered.filter(media => media.media_type === filter.value.mediaType)
  }

  if (filter.value.search) {
    const searchTerm = filter.value.search.toLowerCase()
    filtered = filtered.filter(media => 
      media.title.toLowerCase().includes(searchTerm) ||
      media.description.toLowerCase().includes(searchTerm)
    )
  }

  return filtered
})

// 获取媒体类型图标
const getMediaIcon = (_type: string) => {
  return 'svg' // 简化处理，实际应该返回对应的 SVG 组件
}

// 获取媒体类型标签
const getMediaTypeLabel = (type: string) => {
  const labels = {
    video: '视频',
    audio: '音频',
    image: '图片',
  }
  return labels[type as keyof typeof labels] || type
}

// 防抖搜索
let searchTimeout: number
const debounceSearch = () => {
  clearTimeout(searchTimeout)
  searchTimeout = setTimeout(() => {
    // 搜索逻辑已在计算属性中处理
  }, 300)
}

// 加载媒体列表
const loadMediaList = async () => {
  loading.value = true
  error.value = ''
  
  try {
    mediaList.value = await mediaAPI.getMediaList()
  } catch (err: any) {
    console.error('加载媒体列表失败:', err)
    error.value = '加载媒体列表失败，请稍后重试'
  } finally {
    loading.value = false
  }
}

// 创建媒体
const handleCreateMedia = async () => {
  createLoading.value = true
  
  try {
    const newMedia = await mediaAPI.createMedia({
      title: createForm.value.title,
      description: createForm.value.description,
      media_type: createForm.value.media_type as 'video' | 'audio' | 'image',
    })
    
    mediaList.value.unshift(newMedia)
    showCreateModal.value = false
    
    // 重置表单
    createForm.value = {
      title: '',
      description: '',
      media_type: '',
    }
  } catch (err: any) {
    console.error('创建媒体失败:', err)
    alert('创建媒体失败，请稍后重试')
  } finally {
    createLoading.value = false
  }
}

// 编辑媒体
const editMedia = (media: Media) => {
  getApp().goTo(`/media/${media.id}`)
}

// 删除媒体
const deleteMedia = async (media: Media) => {
  if (!confirm(`确定要删除 "${media.title}" 吗？`)) {
    return
  }
  
  try {
    await mediaAPI.deleteMedia(media.id)
    mediaList.value = mediaList.value.filter(m => m.id !== media.id)
  } catch (err: any) {
    console.error('删除媒体失败:', err)
    alert('删除媒体失败，请稍后重试')
  }
}

// 处理登出
const handleLogout = async () => {
  try {
    await authAPI.logout()
    authUtils.clearAuthData()
  } catch (error) {
    console.error('登出失败:', error)
    authUtils.clearAuthData()
  }
    getApp().goTo("/")
}

// 组件挂载时加载数据
onMounted(() => {
  loadMediaList()
})
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>