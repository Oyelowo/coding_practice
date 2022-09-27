When running large clusters, it is common to add more volume servers, or some volume servers are down, or some volume servers are replaced. These topology changes can cause missing volume replicas, or unbalanced number of volumes on volume servers.

## Optimize volumes
See [[Optimization]] page on how to optimize for concurrent writes and concurrent reads.

## Configure volume management scripts
Run `weed scaffold -config=master` will generate `master.toml` which has sections as these.

```
[master.maintenance]
# periodically run these scripts are the same as running them from 'weed shell'
scripts = """
  ec.encode -fullPercent=95 -quietFor=1h
  ec.rebuild -force
  ec.balance -force
  volume.balance -force
  volume.fix.replication
"""
sleep_minutes = 17          # sleep minutes between each script execution

```

If the `master.toml` has the above configuration, the scripts will run every 17 minutes. It has the same effect as running from `weed shell` directly. 

## Fix missing volumes
When running large clusters, it is common that some volume servers are down. If a volume is replicated and one replica is missing, the volume will be marked as readonly.

One way to fix is to find one healthy copy and replicated to other servers, to meet the replication requirement. This volume id will be marked as writable.

In `weed shell`, the command `volume.fix.replication` will do exactly that, automating the replication fixing process. You can start a crontab job to periodically run `volume.fix.replication` to ensure the system health.

## Balance volumes
When running large clusters, it is common to add more volume severs, or some volume servers are down, or some volume servers are replaced. These topology changes can cause unbalanced number of volumes on volume servers.

In `weed shell`, the command `volume.balance` will generate a balancing plan, and `volume.balance -force` will execute the balancing plan and move the actual volumes.

The balancing plan will try to evenly spread the number of writable and readonly 

```
	For each type of volume server (different max volume count limit){
		for each collection {
			balanceWritableVolumes()
			balanceReadOnlyVolumes()
		}
	}

	func balanceWritableVolumes(){
		idealWritableVolumes = totalWritableVolumes / numVolumeServers
		for {
			sort all volume servers ordered by the number of local writable volumes
			pick the volume server A with the lowest number of writable volumes x
			pick the volume server B with the highest number of writable volumes y
			if y > idealWritableVolumes and x+1 <= idealWritableVolumes {
				if B has a writable volume id v that A does not have {
					move writable volume v from A to B
				}
			}
		}
	}
	func balanceReadOnlyVolumes(){
		//similar to balanceWritableVolumes
	}


```