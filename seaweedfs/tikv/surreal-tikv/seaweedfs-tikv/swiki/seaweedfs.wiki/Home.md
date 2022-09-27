# Welcome to the SeaweedFS wiki!

Here is the white paper for [SeaweedFS Architecture.pdf](SeaweedFS_Architecture.pdf)

![SeaweedFS Architecture](https://raw.githubusercontent.com/seaweedfs/seaweedfs/master/note/SeaweedFS_Architecture.png)

# Make Cloud Storage Cheaper and Faster!

To reduce API cost and transmission cost, and minimize read-write latency, you can mount your cloud storage to a SeaweedFS local cluster with [[SeaweedFS Cloud Drive|Cloud Drive Architecture]].

* Read and write with local network speed. 
* Asynchronously propagate local updates to the cloud storage.
* Works well with existing cloud ecosystems.

![SeaweedFS Remote Storage](https://raw.githubusercontent.com/seaweedfs/seaweedfs/master/note/SeaweedFS_RemoteMount.png)
