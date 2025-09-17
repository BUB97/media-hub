<template>
  <div class="upload-container">
    <div class="upload-header">
      <h1>📁 媒体文件上传</h1>
      <p>支持图片、视频和音频文件上传到腾讯云COS</p>
    </div>

    <div class="upload-section">
      <!-- 文件选择区域 -->
      <div
        class="file-selector"
        :class="{ 'drag-over': isDragOver }"
        @drop="handleDrop"
        @dragover="handleDragOver"
        @dragleave="handleDragLeave"
      >
        <div v-if="!selectedFile" class="drop-zone">
          <div class="upload-icon">📤</div>
          <p>拖拽文件到此处或点击选择文件</p>
          <input
            type="file"
            ref="fileInput"
            @change="handleFileSelect"
            accept="image/*,video/*,audio/*"
            class="file-input"
          />
          <button @click="$refs.fileInput.click()" class="select-btn">选择文件</button>
        </div>

        <div v-else class="file-preview">
          <div class="file-info">
            <div class="file-icon">{{ getFileIcon(selectedFile.type) }}</div>
            <div class="file-details">
              <h3>{{ selectedFile.name }}</h3>
              <p>大小: {{ formatFileSize(selectedFile.size) }}</p>
              <p>类型: {{ selectedFile.type }}</p>
            </div>
          </div>
          <button @click="clearFile" class="clear-btn">✕</button>
        </div>
      </div>

      <!-- 上传进度 -->
      <div v-if="uploadProgress > 0" class="progress-section">
        <div class="progress-bar">
          <div class="progress-fill" :style="{ width: uploadProgress + '%' }"></div>
        </div>
        <p class="progress-text">{{ uploadProgress }}% - {{ uploadStatus }}</p>
      </div>

      <!-- 操作按钮 -->
      <div class="action-buttons">
        <button
          @click="validateFile"
          :disabled="!selectedFile || isUploading"
          class="btn btn-secondary"
        >
          验证文件
        </button>
        <button
          @click="uploadFile"
          :disabled="!selectedFile || isUploading"
          class="btn btn-primary"
        >
          {{ isUploading ? '上传中...' : '开始上传' }}
        </button>
      </div>

      <!-- 状态信息 -->
      <div v-if="statusMessage" class="status-message" :class="statusType">
        {{ statusMessage }}
      </div>

      <!-- COS配置信息 -->
      <div class="cos-config" v-if="cosConfig">
        <h3>📋 COS配置信息</h3>
        <div class="config-grid">
          <div class="config-item">
            <label>区域:</label>
            <span>{{ cosConfig.region }}</span>
          </div>
          <div class="config-item">
            <label>上传前缀:</label>
            <span>{{ cosConfig.upload_prefix }}</span>
          </div>
          <div class="config-item">
            <label>最大文件大小:</label>
            <span>{{ formatFileSize(parseInt(cosConfig.max_file_size)) }}</span>
          </div>
          <div class="config-item">
            <label>支持类型:</label>
            <span>{{ cosConfig.allowed_types }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
  import { ref, onMounted, onUnmounted } from 'vue';
  import apiClient from '@/api/client';
  import cosUploader from '@/utils/cosUploader';

  // 响应式数据
  const selectedFile = ref(null);
  const isDragOver = ref(false);
  const uploadProgress = ref(0);
  const uploadStatus = ref('');
  const isUploading = ref(false);
  const statusMessage = ref('');
  const statusType = ref('info');
  const cosConfig = ref(null);
  const fileInput = ref(null);

  // 获取COS配置
  const fetchCosConfig = async () => {
    try {
      const response = await apiClient.get('/cos/config');
      cosConfig.value = response.data;
    } catch (error) {
      console.error('获取COS配置失败:', error);
      showStatus('获取COS配置失败', 'error');
    }
  };

  // 文件选择处理
  const handleFileSelect = (event) => {
    const file = event.target.files[0];
    if (file) {
      selectedFile.value = file;
      uploadProgress.value = 0;
      statusMessage.value = '';
    }
  };

  // 拖拽处理
  const handleDrop = (event) => {
    event.preventDefault();
    isDragOver.value = false;
    const files = event.dataTransfer.files;
    if (files.length > 0) {
      selectedFile.value = files[0];
      uploadProgress.value = 0;
      statusMessage.value = '';
    }
  };

  const handleDragOver = (event) => {
    event.preventDefault();
    isDragOver.value = true;
  };

  const handleDragLeave = () => {
    isDragOver.value = false;
  };

  // 清除文件
  const clearFile = () => {
    selectedFile.value = null;
    uploadProgress.value = 0;
    statusMessage.value = '';
    if (fileInput.value) {
      fileInput.value.value = '';
    }
  };

  // 验证文件
  const validateFile = async () => {
    if (!selectedFile.value) { return; }

    try {
      const response = await apiClient.post('/cos/validate', {
        filename: selectedFile.value.name,
        file_size: selectedFile.value.size, 
        content_type: selectedFile.value.type,
      });

      if (response.data.valid) {
        showStatus(`✅ 文件验证通过！建议存储路径: ${response.data.suggested_key}`, 'success');
      } else {
        showStatus(`❌ 文件验证失败: ${response.data.message}`, 'error');
      }
    } catch (error) {
      console.error('文件验证失败:', error);
      showStatus('文件验证请求失败', 'error');
    }
  };

  // 上传文件
  const uploadFile = async () => {
    if (!selectedFile.value) { return; }

    isUploading.value = true;
    uploadProgress.value = 0;
    uploadStatus.value = '准备上传...';

    try {
      // 1. 验证文件
      uploadStatus.value = '验证文件...';
      const validateResponse = await apiClient.post('/cos/validate', {
        filename: selectedFile.value.name,
        file_size: selectedFile.value.size,
        content_type: selectedFile.value.type,
      });

      if (!validateResponse.data.valid) {
        throw new Error(validateResponse.data.message);
      }

      uploadProgress.value = 10;

      // 2. 获取STS临时凭证
      uploadStatus.value = '获取上传凭证...';
      const stsResponse = await apiClient.get('/cos/sts?duration_seconds=3600');

      if (stsResponse.data.error) {
        throw new Error(stsResponse.data.message || '获取STS凭证失败');
      }

      const credentials = stsResponse.data.credentials;
      uploadProgress.value = 20;
      console.log('credentials', credentials);

      // 3. 初始化COS上传器
      uploadStatus.value = '初始化上传器...';
      cosUploader.init(
        {
          tmpSecretId: credentials.tmp_secret_id,
          tmpSecretKey: credentials.tmp_secret_key,
          sessionToken: credentials.session_token,
        },
        cosConfig.value?.region || 'ap-beijing'
      );

      uploadProgress.value = 30;

      // 4. 使用COS SDK上传文件
      uploadStatus.value = '上传文件...';

      const uploadResult = await cosUploader.uploadFile({
        file: selectedFile.value,
        bucket: cosConfig.value?.bucket || 'your-bucket-name',
        key: validateResponse.data.suggested_key,
        region: cosConfig.value?.region || 'ap-beijing',
        onProgress: (progressData) => {
          // 上传进度从30%开始到95%
          const adjustedPercent = 30 + progressData.percent * 0.65;
          uploadProgress.value = Math.round(adjustedPercent);
          uploadStatus.value = `上传中... ${Math.round(progressData.percent)}% (${formatFileSize(progressData.loaded)}/${formatFileSize(progressData.total)})`;

          if (progressData.speed) {
            uploadStatus.value += ` - ${formatFileSize(progressData.speed)}/s`;
          }
        },
      });

      // 5. 保存媒体信息到后台数据库
      uploadStatus.value = '保存媒体信息...';
      
      // 确定媒体类型
      let mediaType = 'document';
      if (selectedFile.value.type.startsWith('image/')) {
        mediaType = 'image';
      } else if (selectedFile.value.type.startsWith('video/')) {
        mediaType = 'video';
      } else if (selectedFile.value.type.startsWith('audio/')) {
        mediaType = 'audio';
      }

      // 创建媒体记录
      const mediaData = {
        title: selectedFile.value.name.replace(/\.[^/.]+$/, ''), // 移除文件扩展名作为标题
        description: `通过上传页面上传的${mediaType}文件`,
        filename: (uploadResult.key || validateResponse.data.suggested_key || selectedFile.value.name).split('/').pop() || selectedFile.value.name,
        original_filename: selectedFile.value.name,
        file_size: selectedFile.value.size,
        content_type: selectedFile.value.type,
        cos_key: uploadResult.key || validateResponse.data.suggested_key,
        cos_url: uploadResult.url,
        cos_bucket: cosConfig.value?.bucket || 'your-bucket-name',
        cos_region: cosConfig.value?.region || 'ap-beijing',
        media_type: mediaType,
        metadata: {
          upload_method: 'direct_upload',
          upload_time: new Date().toISOString()
        }
      };

      const savedMedia = await apiClient.post('/media', mediaData);
      
      // 6. 完成上传
      uploadProgress.value = 100;
      uploadStatus.value = '上传完成！';

      showStatus(
        `🎉 文件上传成功！\n媒体ID: ${savedMedia.data.id}\n访问地址: ${uploadResult.url}\n存储路径: ${uploadResult.key}`,
        'success'
      );

      // 清理并跳转到媒体库
      setTimeout(() => {
        clearFile();
        isUploading.value = false;
        // 跳转到媒体库页面
        window.location.href = '/media';
      }, 3000);
    } catch (error) {
      console.error('上传失败:', error);
      let errorMessage = error.message;

      // 处理常见的COS错误
      if (error.code) {
        switch (error.code) {
          case 'NoSuchBucket':
            errorMessage = '存储桶不存在，请检查配置';
            break;
          case 'AccessDenied':
            errorMessage = '访问被拒绝，请检查权限配置';
            break;
          case 'InvalidAccessKeyId':
            errorMessage = '无效的访问密钥，请重新获取凭证';
            break;
          case 'RequestTimeTooSkewed':
            errorMessage = '请求时间偏差过大，请检查系统时间';
            break;
          default:
            errorMessage = `上传失败: ${error.code} - ${error.message}`;
        }
      }

      showStatus(`❌ ${errorMessage}`, 'error');
      isUploading.value = false;
      uploadProgress.value = 0;
    }
  };

  // 显示状态信息
  const showStatus = (message, type = 'info') => {
    statusMessage.value = message;
    statusType.value = type;

    // 自动清除成功消息
    if (type === 'success') {
      setTimeout(() => {
        statusMessage.value = '';
      }, 5000);
    }
  };

  // 工具函数
  const formatFileSize = (bytes) => {
    if (bytes === 0) { return '0 Bytes'; }
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  const getFileIcon = (type) => {
    if (type.startsWith('image/')) { return '🖼️'; }
    if (type.startsWith('video/')) { return '🎬'; }
    if (type.startsWith('audio/')) { return '🎵'; }
    return '📄';
  };

  // 组件挂载时获取配置
  onMounted(() => {
    fetchCosConfig();
  });

  // 组件卸载时清理资源
  onUnmounted(() => {
    // 销毁COS实例，取消正在进行的上传任务
    cosUploader.destroy();
  });
</script>

<style scoped>
  .upload-container {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
  }

  .upload-header {
    text-align: center;
    margin-bottom: 2rem;
  }

  .upload-header h1 {
    color: #2c3e50;
    margin-bottom: 0.5rem;
  }

  .upload-header p {
    color: #7f8c8d;
    font-size: 1.1rem;
  }

  .file-selector {
    border: 2px dashed #bdc3c7;
    border-radius: 12px;
    padding: 2rem;
    text-align: center;
    transition: all 0.3s ease;
    margin-bottom: 1.5rem;
  }

  .file-selector.drag-over {
    border-color: #3498db;
    background-color: #f8f9fa;
  }

  .drop-zone {
    position: relative;
  }

  .upload-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  .file-input {
    position: absolute;
    opacity: 0;
    width: 100%;
    height: 100%;
    cursor: pointer;
  }

  .select-btn {
    background: #3498db;
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    cursor: pointer;
    font-size: 1rem;
    margin-top: 1rem;
    transition: background-color 0.3s;
  }

  .select-btn:hover {
    background: #2980b9;
  }

  .file-preview {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: #f8f9fa;
    padding: 1rem;
    border-radius: 8px;
  }

  .file-info {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .file-icon {
    font-size: 2rem;
  }

  .file-details h3 {
    margin: 0 0 0.5rem 0;
    color: #2c3e50;
  }

  .file-details p {
    margin: 0.25rem 0;
    color: #7f8c8d;
    font-size: 0.9rem;
  }

  .clear-btn {
    background: #e74c3c;
    color: white;
    border: none;
    width: 2rem;
    height: 2rem;
    border-radius: 50%;
    cursor: pointer;
    font-size: 1rem;
  }

  .progress-section {
    margin-bottom: 1.5rem;
  }

  .progress-bar {
    width: 100%;
    height: 8px;
    background: #ecf0f1;
    border-radius: 4px;
    overflow: hidden;
    margin-bottom: 0.5rem;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #3498db, #2ecc71);
    transition: width 0.3s ease;
  }

  .progress-text {
    text-align: center;
    color: #7f8c8d;
    font-size: 0.9rem;
  }

  .action-buttons {
    display: flex;
    gap: 1rem;
    justify-content: center;
    margin-bottom: 1.5rem;
  }

  .btn {
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 1rem;
    transition: all 0.3s;
  }

  .btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .btn-primary {
    background: #2ecc71;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: #27ae60;
  }

  .btn-secondary {
    background: #95a5a6;
    color: white;
  }

  .btn-secondary:hover:not(:disabled) {
    background: #7f8c8d;
  }

  .status-message {
    padding: 1rem;
    border-radius: 6px;
    margin-bottom: 1.5rem;
    text-align: center;
  }

  .status-message.success {
    background: #d4edda;
    color: #155724;
    border: 1px solid #c3e6cb;
  }

  .status-message.error {
    background: #f8d7da;
    color: #721c24;
    border: 1px solid #f5c6cb;
  }

  .status-message.info {
    background: #d1ecf1;
    color: #0c5460;
    border: 1px solid #bee5eb;
  }

  .cos-config {
    background: #f8f9fa;
    padding: 1.5rem;
    border-radius: 8px;
    margin-top: 2rem;
  }

  .cos-config h3 {
    margin-top: 0;
    color: #2c3e50;
  }

  .config-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1rem;
  }

  .config-item {
    display: flex;
    justify-content: space-between;
    padding: 0.5rem 0;
    border-bottom: 1px solid #dee2e6;
  }

  .config-item label {
    font-weight: 600;
    color: #495057;
  }

  .config-item span {
    color: #6c757d;
  }
</style>
