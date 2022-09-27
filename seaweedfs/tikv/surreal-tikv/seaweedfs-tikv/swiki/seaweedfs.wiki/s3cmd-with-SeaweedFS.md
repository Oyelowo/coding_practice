### Installation
See https://s3tools.org/download 

1. Download the zip, unzip, cd into the folder.
1. python setup.py install

### Configuration
Answer questions with the following answers:
```
s3cmd --configure

...

New settings:
  Access Key: any_no_empty_key
  Secret Key: any_no_empty_key
  Default Region: US
  S3 Endpoint: localhost:8333
  DNS-style bucket+hostname:port template for accessing a bucket: localhost:8333
  Encryption password:
  Path to GPG program: /usr/local/bin/gpg
  Use HTTPS protocol: False
  HTTP Proxy server name:
  HTTP Proxy server port: 0

```

Make sure the `.s3cfg` file has these values
```
# Setup endpoint
host_base = localhost:8333
host_bucket = localhost:8333
use_https = No
# Enable S3 v4 signature APIs
signature_v2 = False
```

### Execute commands
```
$ s3cmd mb s3://newbucket
Bucket 's3://newbucket/' created

$ s3cmd ls s3://
2019-01-01 01:30  s3://newbucket

$ s3cmd put /etc/passwd s3://newbucket
WARNING: Module python-magic is not available. Guessing MIME types based on file extensions.
upload: '/etc/passwd' -> 's3://newbucket/passwd'  [1 of 1]
 6804 of 6804   100% in    0s    87.93 kB/s  done

$ s3cmd get s3://newbucket/passwd
download: 's3://newbucket/passwd' -> './passwd'  [1 of 1]
 6804 of 6804   100% in    0s   595.33 kB/s  done

# change the file
$ vi passwd

$ s3cmd sync passwd s3://newbucket/
WARNING: Module python-magic is not available. Guessing MIME types based on file extensions.
upload: 'passwd' -> 's3://newbucket/passwd'  [1 of 1]
 22 of 22   100% in    0s     6.45 kB/s  done
Done. Uploaded 22 bytes in 1.0 seconds, 22.00 B/s.

$ s3cmd ls s3://newbucket/
2019-01-01 01:32         22   s3://newbucket/passwd

$ s3cmd del s3://newbucket/passwd
delete: 's3://newbucket/passwd'

$ s3cmd rb s3://newbucket
Bucket 's3://newbucket/' removed

```