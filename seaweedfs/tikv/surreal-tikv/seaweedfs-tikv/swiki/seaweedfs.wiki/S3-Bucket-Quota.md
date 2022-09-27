# Efficient Usage Collection

To manage quota, the system first needs to know how much storage currently used. Usually this would need some relatively complicated counting, rolling up the directory tree. One way is to count during write time, but the upper tree nodes would become hot spots. This would slow down the normal write operations.

Since SeaweedFS uses one collection for each bucket, the usage can be efficiently collected from the volumes used by the collection. So the usage collection is very cheap and does not interfere with write operations.

# How it works

There are two aspects of quota management: "quota configuration" and "quota enforcement".

* "Quota Configuration" is done in by an admin script in `weed shell`, where you can set/remove/enable/disable bucket quota for each bucket.
* "Quota Enforcement" is actually also an admin script, which you can run it periodically by adding it to `master.toml`. It will check whether all the buckets are over their limit or not. If one bucket is over quota, it will set the bucket to read only.

The bucket's "readOnly" flag is checked when the filer processes write requests. If a bucket is readOnly, the creation and update requests on the bucket will be denied, except deletion. The read operations are not be affected.

# Quota Configuration and Enforcement

**1. View current bucket usage**

Here there are 3 example buckets created. The bucket sizes and file counts are listed. The bucket size is the actual disk space used, including replicated copies.

```
> s3.bucket.list
  b1	size:527971104	file:151
  b2	size:468432	file:6
  b3	size:10280	file:3
```

**2. Set bucket "b1" to a low quota**

```
> s3.bucket.quota -name b1 -sizeMB 15
updated quota for bucket b1
> s3.bucket.list
  b1	size:527971104	file:151	quota:15728640	usage:3356.75%
  b2	size:468432	file:6
  b3	size:10280	file:3
```

**3. Enforce the quota, and found it should be set to read only.**

```
> s3.bucket.quota.enforce -apply
  b1	size:527971104	quota:15728640	usage:3356.75%
    changing bucket b1 to read only!
```

Note: Internally this readOnly flag is saved into filer configuration for `locationPrefix="/buckets/b1/"`. So you could directly check the bucket `readOnly` property with `fs.configure` also.
```
> fs.configure
{
  "locations": [
    {
      "locationPrefix": "/buckets/b1/",
      "collection": "b1",
      "readOnly": true
    }
  ]
}
```

# Schedule Quota Enforcement

`s3.bucket.quota.enforce -apply` should be run regularly. You can add it to the `master.toml` file, or add it to some shell scripts. The frequency depends on how much you trust your users. :)
