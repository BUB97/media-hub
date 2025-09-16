<template>
  <div class="min-h-screen bg-gradient-to-br from-gray-50 via-blue-50 to-purple-50">
    <!-- 统一导航栏 -->
    <AppNavbar />

    <!-- 主要内容 -->
    <main class="max-w-7xl mx-auto py-8 sm:px-6 lg:px-8">
      <!-- 返回按钮 -->
      <div class="px-4 py-4 sm:px-0">
        <button
          @click="$router.back()"
          class="group flex items-center text-gray-600 hover:text-primary-600 transition-colors duration-200"
        >
          <div class="bg-gray-100 group-hover:bg-primary-100 p-2 rounded-xl mr-3 transition-colors duration-200">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
            </svg>
          </div>
          <span class="font-medium">返回媒体库</span>
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

      <div v-else-if="media" class="space-y-8">
        <!-- 媒体信息卡片 -->
        <div class="bg-white/70 backdrop-blur-sm shadow-xl rounded-2xl border border-white/20 overflow-hidden">
          <div class="px-8 py-6 border-b border-gray-100 bg-gradient-to-r from-primary-50 to-secondary-50">
            <div class="flex justify-between items-start">
              <div class="flex items-start space-x-4">
                <div class="h-16 w-16 bg-gradient-to-r from-primary-500 to-secondary-500 rounded-2xl flex items-center justify-center shadow-lg">
                  <svg class="h-8 w-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                  </svg>
                </div>
                <div>
                  <h1 class="text-3xl font-bold text-gray-900 mb-2">{{ media.title }}</h1>
                  <div class="flex items-center space-x-2">
                    <span class="inline-flex items-center px-3 py-1 rounded-full text-xs font-medium bg-primary-100 text-primary-800">
                      {{ getMediaTypeLabel(media.media_type) }}
                    </span>
                  </div>
                </div>
              </div>
              <div class="flex space-x-3">
                <button
                  @click="showEditModal = true"
                  class="bg-gradient-to-r from-blue-500 to-blue-600 hover:from-blue-600 hover:to-blue-700 text-white px-6 py-3 rounded-xl text-sm font-medium transition-all duration-200 shadow-lg hover:shadow-xl flex items-center space-x-2"
                >
                  <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                  </svg>
                  <span>编辑</span>
                </button>
                <button
                  @click="handleDelete"
                  class="bg-gradient-to-r from-red-500 to-red-600 hover:from-red-600 hover:to-red-700 text-white px-6 py-3 rounded-xl text-sm font-medium transition-all duration-200 shadow-lg hover:shadow-xl flex items-center space-x-2"
                >
                  <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
                  </svg>
                  <span>删除</span>
                </button>
              </div>
            </div>
          </div>
          <div class="px-8 py-6">
            <div class="grid grid-cols-1 gap-8 sm:grid-cols-2">
              <div class="bg-gradient-to-br from-blue-50 to-indigo-50 rounded-2xl p-6 border border-blue-100">
                <div class="flex items-center mb-4">
                  <div class="h-8 w-8 bg-gradient-to-r from-blue-500 to-indigo-500 rounded-xl flex items-center justify-center mr-3">
                    <svg class="h-4 w-4 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                    </svg>
                  </div>
                  <h3 class="text-lg font-semibold text-gray-900">基本信息</h3>
                </div>
                <dl class="space-y-4">
                  <div class="bg-white/50 rounded-xl p-4">
                    <dt class="text-sm font-medium text-gray-600 mb-1">标题</dt>
                    <dd class="text-sm font-semibold text-gray-900">{{ media.title }}</dd>
                  </div>
                  <div class="bg-white/50 rounded-xl p-4">
                    <dt class="text-sm font-medium text-gray-600 mb-1">描述</dt>
                    <dd class="text-sm text-gray-900">{{ media.description || '暂无描述' }}</dd>
                  </div>
                  <div class="bg-white/50 rounded-xl p-4">
                    <dt class="text-sm font-medium text-gray-600 mb-1">类型</dt>
                    <dd class="text-sm font-semibold text-gray-900">{{ getMediaTypeLabel(media.media_type) }}</dd>
                  </div>
                </dl>
              </div>
              <div class="bg-gradient-to-br from-purple-50 to-pink-50 rounded-2xl p-6 border border-purple-100">
                <div class="flex items-center mb-4">
                  <div class="h-8 w-8 bg-gradient-to-r from-purple-500 to-pink-500 rounded-xl flex items-center justify-center mr-3">
                    <svg class="h-4 w-4 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                    </svg>
                  </div>
                  <h3 class="text-lg font-semibold text-gray-900">文件信息</h3>
                </div>
                <dl class="space-y-4">
                  <div v-if="media.file_size" class="bg-white/50 rounded-xl p-4">
                    <dt class="text-sm font-medium text-gray-600 mb-1">文件大小</dt>
                    <dd class="text-sm font-semibold text-gray-900">{{ formatFileSize(media.file_size) }}</dd>
                  </div>
                  <div v-if="media.created_at" class="bg-white/50 rounded-xl p-4">
                    <dt class="text-sm font-medium text-gray-600 mb-1">创建时间</dt>
                    <dd class="text-sm font-semibold text-gray-900">{{ formatDate(media.created_at) }}</dd>
                  </div>
                </dl>
              </div>
            </div>
          </div>
        </div>

        <!-- 文件上传区域 -->
        <div class="bg-white/70 backdrop-blur-sm shadow-xl rounded-2xl border border-white/20 overflow-hidden">
          <div class="px-8 py-6 border-b border-gray-100 bg-gradient-to-r from-green-50 to-emerald-50">
            <div class="flex items-center">
              <div class="h-8 w-8 bg-gradient-to-r from-green-500 to-emerald-500 rounded-xl flex items-center justify-center mr-3">
                <svg class="h-4 w-4 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"></path>
                </svg>
              </div>
              <h2 class="text-xl font-semibold text-gray-900">文件管理</h2>
            </div>
          </div>
          <div class="px-8 py-6">
            <div v-if="!media.cos_url" class="text-center py-12">
              <div class="bg-gradient-to-br from-gray-100 to-gray-200 rounded-3xl p-8 mx-auto w-32 h-32 flex items-center justify-center mb-6">
                <svg class="h-16 w-16 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"></path>
                </svg>
              </div>
              <h3 class="text-xl font-semibold text-gray-900 mb-2">暂无文件</h3>
              <p class="text-gray-500 mb-8 max-w-sm mx-auto">上传一个文件到这个媒体记录，开始管理您的媒体内容。</p>
              <div>
                <label class="cursor-pointer bg-gradient-to-r from-primary-500 to-secondary-500 hover:from-primary-600 hover:to-secondary-600 text-white px-8 py-4 rounded-2xl text-sm font-medium transition-all duration-200 shadow-lg hover:shadow-xl inline-flex items-center space-x-2">
                  <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
                  </svg>
                  <span>选择文件</span>
                  <input
                    type="file"
                    class="hidden"
                    @change="handleFileSelect"
                    :accept="getAcceptedFileTypes(media.media_type)"
                  />
                </label>
              </div>
            </div>
            <div v-else class="space-y-6">
              <div class="bg-gradient-to-r from-green-50 to-emerald-50 rounded-2xl p-6 border border-green-200">
                <div class="flex items-center justify-between">
                  <div class="flex items-center">
                    <div class="h-12 w-12 bg-gradient-to-r from-green-500 to-emerald-500 rounded-2xl flex items-center justify-center mr-4">
                      <svg class="h-6 w-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                      </svg>
                    </div>
                    <div>
                      <p class="text-lg font-semibold text-gray-900 mb-1">文件已上传</p>
                      <p class="text-sm text-gray-600 bg-white/50 px-3 py-1 rounded-lg inline-block">{{ media.filename }}</p>
                    </div>
                  </div>
                  <div class="flex space-x-3">
                    <a
                      :href="mediaAPI.getMediaDownloadUrl(media)"
                      target="_blank"
                      class="bg-gradient-to-r from-green-500 to-emerald-500 hover:from-green-600 hover:to-emerald-600 text-white px-6 py-3 rounded-xl text-sm font-medium transition-all duration-200 shadow-lg hover:shadow-xl inline-flex items-center space-x-2"
                    >
                      <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                      </svg>
                      <span>下载</span>
                    </a>
                    <label class="cursor-pointer bg-gradient-to-r from-blue-500 to-blue-600 hover:from-blue-600 hover:to-blue-700 text-white px-6 py-3 rounded-xl text-sm font-medium transition-all duration-200 shadow-lg hover:shadow-xl inline-flex items-center space-x-2">
                      <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"></path>
                      </svg>
                      <span>替换</span>
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
            </div>

            <!-- 上传进度 -->
            <div v-if="uploadProgress > 0 && uploadProgress < 100" class="mt-6">
              <div class="bg-white/70 backdrop-blur-sm rounded-2xl p-6 border border-blue-200">
                <div class="flex justify-between items-center text-sm text-gray-700 mb-3">
                  <div class="flex items-center space-x-2">
                    <div class="h-6 w-6 bg-gradient-to-r from-blue-500 to-blue-600 rounded-lg flex items-center justify-center">
                      <svg class="h-3 w-3 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"></path>
                      </svg>
                    </div>
                    <span class="font-medium">上传进度</span>
                  </div>
                  <span class="font-semibold text-blue-600">{{ uploadProgress }}%</span>
                </div>
                <div class="w-full bg-gray-200 rounded-full h-3 overflow-hidden">
                  <div
                    class="bg-gradient-to-r from-blue-500 to-blue-600 h-3 rounded-full transition-all duration-500 ease-out"
                    :style="{ width: uploadProgress + '%' }"
                  ></div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </main>

    <!-- 编辑模态框 -->
    <div v-if="showEditModal" class="fixed inset-0 bg-black/50 backdrop-blur-sm overflow-y-auto h-full w-full z-[60] flex items-center justify-center p-4">
      <div class="bg-white/90 backdrop-blur-sm shadow-2xl rounded-3xl border border-white/20 w-full max-w-md overflow-hidden">
        <div class="bg-gradient-to-r from-primary-50 to-secondary-50 px-8 py-6 border-b border-gray-100">
          <div class="flex items-center justify-center">
            <div class="h-10 w-10 bg-gradient-to-r from-primary-500 to-secondary-500 rounded-2xl flex items-center justify-center mr-3">
              <svg class="h-5 w-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
              </svg>
            </div>
            <h3 class="text-xl font-semibold text-gray-900">编辑媒体信息</h3>
          </div>
        </div>
        <div class="px-8 py-6">
          <form @submit.prevent="handleUpdate" class="space-y-6">
            <div>
              <label class="block text-sm font-semibold text-gray-700 mb-2">标题</label>
              <input
                v-model="editForm.title"
                type="text"
                required
                class="block w-full px-4 py-3 border border-gray-200 rounded-xl shadow-sm focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-all duration-200 bg-gray-50 focus:bg-white"
                placeholder="输入媒体标题"
              />
            </div>
            <div>
              <label class="block text-sm font-semibold text-gray-700 mb-2">描述</label>
              <textarea
                v-model="editForm.description"
                rows="4"
                class="block w-full px-4 py-3 border border-gray-200 rounded-xl shadow-sm focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-all duration-200 bg-gray-50 focus:bg-white resize-none"
                placeholder="输入媒体描述（可选）"
              ></textarea>
            </div>
            <div class="flex justify-end space-x-4 pt-6">
              <button
                type="button"
                @click="showEditModal = false"
                class="px-6 py-3 border border-gray-200 rounded-xl text-sm font-medium text-gray-700 hover:bg-gray-50 transition-all duration-200"
              >
                取消
              </button>
              <button
                type="submit"
                :disabled="updateLoading"
                class="px-8 py-3 bg-gradient-to-r from-primary-500 to-secondary-500 hover:from-primary-600 hover:to-secondary-600 text-white rounded-xl text-sm font-medium transition-all duration-200 shadow-lg hover:shadow-xl disabled:opacity-50 disabled:cursor-not-allowed flex items-center space-x-2"
              >
                <svg v-if="updateLoading" class="animate-spin h-4 w-4" fill="none" viewBox="0 0 24 24">
                  <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                  <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                <span>{{ updateLoading ? '更新中...' : '更新' }}</span>
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { mediaAPI, type Media } from '../api';
import AppNavbar from '../components/AppNavbar.vue';

