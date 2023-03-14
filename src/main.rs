use std::env;
use std::error::Error;

use spider::website::Website;
use tokio::runtime::Runtime;

use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3Client, S3};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get the URL to crawl from the command-line arguments
    let args: Vec<String> = env::args().collect();
    let url = args.get(1).ok_or("URL not provided")?;

    // Create a new website instance and crawl it
    let mut website = Website::new(url);
    website.crawl().await;

    // Get the page contents as a byte array
    let contents = website.get_html().as_bytes().to_vec();

    // Initialize the S3 client
    let s3_client = S3Client::new(Region::default());

    // Prepare the request to store the contents in S3
    let bucket_name = "my-bucket-name";
    let object_key = format!("{}{}", url, ".html");
    let request = PutObjectRequest {
        bucket: bucket_name.to_string(),
        key: object_key.to_string(),
        body: Some(contents.into()),
        ..Default::default()
    };

    // Send the request to S3
    s3_client.put_object(request).await?;

    Ok(())
}