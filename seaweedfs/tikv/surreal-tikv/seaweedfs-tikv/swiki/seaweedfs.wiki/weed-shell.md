`weed shell` starts an interactive console to do some maintenance operations.

```
$ weed shell
> help
Type:	"help <command>" for help on <command>. Most commands support "<command> -h" also for options.
  cluster.ps                    	# check current cluster process status
  collection.delete             	# delete specified collection
  collection.list               	# list all collections
  ec.balance                    	# balance all ec shards among all racks and volume servers
  ec.decode                     	# decode a erasure coded volume into a normal volume
  ec.encode                     	# apply erasure coding to a volume
  ec.rebuild                    	# find and rebuild missing ec shards among volume servers
  fs.cat                        	# stream the file content on to the screen
  fs.cd                         	# change directory to a directory /path/to/dir
  fs.configure                  	# configure and apply storage options for each location
  fs.du                         	# show disk usage
  fs.ls                         	# list all files under a directory
  fs.meta.cat                   	# print out the meta data content for a file or directory
  fs.meta.load                  	# load saved filer meta data to restore the directory and file structure
  fs.meta.notify                	# recursively send directory and file meta data to notifiction message queue
  fs.meta.save                  	# save all directory and file meta data to a local file for metadata backup.
  fs.mkdir                      	# create a directory
  fs.mv                         	# move or rename a file or a folder
  fs.pwd                        	# print out current directory
  fs.rm                         	# remove file and directory entries
  fs.tree                       	# recursively list all files under a directory
  lock                          	# lock in order to exclusively manage the cluster
  remote.cache                  	# cache the file content for mounted directories or files
  remote.configure              	# remote storage configuration
  remote.meta.sync              	# synchronize the local file meta data with the remote file metadata
  remote.mount                  	# mount remote storage and pull its metadata
  remote.mount.buckets          	# mount all buckets in remote storage and pull its metadata
  remote.uncache                	# keep the metadata but remote cache the file content for mounted directories or files
  remote.unmount                	# unmount remote storage
  s3.bucket.create              	# create a bucket with a given name
  s3.bucket.delete              	# delete a bucket by a given name
  s3.bucket.list                	# list all buckets
  s3.bucket.quota               	# set/remove/enable/disable quota for a bucket
  s3.bucket.quota.enforce       	# check quota for all buckets, make the bucket read only if over the limit
  s3.clean.uploads              	# clean up stale multipart uploads
  s3.configure                  	# configure and apply s3 options for each bucket
  unlock                        	# unlock the cluster-wide lock
  volume.balance                	# balance all volumes among volume servers
  volume.check.disk             	# check all replicated volumes to find and fix inconsistencies. It is optional and resource intensive.
  volume.configure.replication  	# change volume replication value
  volume.copy                   	# copy a volume from one volume server to another volume server
  volume.delete                 	# delete a live volume from one volume server
  volume.deleteEmpty            	# delete empty volumes from all volume servers
  volume.fix.replication        	# add or remove replicas to volumes that are missing replicas or over-replicated
  volume.fsck                   	# check all volumes to find entries not used by the filer
  volume.list                   	# list all volumes
  volume.mark                   	# Mark volume writable or readonly from one volume server
  volume.mount                  	# mount a volume from one volume server
  volume.move                   	# move a live volume from one volume server to another volume server
  volume.tier.download          	# download the dat file of a volume from a remote tier
  volume.tier.move              	# change a volume from one disk type to another
  volume.tier.upload            	# upload the dat file of a volume to a remote tier
  volume.unmount                	# unmount a volume from one volume server
  volume.vacuum                 	# compact volumes if deleted entries are more than the limit
  volumeServer.evacuate         	# move out all data on a volume server
  volumeServer.leave            	# stop a volume server from sending heartbeats to the master
```

For example:
```
$ weed shell
> fs.du /objects
block:2715	byte:  31895432	/objects
```

For most volume operations, you would need to prevent other possible concurrent operations. To do so, lock this way:
```
> lock
> volume.fix.replication
> volume.mount ...
> ...
> unlock
```

Another example: sometimes one of your volume server may go down, and a new volume server is added. Here is the command you can run to fix volumes that are under replicated:

```
# check any volume that are under replicated, and there are servers that meet the replica placement requirement
$ echo "lock; volume.fix.replication -n ; unlock" | weed shell
replicating volume 241 001 from localhost:8080 to dataNode 127.0.0.1:7823 ...

# found one, let's really do it
$ echo "lock; volume.fix.replication ; unlock" | weed shell
replicating volume 241 001 from localhost:8080 to dataNode 127.0.0.1:7823 ...

# all volumes are replicated now
$ echo "lock; volume.fix.replication -n ; unlock" | weed shell
no under replicated volumes

```

# One more trick
You can skip the "fs." prefix, for all "fs.*" commands:
```
> fs.ls
dd.dat
topics
> ls
dd.dat
topics
> ls -al topics
drwxr-xr-x   0 chrislu staff      0 /topics/.system
total 1

> fs.du
block: 515	byte:10039099653	/
> du
block: 515	byte:10039099653	/
```

# Run from Docker Image
`weed shell` commands can also be run via the docker image, allowing an operator to perform maintenance commands.

```
docker run \
  --rm \
  -e SHELL_FILER=localhost:8888 \
  -e SHELL_MASTER=localhost:9333 \
  chrislusf/seaweedfs:local \
  "shell" \
  "fs.configure -locationPrefix=/buckets/foo -volumeGrowthCount=3 -replication=002 -apply"
```

Here `shell` selects the [Docker image entrypoint](https://github.com/seaweedfs/seaweedfs/blob/master/docker/entrypoint.sh#L60-L64).

The arguments are `fs.configure -locationPrefix=/buckets/foo -volumeGrowthCount=3 -replication=002 -apply`