const router = useRouter();
const route = useRoute();

// 状态管理
const media = ref<Media | null>(null);
const loading = ref(false);
const error = ref('');
const showEditModal = ref(false);
const updateLoading = ref(false);
const uploadProgress = ref(0);

// 编辑表单
const editForm = ref({
    title: '',
    description: '',
});

// 获取媒体类型标签
const getMediaTypeLabel = (type: string) => {
    const labels = {
        video: '视频',
        audio: '音频',
        image: '图片',
    };
    return labels[type as keyof typeof labels] || type;
};

// 格式化文件大小
const formatFileSize = (bytes: number) => {
    if (bytes === 0) { return '0 Bytes'; }
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

// 格式化日期
const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleString('zh-CN');
};

// 获取接受的文件类型
const getAcceptedFileTypes = (mediaType: string) => {
    const types = {
        video: 'video/*',
        audio: 'audio/*',
        image: 'image/*',
    };
    return types[mediaType as keyof typeof types] || '*/*';
};

// 加载媒体详情
const loadMediaDetail = async () => {
    const id = route.params.id as string;
    if (!id) {
        error.value = '无效的媒体 ID';
        return;
    }

    loading.value = true;
    error.value = '';
  
    try {
        media.value = await mediaAPI.getMediaById(id);
        // 初始化编辑表单
        editForm.value = {
            title: media.value.title,
            description: media.value.description || '',
        };
    } catch (err: any) {
        console.error('加载媒体详情失败:', err);
        if (err.response?.status === 404) {
            error.value = '媒体不存在';
        } else {
            error.value = '加载媒体详情失败，请稍后重试';
        }
    } finally {
        loading.value = false;
    }
};

