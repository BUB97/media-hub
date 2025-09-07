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
      <div class="px-4 py-6 sm:px-0">
        <h1 class="text-2xl font-bold text-gray-900 mb-6">个人资料</h1>

        <div class="grid grid-cols-1 gap-6 lg:grid-cols-2">
          <!-- 用户信息卡片 -->
          <div class="bg-white shadow rounded-lg">
            <div class="px-6 py-4 border-b border-gray-200">
              <h2 class="text-lg font-medium text-gray-900">基本信息</h2>
            </div>
            <div class="px-6 py-4">
              <div v-if="loading" class="text-center py-4">
                <div class="inline-block animate-spin rounded-full h-6 w-6 border-b-2 border-blue-600"></div>
                <p class="mt-2 text-sm text-gray-600">加载中...</p>
              </div>
              
              <div v-else-if="error" class="text-center py-4">
                <p class="text-red-600 text-sm">{{ error }}</p>
                <button
                  @click="loadUserInfo"
                  class="mt-2 bg-blue-600 text-white px-3 py-1 rounded text-sm hover:bg-blue-700"
                >
                  重试
                </button>
              </div>
              
              <div v-else-if="user" class="space-y-4">
                <div class="flex items-center space-x-4">
                  <div class="flex-shrink-0">
                    <div class="h-16 w-16 bg-blue-100 rounded-full flex items-center justify-center">
                      <svg class="h-8 w-8 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
                      </svg>
                    </div>
                  </div>
                  <div>
                    <h3 class="text-lg font-medium text-gray-900">{{ user.username }}</h3>
                    <p class="text-sm text-gray-500">{{ user.email }}</p>
                  </div>
                </div>
                
                <dl class="grid grid-cols-1 gap-4 sm:grid-cols-2">
                  <div>
                    <dt class="text-sm font-medium text-gray-500">用户 ID</dt>
                    <dd class="mt-1 text-sm text-gray-900 font-mono">{{ user.id }}</dd>
                  </div>
                  <div>
                    <dt class="text-sm font-medium text-gray-500">注册时间</dt>
                    <dd class="mt-1 text-sm text-gray-900">{{ formatDate(user.created_at) }}</dd>
                  </div>
                  <div>
                    <dt class="text-sm font-medium text-gray-500">最后登录</dt>
                    <dd class="mt-1 text-sm text-gray-900">{{ formatDate(user.last_login) }}</dd>
                  </div>
                </dl>
              </div>
            </div>
          </div>

          <!-- 账户设置卡片 -->
          <div class="bg-white shadow rounded-lg">
            <div class="px-6 py-4 border-b border-gray-200">
              <h2 class="text-lg font-medium text-gray-900">账户设置</h2>
            </div>
            <div class="px-6 py-4 space-y-4">
              <div class="border border-gray-200 rounded-lg p-4">
                <div class="flex items-center justify-between">
                  <div>
                    <h3 class="text-sm font-medium text-gray-900">修改密码</h3>
                    <p class="text-sm text-gray-500">更新您的账户密码</p>
                  </div>
                  <button
                    @click="showPasswordModal = true"
                    class="bg-blue-600 text-white px-3 py-1 rounded text-sm hover:bg-blue-700"
                  >
                    修改
                  </button>
                </div>
              </div>
              
              <div class="border border-gray-200 rounded-lg p-4">
                <div class="flex items-center justify-between">
                  <div>
                    <h3 class="text-sm font-medium text-gray-900">清除数据</h3>
                    <p class="text-sm text-gray-500">清除本地存储的认证信息</p>
                  </div>
                  <button
                    @click="clearLocalData"
                    class="bg-yellow-600 text-white px-3 py-1 rounded text-sm hover:bg-yellow-700"
                  >
                    清除
                  </button>
                </div>
              </div>
              
              <div class="border border-red-200 rounded-lg p-4">
                <div class="flex items-center justify-between">
                  <div>
                    <h3 class="text-sm font-medium text-red-900">删除账户</h3>
                    <p class="text-sm text-red-600">永久删除您的账户和所有数据</p>
                  </div>
                  <button
                    @click="showDeleteModal = true"
                    class="bg-red-600 text-white px-3 py-1 rounded text-sm hover:bg-red-700"
                  >
                    删除
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- 使用统计 -->
        <div class="mt-6 bg-white shadow rounded-lg">
          <div class="px-6 py-4 border-b border-gray-200">
            <h2 class="text-lg font-medium text-gray-900">使用统计</h2>
          </div>
          <div class="px-6 py-4">
            <div class="grid grid-cols-1 gap-4 sm:grid-cols-3">
              <div class="text-center">
                <div class="text-2xl font-bold text-blue-600">{{ stats.totalMedia }}</div>
                <div class="text-sm text-gray-500">总媒体数</div>
              </div>
              <div class="text-center">
                <div class="text-2xl font-bold text-green-600">{{ formatFileSize(stats.totalSize) }}</div>
                <div class="text-sm text-gray-500">总存储空间</div>
              </div>
              <div class="text-center">
                <div class="text-2xl font-bold text-purple-600">{{ stats.daysActive }}</div>
                <div class="text-sm text-gray-500">活跃天数</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </main>

    <!-- 修改密码模态框 -->
    <div v-if="showPasswordModal" class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
      <div class="relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white">
        <div class="mt-3">
          <h3 class="text-lg font-medium text-gray-900 text-center">修改密码</h3>
          <form @submit.prevent="handlePasswordChange" class="mt-4 space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700">当前密码</label>
              <input
                v-model="passwordForm.currentPassword"
                type="password"
                required
                class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700">新密码</label>
              <input
                v-model="passwordForm.newPassword"
                type="password"
                required
                minlength="6"
                class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700">确认新密码</label>
              <input
                v-model="passwordForm.confirmPassword"
                type="password"
                required
                class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
              />
            </div>
            <div v-if="passwordError" class="text-red-600 text-sm">
              {{ passwordError }}
            </div>
            <div class="flex justify-end space-x-3 pt-4">
              <button
                type="button"
                @click="showPasswordModal = false"
                class="px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50"
              >
                取消
              </button>
              <button
                type="submit"
                :disabled="passwordLoading"
                class="px-4 py-2 bg-blue-600 text-white rounded-md text-sm font-medium hover:bg-blue-700 disabled:opacity-50"
              >
                {{ passwordLoading ? '修改中...' : '修改密码' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>

    <!-- 删除账户确认模态框 -->
    <div v-if="showDeleteModal" class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
      <div class="relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white">
        <div class="mt-3 text-center">
          <svg class="mx-auto h-12 w-12 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.082 16.5c-.77.833.192 2.5 1.732 2.5z"></path>
          </svg>
          <h3 class="text-lg font-medium text-gray-900 mt-2">删除账户</h3>
          <div class="mt-2 px-7 py-3">
            <p class="text-sm text-gray-500">
              您确定要删除账户吗？此操作将永久删除您的所有数据，包括所有媒体文件，且无法恢复。
            </p>
            <div class="mt-4">
              <input
                v-model="deleteConfirmText"
                type="text"
                placeholder="请输入 'DELETE' 确认删除"
                class="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-red-500 focus:border-red-500 sm:text-sm"
              />
            </div>
          </div>
          <div class="items-center px-4 py-3">
            <div class="flex space-x-3">
              <button
                @click="showDeleteModal = false"
                class="flex-1 px-4 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-50"
              >
                取消
              </button>
              <button
                @click="handleDeleteAccount"
                :disabled="deleteConfirmText !== 'DELETE' || deleteLoading"
                class="flex-1 px-4 py-2 bg-red-600 text-white rounded-md text-sm font-medium hover:bg-red-700 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {{ deleteLoading ? '删除中...' : '确认删除' }}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { authUtils, authAPI, mediaAPI, type User } from '../api'

const router = useRouter()

// 状态管理
const user = ref<User | null>(null)
const loading = ref(false)
const error = ref('')
const showPasswordModal = ref(false)
const showDeleteModal = ref(false)
const passwordLoading = ref(false)
const deleteLoading = ref(false)
const passwordError = ref('')
const deleteConfirmText = ref('')

// 统计数据
const stats = ref({
  totalMedia: 0,
  totalSize: 0,
  daysActive: 0,
})

// 密码修改表单
const passwordForm = ref({
  currentPassword: '',
  newPassword: '',
  confirmPassword: '',
})

// 格式化日期
const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleString('zh-CN')
}

// 格式化文件大小
const formatFileSize = (bytes: number) => {
  if (bytes === 0) return '0 Bytes'
  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

// 加载用户信息
const loadUserInfo = async () => {
  loading.value = true
  error.value = ''
  
  try {
    user.value = await authAPI.getCurrentUser()
    // 同时更新本地存储
    if (user.value) {
      localStorage.setItem('user_info', JSON.stringify(user.value))
    }
  } catch (err: any) {
    console.error('加载用户信息失败:', err)
    error.value = '加载用户信息失败，请稍后重试'
  } finally {
    loading.value = false
  }
}

// 加载统计数据
const loadStats = async () => {
  try {
    const mediaList = await mediaAPI.getMediaList()
    stats.value.totalMedia = mediaList.length
    stats.value.totalSize = mediaList.reduce((total, media) => total + (media.file_size || 0), 0)
    
    // 计算活跃天数（从注册到现在）
    if (user.value) {
      const createdAt = new Date(user.value.created_at)
      const now = new Date()
      const diffTime = Math.abs(now.getTime() - createdAt.getTime())
      stats.value.daysActive = Math.ceil(diffTime / (1000 * 60 * 60 * 24))
    }
  } catch (error) {
    console.error('加载统计数据失败:', error)
  }
}

// 修改密码
const handlePasswordChange = async () => {
  passwordError.value = ''
  
  if (passwordForm.value.newPassword !== passwordForm.value.confirmPassword) {
    passwordError.value = '两次输入的新密码不一致'
    return
  }
  
  if (passwordForm.value.newPassword.length < 6) {
    passwordError.value = '新密码至少需要6个字符'
    return
  }
  
  passwordLoading.value = true
  
  try {
    // 这里应该调用修改密码的 API，但后端还没有实现
    // await authAPI.changePassword(passwordForm.value)
    
    // 模拟 API 调用
    await new Promise(resolve => setTimeout(resolve, 1000))
    
    alert('密码修改成功！请重新登录。')
    showPasswordModal.value = false
    
    // 清除认证信息并跳转到登录页
    authUtils.clearAuthData()
    router.push('/login')
  } catch (err: any) {
    console.error('修改密码失败:', err)
    passwordError.value = '修改密码失败，请稍后重试'
  } finally {
    passwordLoading.value = false
  }
}

// 清除本地数据
const clearLocalData = () => {
  if (confirm('确定要清除本地存储的认证信息吗？这将需要您重新登录。')) {
    authUtils.clearAuthData()
    router.push('/login')
  }
}

// 删除账户
const handleDeleteAccount = async () => {
  if (deleteConfirmText.value !== 'DELETE') {
    return
  }
  
  deleteLoading.value = true
  
  try {
    // 这里应该调用删除账户的 API，但后端还没有实现
    // await authAPI.deleteAccount()
    
    // 模拟 API 调用
    await new Promise(resolve => setTimeout(resolve, 2000))
    
    alert('账户已删除')
    authUtils.clearAuthData()
    router.push('/')
  } catch (err: any) {
    console.error('删除账户失败:', err)
    alert('删除账户失败，请稍后重试')
  } finally {
    deleteLoading.value = false
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
onMounted(async () => {
  // 先从本地存储获取用户信息
  user.value = authUtils.getStoredUser()
  
  // 然后从服务器获取最新信息
  await loadUserInfo()
  await loadStats()
})
</script>