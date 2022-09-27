# Installation for Spark
Follow instructions on spark doc: 
* https://spark.apache.org/docs/latest/configuration.html#inheriting-hadoop-cluster-configuration
* https://spark.apache.org/docs/latest/configuration.html#custom-hadoophive-configuration

## installation inheriting from Hadoop cluster configuration

Inheriting from Hadoop cluster configuration should be the easiest way. 

To make these files visible to Spark, set HADOOP_CONF_DIR in $SPARK_HOME/conf/spark-env.sh to a location containing the configuration file `core-site.xml`, usually `/etc/hadoop/conf`

## installation not inheriting from Hadoop cluster configuration

Copy the seaweedfs-hadoop2-client-3.13.jar to all executor machines.

Add the following to spark/conf/spark-defaults.conf on every node running Spark
```
spark.driver.extraClassPath=/path/to/seaweedfs-hadoop2-client-3.13.jar
spark.executor.extraClassPath=/path/to/seaweedfs-hadoop2-client-3.13.jar
```

And modify the configuration at runtime:

```
./bin/spark-submit \ 
  --name "My app" \ 
  --master local[4] \  
  --conf spark.eventLog.enabled=false \ 
  --conf "spark.executor.extraJavaOptions=-XX:+PrintGCDetails -XX:+PrintGCTimeStamps" \ 
  --conf spark.hadoop.fs.seaweedfs.impl=seaweed.hdfs.SeaweedFileSystem \ 
  --conf spark.hadoop.fs.defaultFS=seaweedfs://localhost:8888 \ 
  myApp.jar
```

# Example

  1. change the spark-defaults.conf

```
spark.driver.extraClassPath=/Users/chris/go/src/github.com/seaweedfs/seaweedfs/other/java/hdfs2/target/seaweedfs-hadoop2-client-3.13.jar
spark.executor.extraClassPath=/Users/chris/go/src/github.com/seaweedfs/seaweedfs/other/java/hdfs2/target/seaweedfs-hadoop2-client-3.13.jar
spark.hadoop.fs.seaweedfs.impl=seaweed.hdfs.SeaweedFileSystem
```

  2. create the spark history folder
```
$ curl -X POST http://192.168.2.3:8888/spark2-history/
```
  3. Run a spark job
```
$ bin/spark-submit --name spark-pi \
--class org.apache.spark.examples.SparkPi \
--conf spark.jars.ivy=/tmp/.ivy \
--conf spark.eventLog.enabled=true \
--conf spark.hadoop.fs.defaultFS=seaweedfs://192.168.2.3:8888 \
--conf spark.eventLog.dir=seaweedfs://192.168.2.3:8888/spark2-history/ \
file:///usr/local/spark/examples/jars/spark-examples_2.12-3.0.0.jar

```


# A Full Example
Here is my local example switching everything to SeaweedFS. In the `/usr/local/spark/conf/spark-defaults.conf` file, 

  1. this is my local `/usr/local/spark/conf/spark-defaults.conf`
```
spark.eventLog.enabled=true
spark.sql.hive.convertMetastoreOrc=true
spark.yarn.queue=default
spark.master=local
spark.history.ui.port=18081
spark.history.fs.cleaner.interval=7d
spark.sql.statistics.fallBackToHdfs=true
spark.yarn.historyServer.address=master:18081
spark.sql.orc.filterPushdown=true
spark.history.provider=org.apache.spark.deploy.history.FsHistoryProvider
spark.history.fs.cleaner.maxAge=90d
spark.sql.orc.impl=native
spark.history.fs.cleaner.enabled=true

spark.history.fs.logDirectory=seaweedfs://localhost:8888/spark2-history/
spark.eventLog.dir=seaweedfs://localhost:8888/spark2-history/

spark.driver.extraClassPath=/Users/chris/go/src/github.com/seaweedfs/seaweedfs/other/java/hdfs2/target/seaweedfs-hadoop2-client-3.13.jar
spark.executor.extraClassPath=/Users/chris/go/src/github.com/seaweedfs/seaweedfs/other/java/hdfs2/target/seaweedfs-hadoop2-client-3.13.jar
spark.hadoop.fs.seaweedfs.impl=seaweed.hdfs.SeaweedFileSystem
spark.hadoop.fs.defaultFS=seaweedfs://localhost:8888
```
  2. create the spark history folder
```
$ curl -X POST http://192.168.2.4:8888/spark2-history/
```
  3. Run a spark shell
```
$ bin/spark-shell
20/10/18 14:11:44 WARN Utils: Set SPARK_LOCAL_IP if you need to bind to another address
20/10/18 14:12:15 WARN NativeCodeLoader: Unable to load native-hadoop library for your platform... using builtin-java classes where applicable
Using Spark's default log4j profile: org/apache/spark/log4j-defaults.properties
Setting default log level to "WARN".
To adjust logging level use sc.setLogLevel(newLevel). For SparkR, use setLogLevel(newLevel).
Spark context Web UI available at http://192.168.2.4:4040
Spark context available as 'sc' (master = local, app id = local-1603055539864).
Spark session available as 'spark'.
Welcome to
      ____              __
     / __/__  ___ _____/ /__
    _\ \/ _ \/ _ `/ __/  '_/
   /___/ .__/\_,_/_/ /_/\_\   version 3.0.0
      /_/

Using Scala version 2.12.10 (Java HotSpot(TM) 64-Bit Server VM, Java 1.8.0_202)
Type in expressions to have them evaluated.
Type :help for more information.

scala> sc.textFile("/buckets/large/ttt.txt").count
res0: Long = 9374

```
