use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3 as s3;
use http::Uri;
use s3::{Client, Credentials, Endpoint, Region};
use std::env;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), s3::Error> {
    // let config = aws_config::load_from_env().await;
    // let client = s3::Client::new(&config);
    /*

     export AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE
    export AWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
    export AWS_DEFAULT_REGION=us-west-2*/
    let key = "KEY";
    env::set_var("AWS_ACCESS_KEY_ID", "VALUE");
    env::set_var("AWS_SECRET_ACCESS_KEY", "VALUE");
    env::set_var("AWS_DEFAULT_REGION", "VALUE");

    // Select a profile by setting the `AWS_PROFILE` environment variable.
    let config = aws_config::load_from_env().await;
    let s3_local_config = aws_sdk_s3::config::Builder::from(&config)
        .region(Region::new("us-east-1"))
        .endpoint_resolver(
            // 8000 is the default dynamodb port
            Endpoint::immutable(Uri::from_static("http://localhost:8333")),
        )
        .build();
    let client = s3::Client::from_conf(s3_local_config);

    let resp = client.list_buckets().send().await?;
    let buckets = resp.buckets().unwrap_or_default();
    let num_buckets = buckets.len();

    let obj = client.get_object();

    dbg!(buckets);
    dbg!(num_buckets);
    dbg!(obj);
    dbg!(show_objects(&client, "lowo").await);
    // ... make some calls with the client

    Ok(())
}

// Lists the objects in a bucket.
// snippet-start:[s3.rust.list-objects]
async fn show_objects(client: &Client, bucket: &str) {
    let resp = client.list_objects_v2().bucket(bucket).send().await.unwrap();
    // let resp = client.list_objects_v2().bucket(bucket).send().await?;

    for object in resp.contents().unwrap_or_default() {
        println!("{}", object.key().unwrap_or_default());
    }
}
// sni
// async fn initialize_variables() -> (Region, Client, String, String, String, String) {
//     let region_provider = RegionProviderChain::first_try(Region::new("us-west-2"));
//     let region = region_provider.region().await.unwrap();

//     let shared_config = aws_config::from_env().credentials_provider(credentials_provider).region(region_provider).load().await;
//     let client = Client::new(&shared_config);
// // let p = uuid::Builder::;
// const ID: Uuid = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");

// let id = Uuid::new_v4();

//     let bucket_name = format!("doc-example-bucket-{}", Uuid::new_v4());

//     let file_name = "s3/testfile.txt".to_string();
//     let key = "test file key name".to_string();
//     let target_key = "target_key".to_string();

//     (region, client, bucket_name, file_name, key, target_key)
// }
