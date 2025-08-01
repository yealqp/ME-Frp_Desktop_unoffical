#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
import sys
import argparse
from qcloud_cos import CosConfig
from qcloud_cos import CosS3Client
from qcloud_cos.cos_exception import CosClientError, CosServiceError

def upload_file_to_cos(file_path, cos_key, bucket, region, secret_id, secret_key):
    """
    上传文件到腾讯云COS
    
    Args:
        file_path: 本地文件路径
        cos_key: COS对象键名
        bucket: COS存储桶名称
        region: COS地域
        secret_id: 腾讯云SecretId
        secret_key: 腾讯云SecretKey
    """
    try:
        # 配置COS客户端
        config = CosConfig(Region=region, SecretId=secret_id, SecretKey=secret_key)
        client = CosS3Client(config)
        
        # 检查文件是否存在
        if not os.path.exists(file_path):
            print(f"错误: 文件 {file_path} 不存在")
            return False
            
        # 获取文件大小
        file_size = os.path.getsize(file_path)
        print(f"开始上传文件: {file_path} ({file_size} bytes)")
        print(f"目标位置: {bucket}/{cos_key}")
        
        # 上传文件
        response = client.upload_file(
            Bucket=bucket,
            LocalFilePath=file_path,
            Key=cos_key,
            PartSize=1,
            MAXThread=10,
            EnableMD5=False
        )
        
        print(f"上传成功! ETag: {response['ETag']}")
        
        # 生成访问URL
        url = f"https://{bucket}.cos.{region}.myqcloud.com/{cos_key}"
        print(f"访问URL: {url}")
        
        return True
        
    except CosClientError as e:
        print(f"客户端错误: {e}")
        return False
    except CosServiceError as e:
        print(f"服务端错误: {e.get_error_code()}, {e.get_error_msg()}")
        return False
    except Exception as e:
        print(f"未知错误: {e}")
        return False

def main():
    parser = argparse.ArgumentParser(description='上传文件到腾讯云COS')
    parser.add_argument('file_path', help='要上传的文件路径')
    parser.add_argument('cos_key', help='COS对象键名')
    parser.add_argument('--bucket', required=True, help='COS存储桶名称')
    parser.add_argument('--region', required=True, help='COS地域')
    parser.add_argument('--secret-id', required=True, help='腾讯云SecretId')
    parser.add_argument('--secret-key', required=True, help='腾讯云SecretKey')
    
    args = parser.parse_args()
    
    success = upload_file_to_cos(
        file_path=args.file_path,
        cos_key=args.cos_key,
        bucket=args.bucket,
        region=args.region,
        secret_id=args.secret_id,
        secret_key=args.secret_key
    )
    
    if success:
        print("文件上传完成!")
        sys.exit(0)
    else:
        print("文件上传失败!")
        sys.exit(1)

if __name__ == '__main__':
    main()