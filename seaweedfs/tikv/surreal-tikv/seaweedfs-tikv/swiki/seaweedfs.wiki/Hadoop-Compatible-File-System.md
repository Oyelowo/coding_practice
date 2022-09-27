HDFS is optimized for large files. The scalability of the single HDFS namenode is limited by the number of files. It is hard for HDFS to store lots of small files.

SeaweedFS excels on small files and has no issue to store large files. Now it is possible to enable Hadoop jobs to read from and write to SeaweedFS.

# Build SeaweedFS Hadoop Client Jar
```
$cd $GOPATH/src/github.com/seaweedfs/seaweedfs/other/java/client
$ mvn install

# build for hadoop2
$cd $GOPATH/src/github.com/seaweedfs/seaweedfs/other/java/hdfs2
$ mvn package
$ ls -al target/seaweedfs-hadoop2-client-3.13.jar

# build for hadoop3
$cd $GOPATH/src/github.com/seaweedfs/seaweedfs/other/java/hdfs3
$ mvn package
$ ls -al target/seaweedfs-hadoop3-client-3.13.jar

```
Maven
```
<dependency>
  <groupId>com.github.chrislusf</groupId>
  <artifactId>seaweedfs-hadoop3-client</artifactId>
  <version>3.13</version>
</dependency>

or 

<dependency>
  <groupId>com.github.chrislusf</groupId>
  <artifactId>seaweedfs-hadoop2-client</artifactId>
  <version>3.13</version>
</dependency>

```

