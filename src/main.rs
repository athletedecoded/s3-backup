use aws_lambda_events::event::s3::S3Event;
use aws_sdk_s3::{
    error::{CopyObjectError, DeleteObjectError},
    output::{CopyObjectOutput, DeleteObjectOutput},
    types::SdkError,
    Client,
};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::Serialize;
use urlencoding::encode;

#[derive(Serialize)]
struct Response {
    event_type: String,
    key: String,
    msg: String,
}

// Copy object from source bucket to destination bucket
async fn cp_object(
    client: &Client,
    src_ap: &str,
    dest_ap: &str,
    obj_key: &str,
) -> Result<CopyObjectOutput, SdkError<CopyObjectError>> {
    // formatted per https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/client/struct.Client.html#method.copy_object
    let copy_src = format!("{src_ap}/object/{obj_key}");
    println!("cp_src: {:?}", copy_src);
    let copy_src_encoded = encode(&copy_src);

    client
        .copy_object()
        .copy_source(copy_src_encoded)
        .bucket(dest_ap)
        .key(obj_key)
        .send()
        .await
}

// Delete object from bucket
async fn delete_object(
    client: &Client,
    bucket: &str,
    key: &str,
) -> Result<DeleteObjectOutput, SdkError<DeleteObjectError>> {
    client.delete_object().bucket(bucket).key(key).send().await
}

/*
AWS Lambda function which extracts info from S3 event and returns it as a JSON string.
 */
async fn function_handler(event: LambdaEvent<S3Event>) -> Result<Response, Error> {
    // Extract event info from S3 event
    let event_type = event.payload.records[0]
        .event_name
        .as_ref()
        .unwrap()
        .as_str();
    let key = event.payload.records[0]
        .s3
        .object
        .key
        .as_ref()
        .unwrap()
        .as_str();
    // Load env variables
    let src_ap = std::env::var("SRC_AP_ARN").expect("SRC_AP_ARN must be set");
    let dest_ap = std::env::var("DEST_AP_ARN").expect("DEST_AP_ARN must be set");
    let rubbish_ap = std::env::var("RUBBISH_AP_ARN").expect("RUBBISH_AP_ARN must be set");
    // Configure client
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    // match event type
    match event_type {
        // If event is ObjectCreated:Put or ObjectCreated:Post, copy object to destination bucket
        "ObjectCreated:Put" | "ObjectCreated:Post" => {
            println!("Copying object: {:?}", key);
            let resp_msg = cp_object(&client, &src_ap, &dest_ap, key).await?;
            return Ok(Response {
                event_type: format!("Event trigger: {event_type}"),
                key: format!("Object key: {key}"),
                msg: format!("SUCCESS: {:?}", resp_msg),
            });
        }
        // If event is ObjectRemoved:Delete, copy to rubbish then delete object from destination bucket
        "ObjectRemoved:Delete" => {
            println!("Moving object to rubbish: {:?}", key);
            let _ = cp_object(&client, &dest_ap, &rubbish_ap, key).await?;
            println!("Deleting object: {:?}", key);
            let resp_msg = delete_object(&client, &dest_ap, key).await?;
            return Ok(Response {
                event_type: format!("Event trigger: {event_type}"),
                key: format!("Object key: {key}"),
                msg: format!("SUCCESS: {:?}", resp_msg),
            });
        }
        _ => {
            println!("Event type not supported: {:?}", event_type);
            return Ok(Response {
                event_type: format!("Event type not supported: {event_type}"),
                key: format!("Event type not supported for key: {key}"),
                msg: format!("ERROR for {event_type}"),
            });
        }
    }
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
