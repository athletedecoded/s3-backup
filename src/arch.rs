use aws_lambda_events::event::s3::S3Event;use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use aws_sdk_s3::{Client, Error};
use aws_config::meta::region::RegionProviderChain;
use std::process;
use std::io::Cursor;
use image::ImageFormat;
use image::io::Reader;
use tokio::io::{AsyncBufReadExt, BufReader};


#[derive(Serialize)]
struct Response {
    key: String,
    url: String,
}

// Create AWS client
async fn create_client() -> Result<Client, Error> {
    let region_provider = RegionProviderChain::first_try(None)
        .or_default_provider()
        .or_else("us-west-2");
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);
    Ok(client)
}

// Put object in bucket
pub async fn upload_object(client: &Client, bucket: &str, filepath: &str) -> Result<(), Error> {
    let body = ByteStream::from_path(Path::new(filepath)).await;
    let key = Path::new(filepath).file_name().unwrap().to_str().unwrap();
    match body {
        Ok(b) => {
            let _resp = client
                .put_object()
                .bucket(bucket)
                .key(key)
                .body(b)
                .send()
                .await?;
            println!("Uploaded {key} to {bucket}");
        }
        Err(e) => {
            println!("Got an error uploading object:");
            println!("{e}");
            process::exit(1);
        }
    }

    Ok(())
}

// Function to get object from S3 using s3-accesspoint-alias
pub async fn get_object(client: &Client, key: &str,) -> Result<(), Error> {
    let ap_alias = "arn:aws:s3:us-east-2:141774272727:accesspoint/img-aug-lambda";
    let result = client
        .get_object()
        .bucket(ap_alias)
        .key(key)
        .send()
        .await?;
    // Convert body to image
    let body = result.body.collect().await.map(|data| data.into_bytes());
    Ok((body))
}

/*
AWS Lambda function which extracts info from S3 event and returns it as a JSON string.
 */
async fn function_handler(event: LambdaEvent<S3Event>) -> Result<Response, Error> {
    // prepare the response
    let bucket = &event.payload.records[0].s3.bucket.name;
    let key = &event.payload.records[0].s3.object.key;
    let url = &event.payload.records[0].s3.object.url_decoded_key;
    // Use aws-sdk-s3 to get the object from S3
    let client = create_client().await.unwrap();


    let resp = Response {
        key: format!("New object key: {:?}", key),
        url: format!("New object url: {:?}", url)
    };
    // return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
