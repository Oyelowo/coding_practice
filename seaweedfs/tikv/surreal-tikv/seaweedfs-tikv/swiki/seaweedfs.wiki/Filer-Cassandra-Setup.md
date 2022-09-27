SeaweedFS can utilize existing familiar data store, e.g., Cassandra, Mysql, Postgres, Redis, Sqlite, to store the filer metadata.

The following takes Cassandra as an example.

## Cassandra Setup

Here is the CQL to create the table.CassandraStore.
Optionally you can adjust the keyspace name and replication settings.
For production, you would want to set replication_factor to 3
if there are at least 3 Cassandra servers.

```cql
create keyspace seaweedfs WITH replication = {
  'class':'SimpleStrategy',
  'replication_factor':1
};

use seaweedfs;

CREATE TABLE filemeta (
  directory varchar,
  name varchar,
  meta blob,
  PRIMARY KEY (directory, name)
) WITH CLUSTERING ORDER BY (name ASC);

```

## Create a filer.toml

Try run ```weed filer -h``` to see an example filer.toml file. The file should be under one of current directory, $HOME/.seaweedfs/, or /etc/seaweedfs/ folders.

Here is the shortest example for Cassandra

```bash
[cassandra]
enabled = true
keyspace="seaweedfs"
hosts=[
    "localhost:9042",
]
```


## Starting the Filer

```bash
weed filer
```
