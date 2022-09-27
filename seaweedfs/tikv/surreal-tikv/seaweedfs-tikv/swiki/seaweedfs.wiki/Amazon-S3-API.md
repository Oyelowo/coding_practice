To be compatible with Amazon S3 API, a separate "weed s3" command is provided. This provides much faster access when reading or writing files, compared to operating files on the cloud.

# How it works?
`weed s3` will start a stateless gateway server to bridge the Amazon S3 API to SeaweedFS Filer.
For convenience, `weed server -s3` will start a master, a volume server, a filer, and the S3 gateway. And `weed filer -s3` can start a filer and the S3 gateway together also.

Each bucket is stored in one collection, and mapped to folder /buckets/<bucket_name> by default.

A bucket can be deleted efficiently by deleting the whole collection.

## Support Many Buckets

Each bucket has its own collection. Usually one collection uses 7 volumes, where each volume is 30GB by default. So if you want to create multiple buckets, you may run out of volumes very quickly unless you have a large disk.

Try to keep the volume size low. For example,
```
 weed master -volumeSizeLimitMB=1024
```

In addition, you can also configure the per bucket storage this way in `weed shell`:
```
 fs.configure -locationPrefix=/buckets/ -volumeGrowthCount=1 -apply
```
This will add 1 physical volume when existing volumes are full. If using replication, you will need to add more volumes, so that it is a multiple of the number of replicas:
```
 fs.configure -locationPrefix=/buckets/ -replication=001 -volumeGrowthCount=2 -apply
```

See https://github.com/seaweedfs/seaweedfs/wiki/Path-Specific-Configuration


# Supported APIs
Currently, the following APIs are supported.

```
// Object operations
* PutObject
* GetObject
* HeadObject
* CopyObject
* DeleteObject
* ListObjectsV2
* ListObjectsV1
* DeleteMultipleObjects
* PostPolicy

// Object Tagging
* GetObjectTagging
* PutObjectTagging
* DeleteObjectTagging

// Bucket operations
* PutBucket
* DeleteBucket
* HeadBucket
* ListBuckets

// Multipart upload operations
* NewMultipartUpload
* CompleteMultipartUpload
* AbortMultipartUpload
* ListMultipartUploads
* PutObjectPart
* CopyObjectPart
* ListObjectParts
```

Not included:
* Policy

# Feature difference
|  | SeaweedFS | Amazon S3 |
| ---------------------------- | --- | -- |
| Multi byte ranges reads      | Yes | No |
| DeleteObject deletes a folder| Yes | No |
| same path for both a file and a folder| No | Yes |

## Empty folders

SeaweedFS has directories while AWS S3 only has objects with "fake" directories. In AWS S3, if the last file is deleted in a directory, the directory will disappear also.

To be consistent with AWS S3, SeaweedFS tries to skip empty folders while listing. You can use `weed s3 -allowEmptyFolder` to toggle this behavior. 

This is not so ideal. Another approach is to list current directory when deleting a file, which will slow down quite a bit especially when deleting multiple files. SeaweedFS did not take this approach.

The last approach, which is most efficient, is to maintain counters for each folder, and drop the folder as soon as it becomes empty. This is implemented in [[Cloud Monitoring]].

# S3 Authentication
By default, the access key and secret key to access `weed s3` is not authenticated. To enable credential based access, you can choose static or dynamic configuration:
* **Dynamic Configuration**: setup auth with `s3.configure` in `weed shell`
* **Static Configuration**: create a config.json file similar to the example below, and specify it via `weed s3 -config=config.json`

## Dynamic Configuration
```
> s3.configure -access_key=any -secret_key=any -buckets=bucket1 -user=me -actions=Read,Write,List,Tagging,Admin -apply
{
  "identities": [
    {
      "name": "me",
      "credentials": [
        {
          "accessKey": "any",
          "secretKey": "any"
        }
      ],
      "actions": [
        "Read:bucket1",
        "Write:bucket1",
        "List:bucket1",
        "Tagging:bucket1",
        "Admin:bucket1"
      ]
    }
  ]
}
```

## Static Configuration

To enable credential based access, create a config.json file similar to the example below, and specify it via `weed s3 -config=config.json`.

You just need to create a user with all "Admin", "Read", "Write", "List", "Tagging" actions.
You can create as many users as needed. Each user can have multiple credentials.

* The "Admin" action is needed to list, create, and delete buckets.
* The "Write" action allows uploading files to all buckets.
* The "Read" action allows reading files from all buckets.
* The "List" action allows listing files from all buckets.
* The "Tagging" action allows tagging files from all buckets.
* The "Write:<bucket_name>" action allows uploading files within a bucket, e.g., "Write:bucket1".
* The "Read:<bucket_name>" action allows reading files within a bucket, e.g., "Read:bucket2".
* The "List:<bucket_name>" action allows listing files within a bucket, e.g., "List:bucket2".
* The "Tagging:<bucket_name>" action allows tagging files within a bucket, e.g., "Tagging:bucket2".


For public access, you can configure an identity with name "anonymous", usually with just "Read" action, or access to specific buckets.

```
{
  "identities": [
    {
      "name": "anonymous",
      "actions": [
        "Read"
      ]
    },
    {
      "name": "some_name",
      "credentials": [
        {
          "accessKey": "some_access_key1",
          "secretKey": "some_secret_key1"
        }
      ],
      "actions": [
        "Admin",
        "Read",
        "List",
        "Tagging",
        "Write"
      ]
    },
    {
      "name": "some_read_only_user",
      "credentials": [
        {
          "accessKey": "some_access_key2",
          "secretKey": "some_secret_key2"
        }
      ],
      "actions": [
        "Read",
        "List"
      ]
    },
    {
      "name": "some_normal_user",
      "credentials": [
        {
          "accessKey": "some_access_key3",
          "secretKey": "some_secret_key3"
        }
      ],
      "actions": [
        "Read",
        "List",
        "Tagging",
        "Write"
      ]
    },
    {
      "name": "user_limited_by_bucket",
      "credentials": [
        {
          "accessKey": "some_access_key4",
          "secretKey": "some_secret_key4"
        }
      ],
      "actions": [
        "Read:bucket1",
        "Read:bucket2",
        "Read:bucket3",
        "Write:bucket1"
      ]
    }
  ]
}
```

### Actions with wildcard

Wildcard is partially supported for prefix lookup. The following example actions are allowed:

```
Read
Read:bucket
Read:bucket_prefix*
Read:bucket/*
Read:bucket/a/b/*

```

## Presigned URL

Presigned URL is supported. See [[AWS-CLI-with-SeaweedFS#presigned-url]] for example.


## Multiple S3 Nodes

If you need to setup multiple S3 nodes, you can just start multiple s3 instances pointing to a filer.

Usually you would also want to have multiple filers. The easiest way is to run filer together with a S3.

```

  weed filer -s3

```

# Authentication with Filer

You can use mTLS for the gRPC connection between S3-API-Proxy and the filer, as
explained in [[Security-Configuration]] -
controlled by the `grpc.*` configuration in `security.toml`.

**Starting with version 2.84, it is also possible to authenticate the HTTP
operations between the S3-API-Proxy and the Filer (especially
uploading new files).** This is configured by setting
`jwt.filer_signing.key` and `jwt.filer_signing.read.key` in
`security.toml`.

With both configurations (gRPC and JWT), it is possible to have Filer
and S3 communicate in fully authenticated fashion; so Filer will reject
any unauthenticated communication.
