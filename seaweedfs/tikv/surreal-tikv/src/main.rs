use surrealdb::Datastore;
use surrealdb::Error;
use surrealdb::Session;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let ds = Datastore::new("file://database.db").await?;
    // let ds = Datastore::new("tikv://127.0.0.1:20160").await?;
    // let ds = Datastore::new("127.0.0.1:20160").await?;
    let ds = Datastore::new("tikv://127.0.0.1:2379").await?;
    // let ses = Session::for_kv();
    // let ast = "USE NS test DB test; SELECT * FROM person;";
    // let res = ds.execute(ast, &ses, None, false).await?;

    // let ds = Datastore::new("0.0.0.0:8000").await?;
    let ses = Session::for_kv();
    let res = ds
        .execute(
            "CREATE author:john SET
	                name.first = 'John',
                    name.last = 'Adams',
                    name.full = string::join(' ', name.first, name.last),
                    age = 29,
                    admin = true,
                    signup_at = time::now();",
            &ses,
            None,
            false,
        )
        .await?;
    // dbg!(res);

    /*     let res = ds
            .execute(
                "CREATE article SET
        created_at = time::now(),
        author = author:john,
        title = 'Lorem ipsum dolor',
        text = 'Donec eleifend, nunc vitae commodo accumsan, mauris est fringilla.',
        account = (SELECT id FROM account WHERE name = 'ACME Inc' LIMIT 1)
    ;",
                &ses,
                None,
                false,
            )
            .await?; */
    // dbg!(res);
    let ast = "SELECT * FROM author;";
    let res = ds.execute(ast, &ses, None, false).await?;
    dbg!(res);
    play().await;
    Ok(())
}

async fn play() {
    let client = tikv_client::RawClient::new(vec!["127.0.0.1:2379"])
        .await
        .unwrap();

    let key = "Hello".to_owned();
    let value = "RawKV".to_owned();

    // put
    let result = client.put(key.to_owned(), value.to_owned()).await.unwrap();
    assert_eq!(result, ());

    // get
    let result = client.get(key.to_owned()).await.unwrap();
    assert_eq!(result.unwrap(), value.as_bytes());

    // delete
    let result = client.delete(key.to_owned()).await.unwrap();
    assert_eq!(result, ());

    // get
    let result = client.get(key.to_owned()).await.unwrap();
    assert_eq!(result, None);

    // scan
    let limit = 1000;
    client.put("k1".to_owned(), "v1".to_owned()).await.unwrap();
    client.put("k2".to_owned(), "v2".to_owned()).await.unwrap();
    client.put("k3".to_owned(), "v3".to_owned()).await.unwrap();
    client.put("k4".to_owned(), "v4".to_owned()).await.unwrap();
    let result = client
        .scan("k1".to_owned().."k5".to_owned(), limit)
        .await
        .unwrap();
    println!("{:?}", result);
}
