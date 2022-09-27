# Setup Hadoop Benchmark

Here are my steps. First, checkout hadoop 2.10.0 binary, untar, and cd in to the hadoop directory.
```
wget http://apache.mirrors.hoobly.com/hadoop/common/hadoop-2.10.0/hadoop-2.10.0.tar.gz
tar xvf hadoop-2.10.0.tar.gz
cd hadoop-2.10.0
```

Modify the file `./etc/hadoop/core-site.xml`

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
</configuration>
```

Then get the seaweedfs hadoop client jar.

```
cd share/hadoop/common/lib/
wget https://oss.sonatype.org/service/local/repositories/releases/content/com/github/chrislusf/seaweedfs-hadoop2-client/3.13/seaweedfs-hadoop2-client-3.13.jar
```

# TestDFSIO Benchmark

The TestDFSIO benchmark is used for measuring I/O (read/write) performance.

**However, the generated data to write is all zero. SeaweedFS automatically compress this kind of data. So this is not scientific for now.**

## TestDFSIO write tests
Start the TestDFSIO write tests:

```
bin/hadoop jar ./share/hadoop/mapreduce/hadoop-mapreduce-client-jobclient-2.10.0-tests.jar TestDFSIO -write -nrFiles 8 -size 32GB -bufferSize 8388608 -resFile /tmp/TestDFSIOwrite.txt

...
20/07/25 16:48:21 INFO fs.TestDFSIO: ----- TestDFSIO ----- : read
20/07/25 16:48:21 INFO fs.TestDFSIO:             Date & time: Sat Jul 25 16:48:21 PDT 2020
20/07/25 16:48:21 INFO fs.TestDFSIO:         Number of files: 8
20/07/25 16:48:21 INFO fs.TestDFSIO:  Total MBytes processed: 262144
20/07/25 16:48:21 INFO fs.TestDFSIO:       Throughput mb/sec: 399.16
20/07/25 16:48:21 INFO fs.TestDFSIO:  Average IO rate mb/sec: 399.34
20/07/25 16:48:21 INFO fs.TestDFSIO:   IO rate std deviation: 8.56
20/07/25 16:48:21 INFO fs.TestDFSIO:      Test exec time sec: 659.45
20/07/25 16:48:21 INFO fs.TestDFSIO:
```

## TestDFSIO read tests

Start the TestDFSIO read tests:

```
bin/hadoop jar ./share/hadoop/mapreduce/hadoop-mapreduce-client-jobclient-2.10.0-tests.jar TestDFSIO -read -nrFiles 8 -size 32GB -bufferSize 8388608 -resFile /tmp/TestDFSIOwrite.txt

...

20/07/17 15:59:38 INFO fs.TestDFSIO: ----- TestDFSIO ----- : read
20/07/17 15:59:38 INFO fs.TestDFSIO:             Date & time: Fri Jul 17 15:59:38 PDT 2020
20/07/17 15:59:38 INFO fs.TestDFSIO:         Number of files: 8
20/07/17 15:59:38 INFO fs.TestDFSIO:  Total MBytes processed: 8192
20/07/17 15:59:38 INFO fs.TestDFSIO:       Throughput mb/sec: 393.26
20/07/17 15:59:38 INFO fs.TestDFSIO:  Average IO rate mb/sec: 393.72
20/07/17 15:59:38 INFO fs.TestDFSIO:   IO rate std deviation: 13.33
20/07/17 15:59:38 INFO fs.TestDFSIO:      Test exec time sec: 22.76
20/07/17 15:59:38 INFO fs.TestDFSIO:
```

# Independent Benchmarks

## Sugon (中科曙光)
To test the performance of HDFS and SeaweedFS, we did a comparison by running 4 common spark operators, such as `count`, `group by`, `join` and `write`, for `group by` and `join` , there is a  `count` followed to act.

The basic configuration information of cluster is as follows: 

- HDFS：
  + Node number: 25
  + Total disks: 36disk  *  25node = 900disk
  + Disk capacity: 3.7T SATA
  + Total disk capacity: 3.19PB
  + Replication: 5

- SeaweedFS:

  + Node number: 6（3+3 rack）
  + Disk capacity: 3.7T SATA
  + Cluster max volume: 21500
  + Total disk capacity: 799TB
  + Replication policy: 010

Here are the details and results of our test. At the beginning of the test, we put our data to  both HDFS and SeaweedFS.  The amount of the data is 100 million records, and stored in 200 parquet files. The size of each parquet file is about 89 MB. We ran spark on yarn with 20 executors. In spark, we got two DataFrames by reading parquet from HDFS and HCFS separately, then executed `count`, `group by` and `join`  by 100 times , and `write` by 10 times, on each DataFrame.

As for `count`,  SeaweedFS's advantage  is obvious. The average time of the DataFrame from HDFS is 4.05 seconds, while SeaweedFS is only 0.659. Following is the result:

| Summary | HDFS  | SeaweedFS  |
| ------- | ----- | ----- |
| Count   | 100   | 100   |
| Mean    | 4.050 | 0.659 |
| Stddev  | 0.264 | 0.941 |
| Min     | 3.678 | 0.392 |
| Max     | 5.692 | 9.688 |



As for `write`, we wrote the DataFrame from HDFS to SeaweedFS, and  wrote the DataFrame from HCFS to SeaweedFS. Following is the result:

| Summary | HDFS    | SeaweedFS    |
| ------- | ------- | ------- |
| Count   | 10      | 10      |
| Mean    | 234.279 | 232.078 |
| Stddev  | 26.823  | 12.652  |
| Min     | 216.931 | 214.349 |
| Max     | 307.330 | 252.375 |



As for `group by`, following is the result:

| Summary | HDFS   | SeaweedFS   |
| ------- | ------ | ------ |
| Count   | 100    | 100    |
| Mean    | 14.121 | 12.515 |
| Stddev  | 1.972  | 1.255  |
| Min     | 12.879 | 11.322 |
| Max     | 32.296 | 22.573 |



As for `join`, every DataFrame join with itself on one column. Following is the result:

| Summary | HDFS   | SeaweedFS   |
| ------- | ------ | ------ |
| Count   | 100    | 100    |
| Mean    | 25.684 | 23.897 |
| Stddev  | 0.934  | 1.381  |
| Min     | 24.006 | 22.275 |
| Max     | 30.991 | 30.279 |
