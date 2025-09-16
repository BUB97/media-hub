import COS from 'cos-js-sdk-v5';

/**
 * 腾讯云COS上传工具类
 */
export class COSUploader {
  constructor() {
    this.cos = null;
    this.credentials = null;
  }

  /**
   * 初始化COS实例
   * @param {Object} credentials - STS临时凭证
   * @param {string} credentials.tmpSecretId - 临时SecretId
   * @param {string} credentials.tmpSecretKey - 临时SecretKey
   * @param {string} credentials.sessionToken - 会话令牌
   * @param {string} region - COS区域
   */
  init(credentials, region = 'ap-beijing') {
    this.credentials = credentials;

    this.cos = new COS({
      SecretId: credentials.tmpSecretId,
      SecretKey: credentials.tmpSecretKey,
      SecurityToken: credentials.sessionToken,
      // 可选配置
      FileParallelLimit: 3, // 控制文件上传并发数
      ChunkParallelLimit: 8, // 控制单个文件下分片上传并发数，在同园区上传可以设置较大的并发数
      ChunkSize: 1024 * 1024 * 8, // 控制分片大小，单位 B，在同园区上传可以设置较大的分片大小
      Timeout: 60000, // 超时时间
      Region: region, // 存储桶所在地域
    });
  }

  /**
   * 上传文件到COS
   * @param {Object} options - 上传选项
   * @param {File} options.file - 要上传的文件
   * @param {string} options.bucket - 存储桶名称
   * @param {string} options.key - 对象键（文件路径）
   * @param {string} options.region - 区域
   * @param {Function} options.onProgress - 进度回调函数
   * @returns {Promise} 上传结果
   */
  async uploadFile({ file, bucket, key, region, onProgress }) {
    if (!this.cos) {
      throw new Error('COS实例未初始化，请先调用init方法');
    }

    return new Promise((resolve, reject) => {
      this.cos.uploadFile(
        {
          Bucket: bucket,
          Region: region,
          Key: key,
          Body: file,
          SliceSize: 1024 * 1024 * 5, // 大于5MB的文件使用分片上传
          onProgress: (progressData) => {
            if (onProgress) {
              const percent = Math.round(progressData.percent * 100);
              onProgress({
                percent,
                loaded: progressData.loaded,
                total: progressData.total,
                speed: progressData.speed,
              });
            }
          },
          onTaskReady: (taskId) => {
            console.log('上传任务已准备:', taskId);
          },
        },
        (err, data) => {
          if (err) {
            console.error('上传失败:', err);
            reject(err);
          } else {
            console.log('上传成功:', data);
            resolve({
              location: data.Location,
              bucket: data.Bucket,
              key: data.Key,
              etag: data.ETag,
              url: `https://${data.Location}`,
            });
          }
        }
      );
    });
  }

  /**
   * 生成预签名URL
   * @param {Object} options - 选项
   * @param {string} options.bucket - 存储桶名称
   * @param {string} options.key - 对象键
   * @param {string} options.region - 区域
   * @param {number} options.expires - 过期时间（秒）
   * @returns {string} 预签名URL
   */
  getObjectUrl({ bucket, key, region, expires = 3600 }) {
    if (!this.cos) {
      throw new Error('COS实例未初始化，请先调用init方法');
    }

    return this.cos.getObjectUrl({
      Bucket: bucket,
      Region: region,
      Key: key,
      Expires: expires,
      Sign: true,
    });
  }

  /**
   * 检查凭证是否即将过期
   * @param {number} bufferTime - 缓冲时间（秒），默认5分钟
   * @returns {boolean} 是否需要刷新凭证
   */
  needsRefresh(bufferTime = 300) {
    if (!this.credentials || !this.credentials.expiredTime) {
      return true;
    }

    const now = Math.floor(Date.now() / 1000);
    const expiredTime = new Date(this.credentials.expiredTime).getTime() / 1000;

    return expiredTime - now < bufferTime;
  }

  /**
   * 销毁COS实例
   */
  destroy() {
    if (this.cos) {
      // 取消所有正在进行的任务
      this.cos.cancelTask();
      this.cos = null;
    }
    this.credentials = null;
  }
}

// 创建单例实例
export const cosUploader = new COSUploader();

// 默认导出
export default cosUploader;
