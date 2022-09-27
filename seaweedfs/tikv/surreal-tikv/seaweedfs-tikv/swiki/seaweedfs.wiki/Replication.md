SeaweedFS can support replication. The replication is implemented not on file level, but on volume level.

## How to use

Basically, the way it works is:

1. start weed master, and optionally specify the default replication type

   ```bash
   # 001 means for each file a replica will be created in the same rack
   ./weed master -defaultReplication=001
   ```

2. start volume servers as this:

   ```bash
   ./weed volume -port=8081 -dir=/tmp/1 -max=100 -mserver="master_address:9333" -dataCenter=dc1 -rack=rack1
   ./weed volume -port=8082 -dir=/tmp/2 -max=100 -mserver="master_address:9333" -dataCenter=dc1 -rack=rack1
   ```
On another rack,
   ```bash
   ./weed volume -port=8081 -dir=/tmp/1 -max=100 -mserver="master_address:9333" -dataCenter=dc1 -rack=rack2
   ./weed volume -port=8082 -dir=/tmp/2 -max=100 -mserver="master_address:9333" -dataCenter=dc1 -rack=rack2
   ```

No change to Submitting, Reading, and Deleting files.

## The meaning of replication type

*Note: This subject to change.*

Value | Meaning
---|---
000 | no replication, just one copy
001 | replicate once on the same rack
010 | replicate once on a different rack in the same data center
100 | replicate once on a different data center
200 | replicate twice on two other different data center
110 | replicate once on a different rack, and once on a different data center
... | ...

So if the replication type is xyz

Column | Meaning
---|---
**x** | number of replica in other data centers
**y** | number of replica in other racks in the same data center
**z** | number of replica in other servers in the same rack

x,y,z each can be 0, 1, or 2. So there are 9 possible replication types, and can be easily extended. 
Each replication type will physically create x+y+z+1 copies of volume data files.


## Allocate File Key on specific data center

Now when requesting a file key, an optional "dataCenter" parameter can limit the assigned volume to the specific data center. For example, this specify

```bash
http://localhost:9333/dir/assign?dataCenter=dc1
```

## Write and Read

For consistent read and write, a quorum `W + R > N` is required. In SeaweedFS, `W = N` and `R = 1`.

In plain words, all the writes are strongly consistent and all N replica should be successful. If one of the replica fails to write, the whole write request will fail. This makes read request fast since it does not need to check and compare other replicas.

For failed write request, there might be some replicas written. These replica would be deleted. Since volumes are append only, the physical volume size may deviate over time.

### write-path

When a client do a write request, here follows the work-flow:
  1. a client sends a specific replication to the master in order to get assigned a fid
  2. the master receives the assign request, depending of the replication, it chooses volume servers that will handle them
  3. the client sends the write request to one of the volume servers and wait for the ACK
  4. the volume server persist the file and also replicated the file if needed.
  5. If everything is fine, the client get a OK response.

When a write is made to the filer, there is an additional step before step 1. and after 5. and the filer acts a client in the step 1 to 5.

## Fix replication

If one replica is missing, there are no automatic repair right away. This is to prevent over replication due to transient volume sever failures or disconnections. In stead, the volume will just become readonly. For any new writes, just assign a different file id to a different volume.

To repair the missing replicas, you can use `volume.fix.replication` in `weed shell`.

### Replicate without deleting
In certain circumstances—like adding/removing/altering replication settings of volumes or servers—the best strategy is to only repair under-replicated volumes and not delete any while working on volume and server modifications, in this citation use the flag `noDelete`: 

`volume.fix.replication -noDelete`

 After all replications and modifications are finished, desired replication consensus can then be obtained by running `volume.fix.replication` without the 'noDelete' flag.

## Change replication

In `weed shell`, you can change a volume replication setting via `volume.configure.replication`. After that, the volume will become readonly since the replication setting is not matched. You will also need to run 'volume.fix.replication` to create missing replicas.

