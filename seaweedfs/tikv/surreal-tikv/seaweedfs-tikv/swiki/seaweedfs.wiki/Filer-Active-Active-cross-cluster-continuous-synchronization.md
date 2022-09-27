# What can it do?

If you have two or more SeaweedFS clusters using filers, now you can asynchronously replicate changes to each other.

It can run either in Active-Active mode or Active-Passive mode.

# Usage

From any computer that can access filer and volume servers in both SeaweedFS clusters, run one of these commands:

```
# synchronize each cluster
weed filer.sync -a <filer1_host>:<filer1_port> -b <filer2_host>:<filer2_port>

# synchronize only specific folders:
weed filer.sync -a <filer1_host>:<filer1_port> -b <filer2_host>:<filer2_port> -a.path /filer1/path1 -b.path /filer2/path2

# active-passive mode, replicate all changes in filer1 to filer2
weed filer.sync -a <filer1_host>:<filer1_port> -b <filer2_host>:<filer2_port> -isActivePassive

```

At the beginning, it will bootstrap from the beginning of time, or resume from the last replication checkpoint. Later, it will just run continuously and persist checkpoints periodically.

## How it works?

Each filer has its own local change logs. `weed filer.sync` will read the logs and replay them in the other cluster.

`weed filer.sync` remembers each filer's "signature" and replication checkpoints. So you can stop `weed filer.sync` and start it later safely.

Also, the "signature" will ensure same change will only be applied once in one filer. Active-Active synchronization would not cause multiple ping-pong changes for one file update.


## More clusters?

If there are 3 or more clusters, you can choose fully connected setup or chained setup, or any more complicated topology.

In a sense, you can mix and match to setup filer synchronization as you wish.

### Fully Connected Setup?

It is tempting to create a fully connected network topology. E.g., run one `weed filer.sync` for each pair of clusters. The fully connected topology may seem to be able to provide redundancy in case of network failures.

```
cluster1 <-- filer.sync --> cluster2
cluster2 <-- filer.sync --> cluster3
cluster3 <-- filer.sync --> cluster1
```

However, this topology has a loop. 

Every filer will leave a signature on each message. The `filer.sync` use the signatures to avoid processing the same message twice. But for any node within a loop, the same message can come from two difference neighbors. So this mechanism could not help to identify the duplicated the messages.

Because most metadata messages are idempotent, the network loop is not efficient but still works OK.

But for directory renaming, the execution order matters. So the loop should be avoided, or the directories will be inconsistent.

### Chained Setup
```
cluster1 <-- filer.sync --> cluster2 <-- filer.sync --> cluster3
```

### One-Master-Multiple-Slaves

With `weed filer.sync -isActivePassive` configured, nothing stops you from setting up multiple following clusters.

```
cluster1  -- filer.sync --> cluster2
cluster1  -- filer.sync --> cluster3
cluster1  -- filer.sync --> cluster4
```

### Multiple-Master-Multiple-Slaves

This should also work, multiple active-active clusters, with chained following clusters.

```

cluster1  <-- filer.sync --> cluster2 -- filer.sync --> cluster3 -- filer.sync --> cluster4
                                                             |
                                                             +----- filer.sync --> cluster5
```

### Different Sync strategy for different folders

Here the first folder is active-active synchronized on 2 clusters, but the `/home/public` on `cluster1` is single directionally synchronized to `cluster2`

```
cluster1:/home/chris <-- filer.sync --> cluster2:/Users/chris

cluster1:/home/public -- filer.sync --> cluster2:/home/www/public

``` 

## Filer Proxy 

By default, `filer.sync` will upload files directly to volume servers. This is the most efficient way to avoid extra hops and distribute the network traffic.

However, it uses volume server IP addresses configured for the local cluster. `filer.sync` is usually cross network. These IPs may not be accessible to the `filer.sync` because of network configurations (for example cluster1 and cluster2 are on different hosting providers). In this case, it could be useful to use the `filerProxy` option to make `filer.sync` does all the transfers through the filer. In order to enable this option, `-a.filerProxy` or/and `-b.filerProxy` can be added to the `weed filer.sync` process starting command line.

## Debug log
To see all detail of transfers executed by `filer.sync`, options `-a.debug` or/and `-b.debug` can be added to the `weed filer.sync` process starting command line.

## Limitations

This should be fairly scalable. However, it is limited by network bandwidth and latency. So even though changes are received within milliseconds and replayed right away, there would be data discrepancies if a file is changed quickly in two distant data centers.

For large clusters, the rate of change may be so high that the replication can not catch up. You may want to only synchronize a specific folder to reduce the work load.

