### Installation
See https://restic.github.io

On mac: `brew install restic`

### Configuration
Set these environment variables. The key values do not matter.
```
export AWS_ACCESS_KEY_ID=any-key-id
export AWS_SECRET_ACCESS_KEY=any-access-key
```

### Execute commands
```
# create the bucket
$ s3cmd mb s3://resticbucket

$ restic -r s3:http://localhost:8333/resticbucket init

$ restic -r s3:http://localhost:8333/resticbucket backup /Users/chris/dev/gopath/bin/
open repository
enter password for repository:
repository 47edd0a2 opened successfully, password is correct
created new cache in /Users/chris/Library/Caches/restic

Files:          71 new,     0 changed,     0 unmodified
Dirs:            4 new,     0 changed,     0 unmodified
Added to the repo: 805.611 MiB

processed 71 files, 807.721 MiB in 0:05
snapshot 7436ec41 saved

```