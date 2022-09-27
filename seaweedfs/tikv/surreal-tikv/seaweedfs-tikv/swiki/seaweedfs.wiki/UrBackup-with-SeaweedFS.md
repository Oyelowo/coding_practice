This just mount SeaweedFS cluster to a local folder, and UrBackup saves data into it. That is it! You will get scalable storage, or use [[Cloud Tier]] to get unlimited storage.

## Start SeaweedFS cluster and mount to a local directory

First, start SeaweedFS master, volume server, and filer. Or just run `weed server -dir=<storage_dir> -filer`

Second, start SeaweedFS FUSE mount, `weed mount -dir=<mount_to_dir> -filer=<filer_host_port>`

## Prepare Folders

```
$ mkdir <mount_to_dir>/urbackup
$ mkdir <mount_to_dir>/urdatabase
```
## Start the UrBackup service

Here is an example of using docker to run the UrBackup server.

```
$ docker run -it \
--name urbackup \
-v /Users/chris/tmp/mm/urbackup:/backups \
-v /Users/chris/tmp/mm/urdatabase:/var/urbackup \
-p 55413-55415:55413-55415 \
-p 35623:35623/udp \
uroni/urbackup-server:latest
```

Now you can visit http://localhost:55414/ to continue the configuration.
