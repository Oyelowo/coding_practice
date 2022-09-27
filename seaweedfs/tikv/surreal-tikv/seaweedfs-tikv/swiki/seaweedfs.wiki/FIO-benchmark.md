# FIO Benchmark

Here is the result of using tool [fio](https://github.com/axboe/fio) running on my personal laptop. Just for reference. Please benchmark with your own hardware.

* The server and mount are restarted before each run.

# Prepare A file

Go to any mounted directory:

```
fio --randrepeat=1 --name=test --filename=fiotest --bs=128k --iodepth=1 --size=10G

```

# Random Read

Random Read with 4KB, 128KB and 2MB block sizes, with direct IO enabled.

```
$ fio --randrepeat=1 --name=test --filename=fiotest --bs=4k --iodepth=1 --readwrite=randread --size=10G -direct=1
test: (g=0): rw=randread, bs=(R) 4096B-4096B, (W) 4096B-4096B, (T) 4096B-4096B, ioengine=psync, iodepth=1
fio-3.28
Starting 1 process
Jobs: 1 (f=1): [r(1)][100.0%][r=5972KiB/s][r=1493 IOPS][eta 00m:00s]
test: (groupid=0, jobs=1): err= 0: pid=17278: Tue Dec 28 13:07:47 2021
  read: IOPS=1376, BW=5504KiB/s (5636kB/s)(10.0GiB/1905070msec)
    clat (nsec): min=0, max=67454k, avg=724310.61, stdev=74651.05
     lat (nsec): min=0, max=67454k, avg=724502.93, stdev=74659.30
    clat percentiles (usec):
     |  1.00th=[  660],  5.00th=[  676], 10.00th=[  685], 20.00th=[  701],
     | 30.00th=[  701], 40.00th=[  709], 50.00th=[  717], 60.00th=[  725],
     | 70.00th=[  734], 80.00th=[  742], 90.00th=[  758], 95.00th=[  775],
     | 99.00th=[  889], 99.50th=[ 1139], 99.90th=[ 1483], 99.95th=[ 1663],
     | 99.99th=[ 2278]
   bw (  KiB/s): min= 4968, max= 6128, per=100.00%, avg=5508.64, stdev=50.73, samples=3793
   iops        : min= 1242, max= 1532, avg=1376.89, stdev=12.68, samples=3793
  lat (nsec)   : 2=0.01%
  lat (usec)   : 100=0.01%, 750=87.03%, 1000=12.30%
  lat (msec)   : 2=0.64%, 4=0.03%, 10=0.01%, 100=0.01%
  cpu          : usr=0.48%, sys=1.20%, ctx=2631575, majf=5, minf=29
  IO depths    : 1=100.0%, 2=0.0%, 4=0.0%, 8=0.0%, 16=0.0%, 32=0.0%, >=64=0.0%
     submit    : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     complete  : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     issued rwts: total=2621440,0,0,0 short=0,0,0,0 dropped=0,0,0,0
     latency   : target=0, window=0, percentile=100.00%, depth=1

Run status group 0 (all jobs):
   READ: bw=5504KiB/s (5636kB/s), 5504KiB/s-5504KiB/s (5636kB/s-5636kB/s), io=10.0GiB (10.7GB), run=1905070-1905070msec



$ fio --randrepeat=1 --name=test --filename=fiotest --bs=128k --iodepth=1 --readwrite=randread --size=10G -direct=1
test: (g=0): rw=randread, bs=(R) 128KiB-128KiB, (W) 128KiB-128KiB, (T) 128KiB-128KiB, ioengine=psync, iodepth=1
fio-3.28
Starting 1 process
Jobs: 1 (f=1): [r(1)][98.7%][r=139MiB/s][r=1110 IOPS][eta 00m:01s]
test: (groupid=0, jobs=1): err= 0: pid=21569: Tue Dec 28 16:59:29 2021
  read: IOPS=1079, BW=135MiB/s (141MB/s)(10.0GiB/75889msec)
    clat (usec): min=790, max=6740, avg=923.86, stdev=111.89
     lat (usec): min=790, max=6741, avg=924.06, stdev=111.91
    clat percentiles (usec):
     |  1.00th=[  832],  5.00th=[  848], 10.00th=[  857], 20.00th=[  873],
     | 30.00th=[  881], 40.00th=[  889], 50.00th=[  898], 60.00th=[  906],
     | 70.00th=[  922], 80.00th=[  947], 90.00th=[ 1004], 95.00th=[ 1074],
     | 99.00th=[ 1401], 99.50th=[ 1565], 99.90th=[ 2311], 99.95th=[ 2474],
     | 99.99th=[ 2868]
   bw (  KiB/s): min=115785, max=144000, per=100.00%, avg=138291.78, stdev=4500.30, samples=151
   iops        : min=  904, max= 1125, avg=1080.05, stdev=35.17, samples=151
  lat (usec)   : 1000=89.92%
  lat (msec)   : 2=9.94%, 4=0.14%, 10=0.01%
  cpu          : usr=0.39%, sys=10.24%, ctx=82736, majf=0, minf=61
  IO depths    : 1=100.0%, 2=0.0%, 4=0.0%, 8=0.0%, 16=0.0%, 32=0.0%, >=64=0.0%
     submit    : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     complete  : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     issued rwts: total=81920,0,0,0 short=0,0,0,0 dropped=0,0,0,0
     latency   : target=0, window=0, percentile=100.00%, depth=1

Run status group 0 (all jobs):
   READ: bw=135MiB/s (141MB/s), 135MiB/s-135MiB/s (141MB/s-141MB/s), io=10.0GiB (10.7GB), run=75889-75889msec



$ fio --randrepeat=1 --name=test --filename=fiotest --bs=2m --iodepth=1 --readwrite=randread --size=10G -direct=1
test: (g=0): rw=randread, bs=(R) 2048KiB-2048KiB, (W) 2048KiB-2048KiB, (T) 2048KiB-2048KiB, ioengine=psync, iodepth=1
fio-3.28
Starting 1 process
Jobs: 1 (f=1): [r(1)][100.0%][r=695MiB/s][r=347 IOPS][eta 00m:00s]
test: (groupid=0, jobs=1): err= 0: pid=21575: Tue Dec 28 17:00:10 2021
  read: IOPS=336, BW=672MiB/s (705MB/s)(10.0GiB/15227msec)
    clat (usec): min=2630, max=6284, avg=2968.73, stdev=282.67
     lat (usec): min=2630, max=6285, avg=2969.09, stdev=282.68
    clat percentiles (usec):
     |  1.00th=[ 2704],  5.00th=[ 2737], 10.00th=[ 2737], 20.00th=[ 2769],
     | 30.00th=[ 2802], 40.00th=[ 2802], 50.00th=[ 2868], 60.00th=[ 2933],
     | 70.00th=[ 2999], 80.00th=[ 3163], 90.00th=[ 3359], 95.00th=[ 3523],
     | 99.00th=[ 3949], 99.50th=[ 4113], 99.90th=[ 4555], 99.95th=[ 4883],
     | 99.99th=[ 6259]
   bw (  KiB/s): min=639236, max=716596, per=100.00%, avg=689137.23, stdev=20736.67, samples=30
   iops        : min=  312, max=  349, avg=335.97, stdev=10.04, samples=30
  lat (msec)   : 4=99.22%, 10=0.78%
  cpu          : usr=0.19%, sys=11.65%, ctx=5382, majf=0, minf=540
  IO depths    : 1=100.0%, 2=0.0%, 4=0.0%, 8=0.0%, 16=0.0%, 32=0.0%, >=64=0.0%
     submit    : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     complete  : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     issued rwts: total=5120,0,0,0 short=0,0,0,0 dropped=0,0,0,0
     latency   : target=0, window=0, percentile=100.00%, depth=1

Run status group 0 (all jobs):
   READ: bw=672MiB/s (705MB/s), 672MiB/s-672MiB/s (705MB/s-705MB/s), io=10.0GiB (10.7GB), run=15227-15227msec





```



# Sequential Read

Sequential Read with 4KB, 128KB and 2MB block sizes.

```

$ fio --randrepeat=1 --name=test --filename=fiotest --bs=4k --iodepth=1 --readwrite=read --size=10G
test: (g=0): rw=read, bs=(R) 4096B-4096B, (W) 4096B-4096B, (T) 4096B-4096B, ioengine=psync, iodepth=1
fio-3.28
Starting 1 process
Jobs: 1 (f=1): [R(1)][100.0%][r=528MiB/s][r=135k IOPS][eta 00m:00s]
test: (groupid=0, jobs=1): err= 0: pid=22103: Tue Dec 28 17:37:14 2021
  read: IOPS=136k, BW=533MiB/s (559MB/s)(10.0GiB/19218msec)
    clat (nsec): min=1000, max=84895k, avg=6930.47, stdev=623830.02
     lat (nsec): min=1000, max=84895k, avg=7003.33, stdev=623829.90
    clat percentiles (nsec):
     |  1.00th=[ 1004],  5.00th=[ 1004], 10.00th=[ 1004], 20.00th=[ 1004],
     | 30.00th=[ 1004], 40.00th=[ 1004], 50.00th=[ 1004], 60.00th=[ 1004],
     | 70.00th=[ 1004], 80.00th=[ 2008], 90.00th=[ 2008], 95.00th=[ 2008],
     | 99.00th=[ 2008], 99.50th=[ 2008], 99.90th=[ 2992], 99.95th=[ 4016],
     | 99.99th=[20096]
   bw (  KiB/s): min=480223, max=589824, per=99.73%, avg=544153.29, stdev=43836.80, samples=38
   iops        : min=120055, max=147456, avg=136038.05, stdev=10959.17, samples=38
  lat (usec)   : 2=71.64%, 4=28.30%, 10=0.04%, 20=0.01%, 50=0.01%
  lat (usec)   : 100=0.01%, 250=0.01%, 500=0.01%, 750=0.01%
  lat (msec)   : 2=0.01%, 4=0.01%, 10=0.01%, 20=0.01%, 50=0.01%
  lat (msec)   : 100=0.01%
  cpu          : usr=10.32%, sys=23.22%, ctx=5346, majf=0, minf=36
  IO depths    : 1=100.0%, 2=0.0%, 4=0.0%, 8=0.0%, 16=0.0%, 32=0.0%, >=64=0.0%
     submit    : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     complete  : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     issued rwts: total=2621440,0,0,0 short=0,0,0,0 dropped=0,0,0,0
     latency   : target=0, window=0, percentile=100.00%, depth=1

Run status group 0 (all jobs):
   READ: bw=533MiB/s (559MB/s), 533MiB/s-533MiB/s (559MB/s-559MB/s), io=10.0GiB (10.7GB), run=19218-19218msec



$ fio --randrepeat=1 --name=test --filename=fiotest --bs=128k --iodepth=1 --readwrite=read --size=10G
test: (g=0): rw=read, bs=(R) 128KiB-128KiB, (W) 128KiB-128KiB, (T) 128KiB-128KiB, ioengine=psync, iodepth=1
fio-3.28
Starting 1 process
Jobs: 1 (f=1): [R(1)][100.0%][r=577MiB/s][r=4612 IOPS][eta 00m:00s]
test: (groupid=0, jobs=1): err= 0: pid=22116: Tue Dec 28 17:38:38 2021
  read: IOPS=4981, BW=623MiB/s (653MB/s)(10.0GiB/16444msec)
    clat (usec): min=10, max=87052, avg=200.10, stdev=3617.56
     lat (usec): min=10, max=87053, avg=200.19, stdev=3617.56
    clat percentiles (usec):
     |  1.00th=[   13],  5.00th=[   13], 10.00th=[   15], 20.00th=[   15],
     | 30.00th=[   15], 40.00th=[   15], 50.00th=[   15], 60.00th=[   16],
     | 70.00th=[   16], 80.00th=[   16], 90.00th=[   17], 95.00th=[   18],
     | 99.00th=[   23], 99.50th=[   32], 99.90th=[71828], 99.95th=[72877],
     | 99.99th=[74974]
   bw (  KiB/s): min=499712, max=688128, per=99.92%, avg=637150.06, stdev=52011.77, samples=32
   iops        : min= 3904, max= 5376, avg=4977.28, stdev=406.32, samples=32
  lat (usec)   : 20=97.76%, 50=1.95%, 100=0.02%, 500=0.01%, 1000=0.01%
  lat (msec)   : 2=0.01%, 10=0.01%, 20=0.01%, 50=0.01%, 100=0.26%
  cpu          : usr=0.49%, sys=19.73%, ctx=5238, majf=0, minf=65
  IO depths    : 1=100.0%, 2=0.0%, 4=0.0%, 8=0.0%, 16=0.0%, 32=0.0%, >=64=0.0%
     submit    : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     complete  : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     issued rwts: total=81920,0,0,0 short=0,0,0,0 dropped=0,0,0,0
     latency   : target=0, window=0, percentile=100.00%, depth=1

Run status group 0 (all jobs):
   READ: bw=623MiB/s (653MB/s), 623MiB/s-623MiB/s (653MB/s-653MB/s), io=10.0GiB (10.7GB), run=16444-16444msec





$ fio --randrepeat=1 --name=test --filename=fiotest --bs=2m --iodepth=1 --readwrite=read --size=10G
test: (g=0): rw=read, bs=(R) 2048KiB-2048KiB, (W) 2048KiB-2048KiB, (T) 2048KiB-2048KiB, ioengine=psync, iodepth=1
fio-3.28
Starting 1 process
Jobs: 1 (f=1): [R(1)][100.0%][r=622MiB/s][r=311 IOPS][eta 00m:00s]
test: (groupid=0, jobs=1): err= 0: pid=22125: Tue Dec 28 17:39:37 2021
  read: IOPS=315, BW=631MiB/s (662MB/s)(10.0GiB/16225msec)
    clat (usec): min=180, max=82363, avg=3166.23, stdev=14070.94
     lat (usec): min=180, max=82363, avg=3166.34, stdev=14070.93
    clat percentiles (usec):
     |  1.00th=[  198],  5.00th=[  208], 10.00th=[  212], 20.00th=[  215],
     | 30.00th=[  215], 40.00th=[  217], 50.00th=[  219], 60.00th=[  221],
     | 70.00th=[  225], 80.00th=[  231], 90.00th=[  258], 95.00th=[  322],
     | 99.00th=[71828], 99.50th=[72877], 99.90th=[76022], 99.95th=[77071],
     | 99.99th=[82314]
   bw (  KiB/s): min=498714, max=688128, per=99.91%, avg=645721.09, stdev=51854.13, samples=32
   iops        : min=  243, max=  336, avg=314.91, stdev=25.39, samples=32
  lat (usec)   : 250=89.30%, 500=6.45%
  lat (msec)   : 10=0.02%, 20=0.06%, 50=0.04%, 100=4.14%
  cpu          : usr=0.06%, sys=19.83%, ctx=5140, majf=0, minf=544
  IO depths    : 1=100.0%, 2=0.0%, 4=0.0%, 8=0.0%, 16=0.0%, 32=0.0%, >=64=0.0%
     submit    : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     complete  : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     issued rwts: total=5120,0,0,0 short=0,0,0,0 dropped=0,0,0,0
     latency   : target=0, window=0, percentile=100.00%, depth=1

Run status group 0 (all jobs):
   READ: bw=631MiB/s (662MB/s), 631MiB/s-631MiB/s (662MB/s-662MB/s), io=10.0GiB (10.7GB), run=16225-16225msec




```