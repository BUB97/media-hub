<template>
  <div
    class="min-h-screen flex items-center justify-center bg-gradient-to-br from-primary-50 to-secondary-50 py-6 sm:py-12 px-4 sm:px-6 lg:px-8"
  >
    <div class="max-w-md w-full mobile-padding">
      <!-- 注册卡片 -->
      <div
        class="bg-white/80 backdrop-blur-sm rounded-2xl shadow-xl border border-white/20 p-6 sm:p-8 space-y-6 sm:space-y-8 animate-scale-in mobile-card"
      >
        <!-- 头部 -->
        <div class="text-center">
          <div
            class="mx-auto h-16 w-16 bg-gradient-to-r from-primary-500 to-secondary-500 rounded-2xl flex items-center justify-center mb-4 animate-pulse"
          >
            <svg class="h-8 w-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"
              ></path>
            </svg>
          </div>
          <h2
            class="text-3xl font-bold bg-gradient-to-r from-primary-600 to-secondary-600 bg-clip-text text-transparent animate-fade-in"
            style="animation-delay: 0.2s; animation-fill-mode: both"
          >
            加入我们
          </h2>
          <p
            class="mt-2 text-gray-600 animate-slide-in-left"
            style="animation-delay: 0.3s; animation-fill-mode: both"
          >
            创建您的 Media Hub 账户
          </p>
          <p class="mt-2 text-center text-sm text-gray-600">
            已有账户？
            <router-link
              to="/login"
              class="font-medium text-primary-600 hover:text-primary-500 transition-colors"
            >
              立即登录
            </router-link>
          </p>
        </div>

        <!-- 注册表单 -->
        <form class="space-y-6" @submit.prevent="handleRegister">
          <div class="space-y-4">
            <!-- 用户名输入 -->
            <div
              class="group animate-slide-in-left"
              style="animation-delay: 0.4s; animation-fill-mode: both"
            >
              <label for="username" class="block text-sm font-medium text-gray-700 mb-2"
                >用户名</label
              >
              <div class="relative">
                <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                  <svg
                    class="h-5 w-5 text-gray-400 group-focus-within:text-primary-500 transition-colors"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                    ></path>
                  </svg>
                </div>
                <input
                  id="username"
                  v-model="form.username"
                  name="username"
                  type="text"
                  required
                  class="block w-full pl-10 pr-3 py-3 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all duration-200 bg-gray-50 focus:bg-white"
                  placeholder="请输入用户名"
                />
              </div>
            </div>

            <!-- 邮箱输入 -->
            <div
              class="group animate-slide-in-right"
              style="animation-delay: 0.5s; animation-fill-mode: both"
            >
              <label for="email" class="block text-sm font-medium text-gray-700 mb-2"
                >邮箱地址</label
              >
              <div class="relative">
                <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                  <svg
                    class="h-5 w-5 text-gray-400 group-focus-within:text-primary-500 transition-colors"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207"
                    ></path>
                  </svg>
                </div>
                <input
                  id="email"
                  v-model="form.email"
                  name="email"
                  type="email"
                  required
                  class="block w-full pl-10 pr-3 py-3 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all duration-200 bg-gray-50 focus:bg-white"
                  placeholder="请输入邮箱地址"
                />
              </div>
            </div>

            <!-- 密码输入 -->
            <div
              class="group animate-slide-in-left"
              style="animation-delay: 0.6s; animation-fill-mode: both"
            >
              <label for="password" class="block text-sm font-medium text-gray-700 mb-2"
                >密码</label
              >
              <div class="relative">
                <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                  <svg
                    class="h-5 w-5 text-gray-400 group-focus-within:text-primary-500 transition-colors"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
                    ></path>
                  </svg>
                </div>
                <input
                  id="password"
                  v-model="form.password"
                  name="password"
                  :type="showPassword ? 'text' : 'password'"
                  required
                  class="block w-full pl-10 pr-12 py-3 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all duration-200 bg-gray-50 focus:bg-white"
                  placeholder="请输入密码（至少6位）"
                />
                <button
                  type="button"
                  @click="showPassword = !showPassword"
                  class="absolute inset-y-0 right-0 pr-3 flex items-center text-gray-400 hover:text-gray-600 transition-colors"
                >
                  <svg
                    v-if="!showPassword"
                    class="h-5 w-5"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                    ></path>
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
                    ></path>
                  </svg>
                  <svg v-else class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.878 9.878L3 3m6.878 6.878L21 21"
                    ></path>
                  </svg>
                </button>
              </div>
            </div>

            <!-- 确认密码输入 -->
            <div
              class="group animate-slide-in-right"
              style="animation-delay: 0.7s; animation-fill-mode: both"
            >
              <label for="confirmPassword" class="block text-sm font-medium text-gray-700 mb-2"
                >确认密码</label
              >
              <div class="relative">
                <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                  <svg
                    class="h-5 w-5 text-gray-400 group-focus-within:text-primary-500 transition-colors"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                    ></path>
                  </svg>
                </div>
                <input
                  id="confirmPassword"
                  v-model="form.confirmPassword"
                  name="confirmPassword"
                  :type="showConfirmPassword ? 'text' : 'password'"
                  required
                  class="block w-full pl-10 pr-12 py-3 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all duration-200 bg-gray-50 focus:bg-white"
                  placeholder="请再次输入密码"
                />
                <button
                  type="button"
                  @click="showConfirmPassword = !showConfirmPassword"
                  class="absolute inset-y-0 right-0 pr-3 flex items-center text-gray-400 hover:text-gray-600 transition-colors"
                >
                  <svg
                    v-if="!showConfirmPassword"
                    class="h-5 w-5"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                    ></path>
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
                    ></path>
                  </svg>
                  <svg v-else class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.878 9.878L3 3m6.878 6.878L21 21"
                    ></path>
                  </svg>
                </button>
              </div>
            </div>
          </div>

          <!-- 错误和成功消息 -->
          <div v-if="error" class="bg-red-50 border border-red-200 rounded-xl p-4">
            <div class="flex">
              <div class="flex-shrink-0">
                <svg
                  class="h-5 w-5 text-red-400"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                  ></path>
                </svg>
              </div>
              <div class="ml-3">
                <p class="text-sm text-red-800">{{ error }}</p>
              </div>
            </div>
          </div>

          <div v-if="success" class="bg-green-50 border border-green-200 rounded-xl p-4">
            <div class="flex">
              <div class="flex-shrink-0">
                <svg
                  class="h-5 w-5 text-green-400"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                  ></path>
                </svg>
              </div>
              <div class="ml-3">
                <p class="text-sm text-green-800">{{ success }}</p>
              </div>
            </div>
          </div>

          <!-- 注册按钮 -->
          <button
            type="submit"
            :disabled="loading"
            class="group relative w-full flex justify-center py-3 px-4 border border-transparent text-sm font-medium rounded-xl text-white btn btn-primary focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200 shadow-lg hover:shadow-xl animate-fade-in"
            style="animation-delay: 0.8s; animation-fill-mode: both"
          >
            <span v-if="loading" class="absolute left-0 inset-y-0 flex items-center pl-3">
              <svg
                class="animate-spin h-5 w-5 text-white"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
              >
                <circle
                  class="opacity-25"
                  cx="12"
                  cy="12"
                  r="10"
                  stroke="currentColor"
                  stroke-width="4"
                ></circle>
                <path
                  class="opacity-75"
                  fill="currentColor"
                  d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                ></path>
              </svg>
            </span>
            {{ loading ? '注册中...' : '创建账户' }}
          </button>

          <!-- 返回首页链接 -->
          <div class="text-center">
            <router-link
              to="/"
              class="text-sm text-gray-600 hover:text-primary-600 transition-colors"
            >
              返回首页
            </router-link>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue';
  import { authAPI } from '../api';

  // 表单数据
  const form = ref({
    username: '',
    email: '',
    password: '',
    confirmPassword: '',
  });

  // 状态管理
  const loading = ref(false);
  const error = ref('');
  const success = ref('');
  const showPassword = ref(false);
  const showConfirmPassword = ref(false);

  // 表单验证
  const validateForm = () => {
    if (
      !form.value.username ||
      !form.value.email ||
      !form.value.password ||
      !form.value.confirmPassword
    ) {
      error.value = '请填写所有字段';
      return false;
    }

    if (form.value.username.length < 3) {
      error.value = '用户名至少需要3个字符';
      return false;
    }

    if (form.value.password.length < 6) {
      error.value = '密码至少需要6个字符';
      return false;
    }

    if (form.value.password !== form.value.confirmPassword) {
      error.value = '两次输入的密码不一致';
      return false;
    }

    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    if (!emailRegex.test(form.value.email)) {
      error.value = '请输入有效的邮箱地址';
      return false;
    }

    return true;
  };

  // 处理注册
  const handleRegister = async () => {
    if (!validateForm()) {
      return;
    }

    loading.value = true;
    error.value = '';
    success.value = '';

    try {
      await authAPI.register({
        username: form.value.username,
        email: form.value.email,
        password: form.value.password,
      });

      // 保存认证信息
      // authUtils.saveAuthData(response)

      success.value = '注册成功！正在跳转...';

      // 延迟跳转以显示成功消息
      setTimeout(() => {
        getApp().goTo('/login');
      }, 1500);
    } catch (err: any) {
      console.error('注册失败:', err);
      if (err.response?.status === 409) {
        error.value = '用户名或邮箱已存在';
      } else if (err.response?.data?.message) {
        error.value = err.response.data.message;
      } else {
        error.value = '注册失败，请稍后重试';
      }
    } finally {
      loading.value = false;
    }
  };
</script>