// 处理文件选择
const handleFileSelect = async (event: Event) => {
    const target = event.target as HTMLInputElement;
    const file = target.files?.[0];
    if (!file || !media.value) { return; }

    uploadProgress.value = 0;
  
    try {
        const updatedMedia = await mediaAPI.uploadMediaFile(
            media.value.id,
            file,
            (progress) => {
                uploadProgress.value = progress;
            }
        );
    
        media.value = updatedMedia;
        uploadProgress.value = 100;
    
        // 重置进度条
        setTimeout(() => {
            uploadProgress.value = 0;
        }, 2000);
    } catch (err: any) {
        console.error('文件上传失败:', err);
        alert('文件上传失败，请稍后重试');
        uploadProgress.value = 0;
    }
  
    // 清除文件输入
    target.value = '';
};

// 处理更新
const handleUpdate = async () => {
    if (!media.value) { return; }
  
    updateLoading.value = true;
  
    try {
        const updatedMedia = await mediaAPI.updateMedia(media.value.id, {
            title: editForm.value.title,
            description: editForm.value.description,
        });
    
        media.value = updatedMedia;
        showEditModal.value = false;
    } catch (err: any) {
        console.error('更新媒体失败:', err);
        alert('更新媒体失败，请稍后重试');
    } finally {
        updateLoading.value = false;
    }
};

// 处理删除
const handleDelete = async () => {
    if (!media.value) { return; }
  
    if (!confirm(`确定要删除 "${media.value.title}" 吗？`)) {
        return;
    }
  
    try {
        await mediaAPI.deleteMedia(media.value.id);
        router.push('/media');
    } catch (err: any) {
        console.error('删除媒体失败:', err);
        alert('删除媒体失败，请稍后重试');
    }
};



// 组件挂载时加载数据
onMounted(() => {
    loadMediaDetail();
});
</script>