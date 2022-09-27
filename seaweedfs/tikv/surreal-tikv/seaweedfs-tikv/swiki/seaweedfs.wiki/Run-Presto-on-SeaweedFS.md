# Installation for Presto
The installation steps are divided into 2 steps:
## install Hive Metastore
### Follow instructions for installation of Hive Metastore
* https://cwiki.apache.org/confluence/display/Hive/AdminManual+Metastore+Administration

### Configure Hive Metastore to support SeaweedFS
1. Copy the seaweedfs-hadoop2-client-3.13.jar to hive lib directory,for example:
```
cp seaweedfs-hadoop2-client-3.13.jar /opt/hadoop/share/hadoop/common/lib/
cp seaweedfs-hadoop2-client-3.13.jar /opt/hive-metastore/lib/
```
2. Modify core-site.xml  
modify core-site.xml to support SeaweedFS, 30888 is the filer port
```
<configuration>
	<property>
	    <name>fs.defaultFS</name>
	    <value>seaweedfs://10.0.100.51:30888</value>
	</property>
	<property>
	    <name>fs.seaweedfs.impl</name>
	    <value>seaweed.hdfs.SeaweedFileSystem</value>
	</property>
	<property>
	   <name>fs.AbstractFileSystem.seaweedfs.impl</name>
	   <value>seaweed.hdfs.SeaweedAbstractFileSystem</value>
	</property>
	<property>
	   <name>fs.seaweed.buffer.size</name>
	   <value>4194304</value>
	</property>
</configuration>
```
3. Modify hive-site.xml  
modify hive-site.xml to support SeaweedFS, need to manually create the /presto/warehouse directory in Filer  
metastore.thrift.port is the access port exposed by the Hive Metadata service itself
```
	<property>
	    <name>metastore.warehouse.dir</name>
	    <value>seaweedfs://10.0.100.51:30888/presto/warehouse</value>
	</property>
	<property>
	    <name>metastore.thrift.port</name>
	    <value>9850</value>
	</property>
```

## install Presto
Follow instructions for installation of Presto:
* https://prestosql.io/docs/current/installation/deployment.html
### Configure Presto to support SeaweedFS
1. Copy the seaweedfs-hadoop2-client-3.13.jar to Presto directory,for example:
```
cp seaweedfs-hadoop2-client-3.13.jar /opt/presto-server-347/plugin/hive-hadoop2/
```
2. Modify core-site.xml

modify /opt/presto-server-347/etc/catalog/core-site.xml to support SeaweedFS, 30888 is the filer port
```
<configuration>
    <property>
        <name>fs.defaultFS</name>
        <value>seaweedfs://10.0.100.51:30888</value>
    </property>
    <property>
        <name>fs.seaweedfs.impl</name>
        <value>seaweed.hdfs.SeaweedFileSystem</value>
    </property>
    <property>
       <name>fs.AbstractFileSystem.seaweedfs.impl</name>
       <value>seaweed.hdfs.SeaweedAbstractFileSystem</value>
    </property>
    <property>
       <name>fs.seaweed.buffer.size</name>
       <value>4194304</value>
    </property>
</configuration>

```
3. Modify hive.properties  
hive.metastore.uri is the service address of the previously deployed Hive Metastore  
hive.config.resources points to the core-site.xml above
```
connector.name=hive-hadoop2
hive.metastore.uri=thrift://10.0.100.51:9850
hive.allow-drop-table=true
hive.max-partitions-per-scan=1000000
hive.compression-codec=NONE
hive.config.resources=/opt/presto-server-347/etc/catalog/core-site.xml
```
4. Modify config.properties  
The default port of presto is 8080  
If you want to modify the default port of the Presto service, you can modify /opt/presto-server-347/etc/config.properties  
Need to modify the ports of http-server.http.port and discovery.uri

```
coordinator=true
node-scheduler.include-coordinator=true
http-server.http.port=8080
query.max-memory=200GB
query.max-memory-per-node=8GB
query.max-total-memory-per-node=10GB
query.max-stage-count=200
task.writer-count=4
discovery-server.enabled=true
discovery.uri=http://10.0.100.51:8080
```

# Using Examples
1. Connect to Presto and create a table boshen  
--server is the ip and port of the Presto service
```
[root@cluster9 ~]# ./presto --server 10.0.100.51:8080--catalog hive --schema default
presto:default> create table boshen(name varchar);
CREATE TABLE
presto:default> 
```
2. Query whether the boshen directory has been generated in 10.0.10.51:30888/presto/warehouse
```
[root@cluster9 ~]# curl -H "Accept: application/json"  http://10.0.100.51:30888/presto/warehouse/?pretty=y
{
  "Path": "presto/warehouse",
  "Entries": [
    {
      "FullPath": "/presto/warehouse/boshen",
      "Mtime": "2020-12-02T20:29:08+08:00",
      "Crtime": "2020-12-02T20:29:08+08:00",
      "Mode": 2147484159,
      "Uid": 0,
      "Gid": 0,
      "Mime": "",
      "Replication": "",
      "Collection": "",
      "TtlSec": 0,
      "UserName": "root",
      "GroupNames": [
        "root"
      ],
      "SymlinkTarget": "",
      "Md5": null,
      "FileSize": 0,
      "Extended": null,
      "HardLinkId": null,
      "HardLinkCounter": 0
    }
  ],
  "Limit": 100,
  "LastFileName": "boshen",
  "ShouldDisplayLoadMore": false
}
```
