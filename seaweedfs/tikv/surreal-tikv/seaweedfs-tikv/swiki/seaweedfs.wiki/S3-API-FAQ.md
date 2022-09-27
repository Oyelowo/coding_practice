# FAQ

## Can not upload due to "no free volumes left"

The symptom is similar to https://github.com/seaweedfs/seaweedfs/issues/1631 where the logs show
```
Nov 20 18:49:37 s2375.j weed[31818]: E1120 18:49:37 31818 filer_server_handlers_write.go:42] failing to assign a file id: rpc error: code = Unknown desc = No free volumes left!
Nov 20 18:49:37 s2375.j weed[31818]: I1120 18:49:37 31818 common.go:53] response method:PUT URL:/buckets/dev-passport-video-recordings/02342a46-7435-b698-2437-c778db34ef59.mp4 with httpStatus:500 and JSON:{"error":"rpc error: code = Unknown desc = No free volumes left!"}
Nov 20 18:49:37 s2375.j weed[31818]: E1120 18:49:37 31818 s3api_object_handlers.go:336] upload to filer error: rpc error: code = Unknown desc = No free volumes left!
```

Each bucket will create one collection, with at least 7 volumes by default, and each volume is pre-allocated with a large size, usually 30GB.

There are 2 ways to fix this. 
* Reduce the global volume size by adjusting `-volumeSizeLimitMB` in `weed master` CLI option. 
* Reduce the number of volumes to grow when a collection runs out of volumes. You can configure the per bucket storage this way in `weed shell`:

```
> fs.configure -locationPrefix=/buckets/ -volumeGrowthCount=1 -apply
```

This will add 1 physical volume when existing volumes are full. If using replication, you will need to add more volumes, so that it is a multiple of the number of replicas:
```
 fs.configure -locationPrefix=/buckets/ -replication=001 -volumeGrowthCount=2 -apply
```

See  https://github.com/seaweedfs/seaweedfs/wiki/Path-Specific-Configuration


## How to speed up bucket deletion?

One common unexpected problem is the deletion can be slow. To delete a file, we need to delete the file content on the volume servers and delete the file entry from the filer store. It is almost the same amount of work as adding a file. If there are millions of files, it can take a long time to delete.

When you need to create large buckets and delete them often, you may choose `leveldb3` as the filer store, or any other stores that supports **Fast Bucket Deletion** in https://github.com/seaweedfs/seaweedfs/wiki/Filer-Stores

`leveldb3` can automatically create a separate LevelDB instance for each bucket. 
So bucket deletion is as simple as deleting the LevelDB instance files and the collection of volume files.

Having separate LevelDB instance, or separate SQL tables, will help to isolate the storage and improve performance also.

## Filer Metadata Store Growth

Due to the semantics of the S3 API, empty directories (aka prefixes) aren't shown. However, an entry is still stored in the filer metadata store. When workload access patterns create many unique directories and then remove all the objects inside those directories, the filer metadata store can grow unbounded with orphaned directories. These directories are visible in the filer metadata store itself, but not using the S3 API.

If the filer argument `-allowEmptyFolder=false` is set, the orphaned directories are cleaned up during list requests for non bucket-level directories. Normally this works well, but if the workload never performs a list operation, the orphaned directories may never be cleaned up. To force cleanup, simply list an existing, non bucket-level directory.

Example using rclone:

```
rclone lsf seaweedfs:my-bucket/dir
```

If the directory `dir` exists in `my-bucket`, the orphaned metadata will be cleaned up. Note that due to slight API usage differences, `rclone ls` does not trigger cleanup, but `rclone lsf` will. 
