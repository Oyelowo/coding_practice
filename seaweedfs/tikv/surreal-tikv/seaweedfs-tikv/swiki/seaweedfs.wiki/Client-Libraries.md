## Clients

Name | Author | Language  
---|---|---
[WeedPHP](https://github.com/micjohnson/weed-php/) | Mic Johnson | PHP
[SeaweedFS Symfony bundle](https://github.com/micjohnson/weed-php-bundle) | Mic Johnson | PHP
[PHP client for SeaweedFS](https://github.com/tystuyfzand/seaweedfs-client) | tystuyfzand | PHP
[Module for kohana](https://github.com/bububa/kohanaphp-weedfs) | Bububa | PHP
[Laravel](https://github.com/jack-koli/laravel-seaweedfs) | jack-koli | PHP
[SeaweedFS Node.js client](https://github.com/cruzrr/node-weedfs) | Aaron Blakely | Javascript
[SeaweedFS Node.js client](https://github.com/atroo/node-weedfs) | atroo | Javascript
[SeaweedFS Node](https://github.com/playlyfe/seaweedfs-node) | Johny Jose | Javascript
[jude-seaweedfs](https://www.npmjs.com/package/jude-seaweedfs) | Roberto Sales |  Javascript
[@trubavuong/seaweedfs](https://www.npmjs.com/package/@trubavuong/seaweedfs) | Vuong Tru |  Javascript
[@trubavuong/fastify-seaweedfs](https://www.npmjs.com/package/@trubavuong/fastify-seaweedfs) | Vuong Tru |  Javascript
[seaweedfs client plug-in for egg.js](https://github.com/aliwalker/egg-seaweed-client) | aliwalker | Javascript
[Weedo](https://github.com/ginuerzh/weedo) | Ginuerzh |  Go
[goseaweed](https://github.com/tnextday/goseaweed) | tnextday |  Go
[goseaweedfs](https://github.com/linxGnu/goseaweedfs) | linxGnu |  Go
[Weedharvester](https://github.com/ChristianNorbertBraun/Weedharvester) | ChristianNorbertBraun | Go
[Java SeaweedFS client](https://github.com/zenria/Weed-FS-Java-Client) | Zenria | Java
[SeaweedFS Client](https://github.com/lokra/seaweedfs-client) | Lokra Studio | Java
[SeaweedFS Client For Java](https://github.com/Shuyun123/seaweedfs-java-client) | Shuyun123 | Java
[seaweedfs4j](https://github.com/zhiyoucai/seaweedfs4j) | zhiyoucai | Java
[Python-weed](https://github.com/darkdarkfruit/python-weed) | Darkdarkfruit | Python
[Django-weed](https://github.com/ProstoKSI/django-weed) | ProstoKSI | Python
[Pyweed](https://github.com/utek/pyweed) | Utek | Python
[Scala SeaweedFS client](https://github.com/chiradip/WeedFsScalaClient) | Chiradip | Scala
[Seaweedrb](https://github.com/jguest/seaweedrb) | John Guest | Ruby
[SeaweedFs.NET](https://github.com/piechpatrick/SeaweedFs.Client) | piechpatrick |  C#
[seaweedfs client](https://github.com/TheCageMan/seaweedfs-client) | TheCageMan |  C#
[C# client library for the SeaweedFS](https://github.com/TerabyteX/WeedCSharpClient) | TerabyteX |  C#
[Erlang SeaweedFS Client](https://github.com/Neurotec/seaweedfs.erl) | Neurotec | Erlang
[Julia SeaweedFS Client](https://github.com/lawless-m/SeaweedFSClient.jl) | Lawless-m | Julia
## GRPC APIs

SeaweedFS uses GRPC internally. You can use them too. Just check https://github.com/seaweedfs/seaweedfs/tree/master/weed/pb for the proto files.

When developing HDFS compatible file system, which allows replacing HDFS with SeaweedFS, a Java implementation of the GRPC client API is developed.

* Java GRPC client Source code: https://github.com/seaweedfs/seaweedfs/tree/master/other/java/client
* Java GRPC client Maven Repo: https://mvnrepository.com/artifact/com.github.chrislusf/seaweedfs-client

## Projects using SeaweedFS

- An [Email River Plugin](https://github.com/medcl/elasticsearch-river-email/) for Elasticsearch uses SeaweedFS server to save attachments
- A [Cloud CCTV Storage Platform](https://github.com/evercam/evercam-media) for [Evercam.io](https://evercam.io/) uses SeaweedFS server to save image files
- A [lua-resty-weedfs](https://github.com/medcl/lua-resty-weedfs), Nginx backend with weedfs and file post processing with ffmpeg and graphicsmagick by Medcl
