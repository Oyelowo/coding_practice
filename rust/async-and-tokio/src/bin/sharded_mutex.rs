use std::{collections::HashMap, sync::Arc};

type ShardedDb = Arc<Vec<::parking_lot::Mutex<HashMap<String, ::bytes::Bytes>>>>;

fn sharded_db(num_of_shards: usize) -> ShardedDb {
    let mut shards = Vec::with_capacity(num_of_shards);
    // let mut db = Arc::new();
    for _ in 0..num_of_shards {
        shards.push(::parking_lot::Mutex::new(HashMap::new()));
    }
    Arc::new(shards)
}


fn main() {

    let db = sharded_db(5);
    // Then, finding the cell for any given key becomes a two step process. First, the key is used to identify which shard it is part of. Then, the key is looked up in the HashMap.
    // db.clone().get(hash("") & )
    // let shard = db[hash(key) % db.len()].lock();
    // shard.insert(key, value)
}