Or you can download the latest version from MavenCentral
* https://mvnrepository.com/artifact/com.github.chrislusf/seaweedfs-hadoop2-client
  * [seaweedfs-hadoop2-client-3.13.jar](https://oss.sonatype.org/service/local/repositories/releases/content/com/github/chrislusf/seaweedfs-hadoop2-client/3.13/seaweedfs-hadoop2-client-3.13.jar)
* https://mvnrepository.com/artifact/com.github.chrislusf/seaweedfs-hadoop3-client
  * [seaweedfs-hadoop3-client-3.13.jar](https://oss.sonatype.org/service/local/repositories/releases/content/com/github/chrislusf/seaweedfs-hadoop3-client/3.13/seaweedfs-hadoop3-client-3.13.jar)

# Test SeaweedFS on Hadoop

Suppose you are getting a new Hadoop installation. Here are the minimum steps to get SeaweedFS to run.

You would need to start a weed filer first, build the seaweedfs-hadoop2-client-3.13.jar 
or seaweedfs-hadoop3-client-3.13.jar, and do the following:

```
# optionally adjust hadoop memory allocation
$ export HADOOP_CLIENT_OPTS="-Xmx4g"

$ cd ${HADOOP_HOME}
# create etc/hadoop/mapred-site.xml, just to satisfy hdfs dfs. skip this if the file already exists.
$ echo "<configuration></configuration>" > etc/hadoop/mapred-site.xml

# on hadoop2
$ bin/hdfs dfs -Dfs.defaultFS=seaweedfs://localhost:8888 \
               -Dfs.seaweedfs.impl=seaweed.hdfs.SeaweedFileSystem \
               -libjars ./seaweedfs-hadoop2-client-3.13.jar \
               -ls /
# or on hadoop3
$ bin/hdfs dfs -Dfs.defaultFS=seaweedfs://localhost:8888 \
               -Dfs.seaweedfs.impl=seaweed.hdfs.SeaweedFileSystem \
               -libjars ./seaweedfs-hadoop3-client-3.13.jar \
               -ls /

```
Both reads and writes are working fine.

# Installation for Hadoop
* Configure Hadoop to use SeaweedFS in `etc/hadoop/conf/core-site.xml`. `core-site.xml` resides on each node in the Hadoop cluster. You must add the same properties to each instance of `core-site.xml`. There are several properties to modify:
    1. `fs.seaweedfs.impl`: This property defines the Seaweed HCFS implementation classes that are contained in the SeaweedFS HDFS client JAR. It is required.
    1. `fs.defaultFS`: This property defines the default file system URI to use. It is optional if a path always has prefix `seaweedfs://localhost:8888`.
    1. `fs.AbstractFileSystem.seaweedfs.impl`: Add the SeaweedFS implementation of Hadoop AbstractFileSystem to delegates to the existing SeaweedFS FileSystem and is only necessary for use with Hadoop 3.x.
    1. `fs.seaweed.buffer.size`: Optionally change the default buffer size 4194304 to a larger number. It will be the default chunk size.
    1. `fs.seaweed.volume.server.access`: `[direct|publicUrl|filerProxy]` Optionally access volume servers via their publicUrl settings, or use filer as a proxy. This is useful when volume servers are inside a cluster and not directly accessible.

```
<configuration>
    <property>
        <name>fs.seaweedfs.impl</name>
        <value>seaweed.hdfs.SeaweedFileSystem</value>
    </property>
    <property>
        <name>fs.defaultFS</name>
        <value>seaweedfs://localhost:8888</value>
    </property>
    <property>
       <name>fs.AbstractFileSystem.seaweedfs.impl</name>
       <value>seaweed.hdfs.SeaweedAbstractFileSystem</value>
    </property>
    <property>
       <name>fs.seaweed.buffer.size</name>
       <value>4194304</value>
    </property>
    <property>
       <name>fs.seaweed.volume.server.access</name>
       <!-- [direct|publicUrl|filerProxy] -->
       <value>direct</value>
    </property>
</configuration>
```
* Deploy the SeaweedFS HDFS client jar
```
# Run the classpath command to get the list of directories in the classpath
$ bin/hadoop classpath

# Copy SeaweedFS HDFS client jar to one of the folders
$ cd ${HADOOP_HOME}
# for hadoop2
$ cp ./seaweedfs-hadoop2-client-3.13.jar share/hadoop/common/lib/
# or for hadoop3
$ cp ./seaweedfs-hadoop3-client-3.13.jar share/hadoop/common/lib/
```

Now you can do this:
```
$ cd ${HADOOP_HOME}
$ bin/hdfs dfs -ls /

# if you did not set fs.defaultFS in etc/hadoop/core-site.xml
# or you want to access a different SeaweedFS filer
$ bin/hdfs dfs -ls seaweedfs://localhost:8888/
```



# Supported HDFS Operations
```
    bin/hdfs dfs -appendToFile README.txt /weedfs/weedfs.txt
    bin/hdfs dfs -cat /weedfs/weedfs.txt
    bin/hdfs dfs -rm -r /uber
    bin/hdfs dfs -chown -R chris:chris /weedfs
    bin/hdfs dfs -chmod -R 755 /weedfs
    bin/hdfs dfs -copyFromLocal README.txt /weedfs/README.txt.2
    bin/hdfs dfs -copyToLocal /weedfs/README.txt.2 .
    bin/hdfs dfs -count /weedfs/README.txt.2
    bin/hdfs dfs -cp /weedfs/README.txt.2 /weedfs/README.txt.3
    bin/hdfs dfs -du -h /weedfs
    bin/hdfs dfs -find /weedfs -name "*.txt" -print
    bin/hdfs dfs -get /weedfs/weedfs.txt
    bin/hdfs dfs -getfacl /weedfs
    bin/hdfs dfs -getmerge -nl /weedfs w.txt
    bin/hdfs dfs -ls /
    bin/hdfs dfs -mkdir /tmp
    bin/hdfs dfs -mkdir -p /tmp/x/y
    bin/hdfs dfs -moveFromLocal README.txt.2 /tmp/x/
    bin/hdfs dfs -mv /tmp/x/y/README.txt.2 /tmp/x/y/README.txt.3
    bin/hdfs dfs -mv /tmp/x /tmp/z
    bin/hdfs dfs -put README.txt /tmp/z/y/
    bin/hdfs dfs -rm /tmp/z/y/*
    bin/hdfs dfs -rmdir /tmp/z/y
    bin/hdfs dfs -stat /weedfs
    bin/hdfs dfs -tail /weedfs/weedfs.txt
    bin/hdfs dfs -test -f /weedfs/weedfs.txt
    bin/hdfs dfs -text /weedfs/weedfs.txt
    bin/hdfs dfs -touchz /weedfs/weedfs.txtx
```

## Operations Plan to Support
```
  getfattr
  setfacl
  setfattr
  truncate
  createSnapshot
  deleteSnapshot
  renameSnapshot
  setrep
```


# Notes
## Atomicity
SeaweedFS satisfies the HCFS [requirements](https://hadoop.apache.org/docs/r2.7.2/hadoop-project-dist/hadoop-common/filesystem/introduction.html) that the following operations to be atomic, when using MySql/Postgres/Sqlite database transactions.

1. Creating a file. If the overwrite parameter is false, the check and creation MUST be atomic.
1. Deleting a file.
1. Renaming a file.
1. Renaming a directory.
1. Creating a single directory with mkdir().

Among these, except file or directory renaming, the following operations are all atomic for any filer store.
1. Creating a file
1. Deleting a file
1. Creating a single directory with mkdir().

## No native shared libraries
The SeaweedFS hadoop client is a pure java library. There are no native libraries to install if you already have Hadoop running.

This is different from many other HCFS options. If native shared libraries are needed, these libraries need to be install on all hadoop nodes. This is quite some task.

## Shaded Fat Jar
One of the headache with complicated Java systems is the jar runtime dependency problem, which is resolved by Go's build time dependency resolution. For this SeaweedFS hadoop client, the required jars are mostly shaded and packaged as one fat jar, so there are no extra jar files needed.

## Note
### use `-Djava.net.preferIPv4Stack=true` if possible, see https://github.com/netty/netty/issues/6454
error message:
```
Failed construction of Master: class org.apache.hadoop.hbase.master.HMasterCommandLine$LocalHMasterConnection refused: localhost/0:0:0:0:0:0:0:1:18888
```

### See [[Security-Configuration#for-java-grpc]] if you enabled gRpc security.
```
Failed construction of Master: class org.apache.hadoop.hbase.master.HMasterCommandLine$LocalHMasternot an SSL/TLS record: 000006040000000000000500004000
```
