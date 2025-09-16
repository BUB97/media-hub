import axios from 'axios';

// 创建 axios 实例
const apiClient = axios.create({
  baseURL: '/api', // 使用相对路径，通过 nginx 代理
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
  },
  withCredentials: true, // 确保 Cookie 会被自动发送
});

// 请求拦截器 - Cookie 会自动发送，无需手动添加 token
apiClient.interceptors.request.use(
  (config) => {
    // Cookie 认证不需要手动添加 Authorization header
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

// 响应拦截器 - 处理错误
apiClient.interceptors.response.use(
  (response) => {
    return response;
  },
  (error) => {
    if (error.response?.status === 401) {
      // Cookie 过期或无效，清除本地存储
      localStorage.removeItem('user_info');
      localStorage.removeItem('token_expires_at');
      // Cookie 会由服务器端清除
      // 路由到登录页面
      getApp().goTo({ path: '/login', query: { a: 1 } });
    }
    return Promise.reject(error);
  }
);

export default apiClient;
