Current recommended way for Hadoop to access SeaweedFS is via [SeaweedFS Hadoop Compatible File System](Hadoop-Compatible-File-System), which is the most efficient way with the client directly accessing filer for metadata and accessing volume servers for file content.

However, the downside is that you need to add a SeaweedFS jar to classpath, and change some Hadoop settings.

# HDFS Access SeaweedFS via S3 connector

The S3a connector is already included in hadoop distributions. You can use it directly.

Here is an example spark job pom.xml file, using hadoop version later than `3.3.1`: 

```
<properties>
    <maven.compiler.source>8</maven.compiler.source>
    <maven.compiler.target>8</maven.compiler.target>
    <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>
    <scala.version>2.12.11</scala.version>
    <spark.version>3.1.2</spark.version>
    <hadoop.version>3.3.1</hadoop.version>
    <spark.pom.scope>compile</spark.pom.scope>
</properties>

```
And add this in your code:
```
SparkSession spark = SparkSession.builder()
    .master("local[*]")
    .config("spark.eventLog.enabled", "false")
    .config("spark.driver.memory", "1g")
    .config("spark.executor.memory", "1g")
    .appName("SparkDemoFromS3")
    .getOrCreate();
spark.sparkContext().hadoopConfiguration().set("fs.s3a.access.key", "admin");
spark.sparkContext().hadoopConfiguration().set("fs.s3a.secret.key", "xx");
spark.sparkContext().hadoopConfiguration().set("fs.s3a.endpoint", "ip:8333");
spark.sparkContext().hadoopConfiguration().set("com.amazonaws.services.s3a.enableV4", "true");
spark.sparkContext().hadoopConfiguration().set("fs.s3a.path.style.access", "true");
spark.sparkContext().hadoopConfiguration().set("fs.s3a.connection.ssl.enabled", "false");
spark.sparkContext().hadoopConfiguration().set("fs.s3a.multiobjectdelete.enable", "false");
spark.sparkContext().hadoopConfiguration().set("fs.s3a.directory.marker.retention", "keep");
spark.sparkContext().hadoopConfiguration().set("fs.s3a.change.detection.version.required", "false");
spark.sparkContext().hadoopConfiguration().set("fs.s3a.change.detection.mode", "warn");
RDD<String> rdd = spark.sparkContext().textFile("s3a://bk002/test1.txt", 1);
System.out.println(rdd.count());
rdd.saveAsTextFile("s3a://bk002/testcc/t2");

```
