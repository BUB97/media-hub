import apiClient from './client'

// 媒体接口类型定义
export interface Media {
  id: number
  title: string
  description: string
  media_type: 'video' | 'audio' | 'image'
  file_path?: string
  file_size?: number
  duration?: number
  created_at?: string
  updated_at?: string
}

export interface CreateMediaRequest {
  title: string
  description: string
  media_type: 'video' | 'audio' | 'image'
}

export interface UpdateMediaRequest {
  title?: string
  description?: string
}

// 媒体 API 函数
export const mediaAPI = {
  // 获取所有媒体列表
  getMediaList: async (): Promise<Media[]> => {
    const response = await apiClient.get('/media')
    return response.data
  },

  // 根据 ID 获取媒体详情
  getMediaById: async (id: number): Promise<Media> => {
    const response = await apiClient.get(`/media/${id}`)
    return response.data
  },

  // 创建新媒体记录
  createMedia: async (data: CreateMediaRequest): Promise<Media> => {
    const response = await apiClient.post('/media', data)
    return response.data
  },

  // 更新媒体信息
  updateMedia: async (id: number, data: UpdateMediaRequest): Promise<Media> => {
    const response = await apiClient.put(`/media/${id}`, data)
    return response.data
  },

  // 删除媒体
  deleteMedia: async (id: number): Promise<{ message: string }> => {
    const response = await apiClient.delete(`/media/${id}`)
    return response.data
  },

  // 上传媒体文件
  uploadMediaFile: async (id: number, file: File, onProgress?: (progress: number) => void): Promise<Media> => {
    const formData = new FormData()
    formData.append('file', file)

    const response = await apiClient.post(`/media/${id}/upload`, formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
      onUploadProgress: (progressEvent) => {
        if (onProgress && progressEvent.total) {
          const progress = Math.round((progressEvent.loaded * 100) / progressEvent.total)
          onProgress(progress)
        }
      },
    })
    return response.data
  },

  // 获取媒体文件下载链接
  getMediaDownloadUrl: (id: number): string => {
    return `${apiClient.defaults.baseURL}/media/${id}/download`
  },

  // 获取媒体文件流媒体链接
  getMediaStreamUrl: (id: number): string => {
    return `${apiClient.defaults.baseURL}/media/${id}/stream`
  },
}