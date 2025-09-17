import apiClient from './client';
import axios from 'axios';

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

    // 直接上传文件并创建媒体记录（一体化操作）
    createMediaWithFile: async (
        file: File,
        title?: string,
        description?: string,
        onProgress?: (progress: number) => void
    ): Promise<Media> => {
        console.log('🚀 开始上传文件:', file.name, '大小:', file.size, '类型:', file.type);
        
        try {
            // 1. 获取COS配置和STS凭证
            console.log('📡 获取COS配置和STS凭证...');
            const [cosConfig, stsCredentials] = await Promise.all([
                mediaAPI.getCosConfig(),
                mediaAPI.getStsCredentials()
            ]);
            console.log('✅ COS配置获取成功:', cosConfig);
            console.log('✅ STS凭证获取成功');

            // 2. 根据文件类型确定媒体类型
            const fileExtension = file.name.split('.').pop()?.toLowerCase() || '';
            let mediaType: 'image' | 'video' | 'audio' | 'document' = 'document';
            if (['jpg', 'jpeg', 'png', 'gif', 'webp', 'svg'].includes(fileExtension)) {
                mediaType = 'image';
            } else if (['mp4', 'avi', 'mov', 'wmv', 'flv', 'webm'].includes(fileExtension)) {
                mediaType = 'video';
            } else if (['mp3', 'wav', 'flac', 'aac', 'ogg'].includes(fileExtension)) {
                mediaType = 'audio';
            }
            console.log('📁 文件类型判断:', fileExtension, '->', mediaType);

            // 3. 生成COS对象键
            const timestamp = Date.now();
            const randomStr = Math.random().toString(36).substring(2, 8);
            const cosKey = `${cosConfig.upload_prefix}${timestamp}_${randomStr}_${file.name}`;
            console.log('🔑 生成COS对象键:', cosKey);

            // 4. 准备FormData
            const formData = new FormData();
            formData.append('key', cosKey);
            formData.append('policy', stsCredentials.policy);
            formData.append('q-sign-algorithm', stsCredentials.qSignAlgorithm);
            formData.append('q-ak', stsCredentials.qAk);
            formData.append('q-key-time', stsCredentials.qKeyTime);
            formData.append('q-signature', stsCredentials.qSignature);
            formData.append('file', file);

            // 5. 上传文件到COS
            const uploadUrl = `https://${cosConfig.bucket}.cos.${cosConfig.region}.myqcloud.com/`;
            console.log('☁️ 开始上传到COS:', uploadUrl);
            
            await axios.post(uploadUrl, formData, {
                headers: {
                    'Content-Type': 'multipart/form-data',
                },
                onUploadProgress: (progressEvent) => {
                    if (progressEvent.total && onProgress) {
                        const progress = Math.round((progressEvent.loaded / progressEvent.total) * 100);
                        onProgress(progress);
                    }
                },
                timeout: 300000, // 5分钟超时
            });
            console.log('✅ 文件上传到COS成功');

            // 6. 创建媒体记录
            const cosUrl = `http://${cosConfig.bucket}.cos.${cosConfig.region}.myqcloud.com/${cosKey}`;
            console.log('🔗 生成的COS URL:', cosUrl);
            
            const mediaData: CreateMediaRequest = {
                title: title || file.name,
                description: description || `上传的${mediaType === 'image' ? '图片' : mediaType === 'video' ? '视频' : mediaType === 'audio' ? '音频' : '文档'}文件`,
                filename: cosKey,
                original_filename: file.name,
                file_size: file.size,
                content_type: file.type,
                cos_key: cosKey,
                cos_url: cosUrl,
                cos_bucket: cosConfig.bucket,
                cos_region: cosConfig.region,
                media_type: mediaType,
            };
            
            console.log('💾 准备创建媒体记录:', mediaData);
            const response = await apiClient.post('/media', mediaData);
            console.log('✅ 媒体记录创建成功:', response.data);
            
            return response.data;

        } catch (error: any) {
            console.error('❌ 文件上传并创建媒体记录失败:', error);
            if (error.response) {
                console.error('❌ 响应错误:', error.response.status, error.response.data);
            }
            throw error;
        }
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
      
            // 使用axios来支持进度回调
            await axios.post(uploadUrl, formData, {
                headers: {
                    'Content-Type': 'multipart/form-data',
                },
                onUploadProgress: (progressEvent) => {
                    if (progressEvent.total && onProgress) {
                        const progress = Math.round((progressEvent.loaded / progressEvent.total) * 100);
                        onProgress(progress);
                    }
                },
                timeout: 300000, // 5分钟超时
            });
      
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