import apiClient from './client';

// 媒体接口类型定义
export interface Media {
  id: string
  user_id: string
  title: string
  description?: string
  filename: string
  original_filename: string
  file_size: number
  content_type: string
  cos_key: string
  cos_url: string
  cos_bucket: string
  cos_region: string
  media_type: 'video' | 'audio' | 'image' | 'document'
  status: string
  metadata?: any
  created_at: string
  updated_at: string
}

export interface MediaListResponse {
  items: Media[]
  total: number
  page: number
  per_page: number
}

export interface CreateMediaRequest {
  title: string
  description?: string
  filename: string
  original_filename: string
  file_size: number
  content_type: string
  cos_key: string
  cos_url: string
  cos_bucket: string
  cos_region: string
  media_type: 'video' | 'audio' | 'image' | 'document'
  metadata?: any
}

export interface MediaQueryParams {
  page?: number
  per_page?: number
  media_type?: string
  q?: string
}

export interface UpdateMediaRequest {
  title?: string
  description?: string
}

// 媒体 API 函数
export const mediaAPI = {
    // 获取媒体列表（支持分页和搜索）
    getMediaList: async (params?: MediaQueryParams): Promise<MediaListResponse> => {
        const response = await apiClient.get('/media', { params });
        return response.data;
    },

    // 根据 ID 获取媒体详情
    getMediaById: async (id: string): Promise<Media> => {
        const response = await apiClient.get(`/media/${id}`);
        return response.data;
    },

    // 创建新媒体记录
    createMedia: async (data: CreateMediaRequest): Promise<Media> => {
        const response = await apiClient.post('/media', data);
        return response.data;
    },

    // 更新媒体信息
    updateMedia: async (id: string, data: UpdateMediaRequest): Promise<Media> => {
        const response = await apiClient.put(`/media/${id}`, data);
        return response.data;
    },

    // 删除媒体
    deleteMedia: async (id: string): Promise<void> => {
        await apiClient.delete(`/media/${id}`);
    },

    // 搜索媒体
    searchMedia: async (params: MediaQueryParams): Promise<MediaListResponse> => {
        const response = await apiClient.get('/media/search', { params });
        return response.data;
    },

    // 获取腾讯云 STS 临时凭证
    getStsCredentials: async () => {
        const response = await apiClient.get('/cos/sts');
        return response.data;
    },

    // 获取腾讯云 COS 配置
    getCosConfig: async () => {
        const response = await apiClient.get('/cos/config');
        return response.data;
    },

    // 验证文件上传参数
    validateFileUpload: async (data: any) => {
        const response = await apiClient.post('/cos/validate', data);
        return response.data;
    },

    // 获取媒体文件下载链接（直接使用 COS URL）
    getMediaDownloadUrl: (media: Media): string => {
        return media.cos_url;
    },

    // 获取媒体文件预览链接
    getMediaPreviewUrl: (media: Media): string => {
        return media.cos_url;
    },

    // 上传媒体文件
    uploadMediaFile: async (
        mediaId: string,
        file: File,
        onProgress?: (progress: number) => void
    ): Promise<Media> => {
        try {
            // 1. 获取STS临时凭证
            const stsCredentials = await mediaAPI.getStsCredentials();
      
            // 2. 获取COS配置
            const cosConfig = await mediaAPI.getCosConfig();
      
            // 3. 生成文件key
            const fileExtension = file.name.split('.').pop();
            const cosKey = `media/${Date.now()}_${Math.random().toString(36).substr(2, 9)}.${fileExtension}`;
      
            // 4. 上传文件到COS
            const formData = new FormData();
            formData.append('key', cosKey);
            formData.append('policy', stsCredentials.policy);
            formData.append('q-sign-algorithm', 'sha1');
            formData.append('q-ak', stsCredentials.credentials.tmpSecretId);
            formData.append('q-key-time', stsCredentials.keyTime);
            formData.append('q-signature', stsCredentials.signature);
            formData.append('q-sign-time', stsCredentials.keyTime);
            formData.append('file', file);
      
            const uploadUrl = `https://${cosConfig.bucket}.cos.${cosConfig.region}.myqcloud.com/`;
      
            // 使用XMLHttpRequest来支持进度回调
            const uploadPromise = new Promise<void>((resolve, reject) => {
                const xhr = new XMLHttpRequest();
        
                xhr.upload.addEventListener('progress', (event) => {
                    if (event.lengthComputable && onProgress) {
                        const progress = Math.round((event.loaded / event.total) * 100);
                        onProgress(progress);
                    }
                });
        
                xhr.addEventListener('load', () => {
                    if (xhr.status >= 200 && xhr.status < 300) {
                        resolve();
                    } else {
                        reject(new Error(`Upload failed with status ${xhr.status}`));
                    }
                });
        
                xhr.addEventListener('error', () => {
                    reject(new Error('Upload failed'));
                });
        
                xhr.open('POST', uploadUrl);
                xhr.send(formData);
            });
      
            await uploadPromise;
      
            // 5. 更新媒体记录
            const cosUrl = `https://${cosConfig.bucket}.cos.${cosConfig.region}.myqcloud.com/${cosKey}`;
      
            const updateData = {
                filename: cosKey,
                original_filename: file.name,
                file_size: file.size,
                content_type: file.type,
                cos_key: cosKey,
                cos_url: cosUrl,
                cos_bucket: cosConfig.bucket,
                cos_region: cosConfig.region,
                media_type: file.type.startsWith('image/') ? 'image' as const :
                    file.type.startsWith('video/') ? 'video' as const :
                        file.type.startsWith('audio/') ? 'audio' as const : 'document' as const
            };
      
            // 调用后端API更新媒体记录
            const response = await apiClient.put(`/media/${mediaId}/upload`, updateData);
            return response.data;
      
        } catch (error) {
            console.error('文件上传失败:', error);
            throw error;
        }
    },
};