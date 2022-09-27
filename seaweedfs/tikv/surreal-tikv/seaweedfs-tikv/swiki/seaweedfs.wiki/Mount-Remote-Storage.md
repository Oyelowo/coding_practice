After [[Configure Remote Storage]], you will get a storage name `cloud1`.

## Mount a Remote Storage
Now you can run `remote.mount` in `weed shell`:
```
> remote.mount -h
Usage of remote.mount:
  -dir string
    	a directory in filer
  -nonempty
    	allows the mounting over a non-empty directory
  -remote string
    	a directory in remote storage, ex. <storageName>/<bucket>/path/to/dir

> help remote.mount
  remote.mount	# mount remote storage and pull its metadata

	# assume a remote storage is configured to name "s3_1"
	remote.configure -name=cloud1 -type=s3 -s3.access_key=xyz -s3.secret_key=yyy

	# mount and pull one bucket
	remote.mount -dir=/xxx -remote=cloud1/bucket
	# mount and pull one directory in the bucket
	remote.mount -dir=/xxx -remote=cloud1/bucket/dir1

	# after mount, start a separate process to write updates to remote storage
	weed filer.remote.sync -filer=<filerHost>:<filerPort> -dir=/xxx

```

With `remote.mount`, you can mount one bucket or any directory in the bucket.

## Cache Metadata

This `remote.mount` will also pull down all metadata from the remote storage. 

Later, any metadata operations will be fast to just read local metadata.

`remote.unmount` will drop all local metadata and cached file content.

### Repeatedly Update Metadata

Sometimes the data on the cloud has changes and local metadata becomes stale. To `unmount` first and `mount` again can work but costly, since all data has to be cached again. 

To refresh the metadata changes, you can run this on the mounted directory or any sub directories, e.g.:
```
	remote.meta.sync -dir=/xxx
	remote.meta.sync -dir=/xxx/sub/dir
```
This will update local metadata accordingly and still keep file contents that are not changed.

If the data on the cloud can changed often, you can create a cronjob to run it. Or you can add this command to the admin scripts defined in `master.toml`, to run it regularly.

## Write Back changes to Remote Storage

If the mounted directory is only for reading, you can skip this step.

If local changes need to be synchronized to the cloud storage, a separate process `weed filer.remote.sync -dir=xxx` should be started.
This process will listen to filer change events, and write any changes back to the remote storage.

```
weed filer.remote.sync -filer=<filerHost>:<filerPort> -dir=xxx
```
The process is designed to be worry-free. It should automatically resume if stopped, and can reconnect automatically.

## Unmount a Remote Storage

Similarly, running `remote.unmount -dir=xxx` can unmount a remote storage. However, this means all cached data and metadata will be deleted.
And if `weed filer.remote.sync -filer=<filerHost>:<filerPort> -dir=xxx` was not run, the local updates have not been uploaded to the remote storage, so these local updates will be lost.

The `weed filer.remote.sync` will stop as soon as it sees the directory is unmounted. So the local deletion will not propagate back to the cloud, avoiding possible data loss.

