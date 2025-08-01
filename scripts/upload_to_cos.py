#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
import sys
import argparse
from qcloud_cos import CosConfig
from qcloud_cos import CosS3Client
from qcloud_cos.cos_exception import CosClientError, CosServiceError

# Set UTF-8 encoding for Windows
if sys.platform == 'win32':
    import codecs
    sys.stdout = codecs.getwriter('utf-8')(sys.stdout.buffer)
    sys.stderr = codecs.getwriter('utf-8')(sys.stderr.buffer)

def upload_file_to_cos(file_path, cos_key, bucket, region, secret_id, secret_key):
    """
    Upload file to Tencent Cloud COS
    
    Args:
        file_path: Local file path
        cos_key: COS object key
        bucket: COS bucket name
        region: COS region
        secret_id: Tencent Cloud SecretId
        secret_key: Tencent Cloud SecretKey
    """
    # Validate required parameters
    if not all([bucket, region, secret_id, secret_key]):
        print("Error: Missing required COS configuration parameters")
        print(f"Bucket: {'✓' if bucket else '✗'}")
        print(f"Region: {'✓' if region else '✗'}")
        print(f"SecretId: {'✓' if secret_id else '✗'}")
        print(f"SecretKey: {'✓' if secret_key else '✗'}")
        return False
        
    try:
        # 配置COS客户端
        config = CosConfig(Region=region, SecretId=secret_id, SecretKey=secret_key)
        client = CosS3Client(config)
        
        # Check if file exists
        if not os.path.exists(file_path):
            print(f"Error: File {file_path} does not exist")
            return False
            
        # Get file size
        file_size = os.path.getsize(file_path)
        print(f"Starting upload: {file_path} ({file_size} bytes)")
        print(f"Target location: {bucket}/{cos_key}")
        
        # Upload file
        response = client.upload_file(
            Bucket=bucket,
            LocalFilePath=file_path,
            Key=cos_key,
            PartSize=1,
            MAXThread=10,
            EnableMD5=False
        )
        
        print(f"Upload successful! ETag: {response['ETag']}")
        
        # Generate access URL
        url = f"https://{bucket}.cos.{region}.myqcloud.com/{cos_key}"
        print(f"Access URL: {url}")
        
        return True
        
    except CosClientError as e:
        print(f"Client error: {e}")
        return False
    except CosServiceError as e:
        print(f"Service error: {e.get_error_code()}, {e.get_error_msg()}")
        return False
    except Exception as e:
        print(f"Unknown error: {e}")
        return False

def main():
    parser = argparse.ArgumentParser(description='Upload file to Tencent Cloud COS')
    parser.add_argument('file_path', help='File path to upload')
    parser.add_argument('cos_key', help='COS object key')
    parser.add_argument('--bucket', required=True, help='COS bucket name')
    parser.add_argument('--region', required=True, help='COS region')
    parser.add_argument('--secret-id', required=True, help='Tencent Cloud SecretId')
    parser.add_argument('--secret-key', required=True, help='Tencent Cloud SecretKey')
    
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
        print("File upload completed!")
        sys.exit(0)
    else:
        print("File upload failed!")
        sys.exit(1)

if __name__ == '__main__':
    main()