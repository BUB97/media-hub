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
              to="/media"
              class="text-gray-700 hover:text-blue-600 px-3 py-2 rounded-md text-sm font-medium"
            >
              媒体库
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
      <!-- 返回按钮 -->
      <div class="px-4 py-4 sm:px-0">
        <button
          @click="$router.back()"
          class="flex items-center text-gray-600 hover:text-gray-900"
        >
          <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
          </svg>
          返回媒体库
        </button>
      </div>

      <div v-if="loading" class="text-center py-8">
        <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
        <p class="mt-2 text-gray-600">加载中...</p>
      </div>

      <div v-else-if="error" class="text-center py-8">
        <p class="text-red-600">{{ error }}</p>
        <button
          @click="loadMediaDetail"
          class="mt-2 bg-blue-600 text-white px-4 py-2 rounded-md text-sm font-medium hover:bg-blue-700"
        >
          重试
        </button>
      </div>

      <div v-else-if="media" class="space-y-6">
        <!-- 媒体信息卡片 -->
        <div class="bg-white shadow rounded-lg">
          <div class="px-6 py-4 border-b border-gray-200">
            <div class="flex justify-between items-start">
              <div>
                <h1 class="text-2xl font-bold text-gray-900">{{ media.title }}</h1>
                <p class="mt-1 text-sm text-gray-500">{{ getMediaTypeLabel(media.media_type) }}</p>
              </div>
              <div class="flex space-x-2">
                <button
                  @click="showEditModal = true"
                  class="bg-blue-600 text-white px-4 py-2 rounded-md text-sm font-medium hover:bg-blue-700"
                >
                  编辑
                </button>
                <button
                  @click="handleDelete"
                  class="bg-red-600 text-white px-4 py-2 rounded-md text-sm font-medium hover:bg-red-700"
                >
                  删除
                </button>
              </div>
            </div>
          </div>
          <div class="px-6 py-4">
            <div class="grid grid-cols-1 gap-6 sm:grid-cols-2">
              <div>
                <h3 class="text-lg font-medium text-gray-900 mb-2">基本信息</h3>
                <dl class="space-y-2">
                  <div>
                    <dt class="text-sm font-medium text-gray-500">标题</dt>
                    <dd class="text-sm text-gray-900">{{ media.title }}</dd>
                  </div>
                  <div>
                    <dt class="text-sm font-medium text-gray-500">描述</dt>
                    <dd class="text-sm text-gray-900">{{ media.description || '暂无描述' }}</dd>
                  </div>
                  <div>
                    <dt class="text-sm font-medium text-gray-500">类型</dt>
                    <dd class="text-sm text-gray-900">{{ getMediaTypeLabel(media.media_type) }}</dd>
                  </div>
                </dl>
              </div>
              <div>
                <h3 class="text-lg font-medium text-gray-900 mb-2">文件信息</h3>
                <dl class="space-y-2">
                  <div v-if="media.file_size">
                    <dt class="text-sm font-medium text-gray-500">文件大小</dt>
                    <dd class="text-sm text-gray-900">{{ formatFileSize(media.file_size) }}</dd>
                  </div>
                  <div v-if="media.duration">
                    <dt class="text-sm font-medium text-gray-500">时长</dt>
                    <dd class="text-sm text-gray-900">{{ formatDuration(media.duration) }}</dd>
                  </div>
                  <div v-if="media.created_at">
                    <dt class="text-sm font-medium text-gray-500">创建时间</dt>
                    <dd class="text-sm text-gray-900">{{ formatDate(media.created_at) }}</dd>
                  </div>
                </dl>
              </div>
            </div>
          </div>
        </div>

        <!-- 文件上传区域 -->
        <div class="bg-white shadow rounded-lg">
          <div class="px-6 py-4 border-b border-gray-200">
            <h2 class="text-lg font-medium text-gray-900">文件管理</h2>
          </div>
          <div class="px-6 py-4">
            <div v-if="!media.file_path" class="text-center py-8">
              <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"></path>
              </svg>
              <h3 class="mt-2 text-sm font-medium text-gray-900">暂无文件</h3>
              <p class="mt-1 text-sm text-gray-500">上传一个文件到这个媒体记录。</p>
              <div class="mt-6">
                <label class="cursor-pointer bg-blue-600 text-white px-4 py-2 rounded-md text-sm font-medium hover:bg-blue-700">
                  选择文件
                  <input
                    type="file"
                    class="hidden"
                    @change="handleFileSelect"
                    :accept="getAcceptedFileTypes(media.media_type)"
                  />
                </label>
              </div>
            </div>
            <div v-else class="space-y-4">
              <div class="flex items-center justify-between">
                <div class="flex items-center">
                  <svg class="h-8 w-8 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                  </svg>
                  <div class="ml-3">
                    <p class="text-sm font-medium text-gray-900">文件已上传</p>
                    <p class="text-sm text-gray-500">{{ media.file_path }}</p>
                  </div>
                </div>
                <div class="flex space-x-2">
                  <a
                    :href="mediaAPI.getMediaDownloadUrl(media.id)"
                    target="_blank"
                    class="bg-green-600 text-white px-3 py-1 rounded text-sm hover:bg-green-700"
                  >
                    下载
                  </a>
                  <label class="cursor-pointer bg-blue-600 text-white px-3 py-1 rounded text-sm hover:bg-blue-700">
                    替换
                    <input
                      type="file"
                      class="hidden"
                      @change="handleFileSelect"
                      :accept="getAcceptedFileTypes(media.media_type)"
                    />
                  </label>
                </div>
              </div>
            </div>

            <!-- 上传进度 -->
            <div v-if="uploadProgress > 0 && uploadProgress < 100" class="mt-4">
              <div class="flex justify-between text-sm text-gray-600 mb-1">
                <span>上传进度</span>
                <span>{{ uploadProgress }}%</span>
              </div>
              <div class="w-full bg-gray-200 rounded-full h-2">
                <div
                  class="bg-blue-600 h-2 rounded-full transition-all duration-300"
                  :style="{ width: uploadProgress + '%' }"
                ></div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </main>

    <!-- 编辑模态框 -->
    <div v-if="showEditModal" class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
      <div class="relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white">
        <div class="mt-3">
          <h3 class="text-lg font-medium text-gray-900 text-center">编辑媒体信息</h3>
          <form @submit.prevent="handleUpdate" class="mt-4 space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700">标题</label>
              <input
                v-model="editForm.title"
                type="text"
                required
                class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700">描述</label>
              <textarea
                v-model="editForm.description"
                rows="3"
                class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
              ></textarea>
            </div>
            <div class="flex justify-end space-x-3 pt-4">
              <button
                type="button"
                @click="showEditModal = false"
                class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50"
              >
                取消
              </button>
              <button
                type="submit"
                :disabled="updateLoading"
                class="px-4 py-2 bg-blue-600 text-white rounded-md text-sm font-medium hover:bg-blue-700 disabled:opacity-50"
              >
                {{ updateLoading ? '更新中...' : '更新' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { authUtils, authAPI, mediaAPI, type Media } from '../api'

const router = useRouter()
const route = useRoute()

// 状态管理
const media = ref<Media | null>(null)
const loading = ref(false)
const error = ref('')
const showEditModal = ref(false)
const updateLoading = ref(false)
const uploadProgress = ref(0)

// 编辑表单
const editForm = ref({
  title: '',
  description: '',
})

// 获取媒体类型标签
const getMediaTypeLabel = (type: string) => {
  const labels = {
    video: '视频',
    audio: '音频',
    image: '图片',
  }
  return labels[type as keyof typeof labels] || type
}

// 格式化文件大小
const formatFileSize = (bytes: number) => {
  if (bytes === 0) return '0 Bytes'
  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

// 格式化时长
const formatDuration = (seconds: number) => {
  const hours = Math.floor(seconds / 3600)
  const minutes = Math.floor((seconds % 3600) / 60)
  const secs = Math.floor(seconds % 60)
  
  if (hours > 0) {
    return `${hours}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`
  }
  return `${minutes}:${secs.toString().padStart(2, '0')}`
}

// 格式化日期
const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleString('zh-CN')
}

// 获取接受的文件类型
const getAcceptedFileTypes = (mediaType: string) => {
  const types = {
    video: 'video/*',
    audio: 'audio/*',
    image: 'image/*',
  }
  return types[mediaType as keyof typeof types] || '*/*'
}

// 加载媒体详情
const loadMediaDetail = async () => {
  const id = parseInt(route.params.id as string)
  if (!id) {
    error.value = '无效的媒体 ID'
    return
  }

  loading.value = true
  error.value = ''
  
  try {
    media.value = await mediaAPI.getMediaById(id)
    // 初始化编辑表单
    editForm.value = {
      title: media.value.title,
      description: media.value.description || '',
    }
  } catch (err: any) {
    console.error('加载媒体详情失败:', err)
    if (err.response?.status === 404) {
      error.value = '媒体不存在'
    } else {
      error.value = '加载媒体详情失败，请稍后重试'
    }
  } finally {
    loading.value = false
  }
}

// 处理文件选择
const handleFileSelect = async (event: Event) => {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (!file || !media.value) return

  uploadProgress.value = 0
  
  try {
    const updatedMedia = await mediaAPI.uploadMediaFile(
      media.value.id,
      file,
      (progress) => {
        uploadProgress.value = progress
      }
    )
    
    media.value = updatedMedia
    uploadProgress.value = 100
    
    // 重置进度条
    setTimeout(() => {
      uploadProgress.value = 0
    }, 2000)
  } catch (err: any) {
    console.error('文件上传失败:', err)
    alert('文件上传失败，请稍后重试')
    uploadProgress.value = 0
  }
  
  // 清除文件输入
  target.value = ''
}

// 处理更新
const handleUpdate = async () => {
  if (!media.value) return
  
  updateLoading.value = true
  
  try {
    const updatedMedia = await mediaAPI.updateMedia(media.value.id, {
      title: editForm.value.title,
      description: editForm.value.description,
    })
    
    media.value = updatedMedia
    showEditModal.value = false
  } catch (err: any) {
    console.error('更新媒体失败:', err)
    alert('更新媒体失败，请稍后重试')
  } finally {
    updateLoading.value = false
  }
}

// 处理删除
const handleDelete = async () => {
  if (!media.value) return
  
  if (!confirm(`确定要删除 "${media.value.title}" 吗？`)) {
    return
  }
  
  try {
    await mediaAPI.deleteMedia(media.value.id)
    router.push('/media')
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
    router.push('/')
  } catch (error) {
    console.error('登出失败:', error)
    authUtils.clearAuthData()
    router.push('/')
  }
}

// 组件挂载时加载数据
onMounted(() => {
  loadMediaDetail()
})
</script>