use aws_lambda_events::event::s3::S3Event;use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Serialize};
use aws_sdk_s3::{
    error::CopyObjectError, output::CopyObjectOutput, types::SdkError, Client};
// use urlencoding::encode;

#[derive(Serialize)]
struct Response {
    bucket: String,
    key: String,
    // msg: String
}

async fn cp_object(
    client: &Client,
    source_bucket: &str,
    destination_bucket: &str,
    img_key: &str,
) -> Result<CopyObjectOutput, SdkError<CopyObjectError>> {
    //   accesspoint-alias/object/source-object-name
    let cp_src = format!("{source_bucket}/object/{img_key}");
    // let cp_src = format!("img-aug-lambda-hdq51x6px8ccuse8h5p1y63x86nqsuse2a-s3alias/object/{img_key}");
    // let cps_src_url = encode(&cp_src);

    client
        .copy_object()
        .copy_source(cp_src)
        .bucket(destination_bucket)
        .key(img_key)
        .send()
        .await
}

/*
AWS Lambda function which extracts info from S3 event and returns it as a JSON string.
 */
async fn function_handler(event: LambdaEvent<S3Event>) -> Result<Response, Error> {
    // get bucket name as string
    let bucket = event.payload.records[0].s3.bucket.name.as_ref().unwrap();
    let key = event.payload.records[0].s3.object.key.as_ref().unwrap();
    println!("Bucket: {bucket}, Key: {key}");
    // Use aws-sdk-s3 to get the object from S3
    // let config = aws_config::load_from_env().await;
    // let client = Client::new(&config);
    // Copy object to destination bucket
    // let cp_resp = cp_object(&client, bucket, "img-aug-output", key).await.unwrap();

    let resp = Response {
        bucket: format!("New object bucket: {bucket}"),
        key: format!("New object key: {key}"),
        // msg: format!("New object: {:?}", cp_resp)
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