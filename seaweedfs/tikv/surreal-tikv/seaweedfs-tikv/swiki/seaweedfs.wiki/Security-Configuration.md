

The first step is generating `security.toml` file via `weed scaffold -config=security`:

```
$ weed scaffold -config=security

# Put this file to one of the location, with descending priority
#    ./security.toml
#    $HOME/.seaweedfs/security.toml
#    /etc/seaweedfs/security.toml
# this file is read by master, volume server, and filer

# this jwt signing key is read by master and volume server, and it is used for write operations:
# - the Master server generates the JWT, which can be used to write a certain file on a volume server
# - the Volume server validates the JWT on writing
# the jwt defaults to expire after 10 seconds.
[jwt.signing]
key = ""
expires_after_seconds = 10           # seconds

# by default, if the signing key above is set, the Volume UI over HTTP is disabled.
# by setting ui.access to true, you can re-enable the Volume UI. Despite
# some information leakage (as the UI is not authenticated), this should not
# pose a security risk.
[access]
ui = false

# this jwt signing key is read by master and volume server, and it is used for read operations:
# - the Master server generates the JWT, which can be used to read a certain file on a volume server
# - the Volume server validates the JWT on reading
# NOTE: jwt for read is only supported with master+volume setup. Filer does not support this mode.
[jwt.signing.read]
key = ""
expires_after_seconds = 10           # seconds


# If this JWT key is configured, Filer only accepts writes over HTTP if they are signed with this JWT:
# - f.e. the S3 API Shim generates the JWT
# - the Filer server validates the JWT on writing
# the jwt defaults to expire after 10 seconds.
[jwt.filer_signing]
key = ""
expires_after_seconds = 10           # seconds

# If this JWT key is configured, Filer only accepts reads over HTTP if they are signed with this JWT:
# - f.e. the S3 API Shim generates the JWT
# - the Filer server validates the JWT on writing
# the jwt defaults to expire after 10 seconds.
[jwt.filer_signing.read]
key = ""
expires_after_seconds = 10           # seconds

# all grpc tls authentications are mutual
# the values for the following ca, cert, and key are paths to the PERM files.
# the host name is not checked, so the PERM files can be shared.
[grpc]
ca = ""
# Set wildcard domain for enable TLS authentication by common names
allowed_wildcard_domain = "" # .mycompany.com

[grpc.volume]
cert = ""
key = ""
allowed_commonNames = ""    # comma-separated SSL certificate common names

[grpc.master]
cert = ""
key = ""
allowed_commonNames = ""    # comma-separated SSL certificate common names

[grpc.filer]
cert = ""
key = ""
allowed_commonNames = ""    # comma-separated SSL certificate common names

[grpc.msg_broker]
cert = ""
key = ""
allowed_commonNames = ""    # comma-separated SSL certificate common names

# use this for any place needs a grpc client
# i.e., "weed backup|benchmark|filer.copy|filer.replicate|mount|s3|upload"
[grpc.client]
cert = ""
key = ""

# volume server https options
# Note: work in progress!
#     this does not work with other clients, e.g., "weed filer|mount" etc, yet.
[https.client]
enabled = true
[https.volume]
cert = ""
key = ""

```

The following command is what I used to generate the private key and certificate files, using https://github.com/square/certstrap. To compile this tool, you can run `go get github.com/square/certstrap` - or alternatively `brew install certstrap` if you are on Mac OS and use [Homebrew](https://brew.sh).

```
certstrap init --common-name "SeaweedFS CA"
certstrap request-cert --common-name volume01
certstrap request-cert --common-name master01
certstrap request-cert --common-name filer01
certstrap request-cert --common-name client01
certstrap sign --CA "SeaweedFS CA" volume01
certstrap sign --CA "SeaweedFS CA" master01
certstrap sign --CA "SeaweedFS CA" filer01
certstrap sign --CA "SeaweedFS CA" client01
```

Here is my `security.toml` file content:
```

# Put this file to one of the location, with descending priority
#    ./security.toml
#    $HOME/.seaweedfs/security.toml
#    /etc/seaweedfs/security.toml

[jwt.signing]
key = "blahblahblahblah"

[jwt.filer_signing]
key = "blahblahblahblah"

# all grpc tls authentications are mutual 
[grpc]
ca = "/Users/chris/.seaweedfs/out/SeaweedFS_CA.crt"

[grpc.volume]
cert = "/Users/chris/.seaweedfs/out/volume01.crt"
key  = "/Users/chris/.seaweedfs/out/volume01.key"

[grpc.master]
cert = "/Users/chris/.seaweedfs/out/master01.crt"
key  = "/Users/chris/.seaweedfs/out/master01.key"

[grpc.filer]
cert = "/Users/chris/.seaweedfs/out/filer01.crt"
key  = "/Users/chris/.seaweedfs/out/filer01.key"

[grpc.client]
cert = "/Users/chris/.seaweedfs/out/client01.crt"
key  = "/Users/chris/.seaweedfs/out/client01.key"


```

### For Java gRPC
Java gRPC uses Netty's SslContext. From https://netty.io/wiki/sslcontextbuilder-and-private-key.html

> The SslContextBuilder and so Netty's SslContext implementations only support PKCS8 keys.
>
> If you have a key with another format you need to convert it to PKCS8 first to be able to use it. This can be done easily by using openssl.
>
> For example to convert a non-encrypted PKCS1 key to PKCS8 you would use:
>
> openssl pkcs8 -topk8 -nocrypt -in pkcs1_key_file -out pkcs8_key.pem
