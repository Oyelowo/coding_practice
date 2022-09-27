### Installation
See https://aws.amazon.com/cli/

On mac or linux: `pip install awscli`

### Configuration
Configure for AWS. The key values do not matter.
```
$ aws configure
AWS Access Key ID [None]: any
AWS Secret Access Key [None]: any
Default region name [None]: us-east-1
Default output format [None]:
```

Also, make sure using AWS signature version 4:
```
$ aws configure set default.s3.signature_version s3v4

```

### Execute commands
```
# list buckets
$ aws --endpoint-url http://localhost:8333 s3 ls
2019-01-02 01:59:25 newbucket

# list files inside the bucket
$ aws --endpoint-url http://localhost:8333 s3 ls s3://newbucket
2019-01-02 12:52:44       6804 password

# make a bucket
$ aws --endpoint-url http://localhost:8333 s3 mb s3://newbucket3
make_bucket: newbucket3

# add an object
$ aws --endpoint-url http://localhost:8333 s3 cp /etc/passwd s3://newbucket3
upload: ../../../../../etc/passwd to s3://newbucket3/passwd

# copy an object
$ aws --endpoint-url http://localhost:8333 s3 cp s3://newbucket3/passwd s3://newbucket3/passwd.txt
copy: s3://newbucket3/passwd to s3://newbucket3/passwd.txt

# remove an object
$ aws --endpoint-url http://localhost:8333 s3 rm s3://newbucket3/passwd
delete: s3://newbucket3/passwd

# remove a bucket
$ aws --endpoint-url http://localhost:8333 s3 rb s3://newbucket3
remove_bucket: newbucket3

```


# Presigned URL

If [authentication](https://github.com/seaweedfs/seaweedfs/wiki/Amazon-S3-API#authentication) is enabled, the url is not accessible without proper credentials. But you can presign a url and access it.

```

# presign url, default to 1 hour
$ aws --endpoint-url http://localhost:8333 s3 presign s3://newbucket/t.txt
http://localhost:8333/newbucket/t.txt?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=some_access_key1%2F20200726%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20200726T161749Z&X-Amz-Expires=3600&X-Amz-SignedHeaders=host&X-Amz-Signature=e0cc153209e414ca8168661f57827aa03ab84e7041ef9270ff639bcc519d24f5

# access the url
$ curl "http://localhost:8333/newbucket/t.txt?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=some_access_key1%2F20200726%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20200726T161749Z&X-Amz-Expires=3600&X-Amz-SignedHeaders=host&X-Amz-Signature=e0cc153209e414ca8168661f57827aa03ab84e7041ef9270ff639bcc519d24f5"

```
