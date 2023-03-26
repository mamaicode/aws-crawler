use std::env;

use aws_config::load_from_env;
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::model::{BucketLocationConstraint, CreateBucketConfiguration};

use spider::configuration::Configuration;
use spider::reqwest;
use spider::website::Website;

async fn create_s3_bucket(s3_client: &S3Client, bucket_name: &str, region: &str) -> Result<(), Box<dyn std::error::Error>> {
    let constraint = BucketLocationConstraint::from(region);
    let cfg = CreateBucketConfiguration::builder()
        .location_constraint(constraint)
        .build();
    s3_client
        .create_bucket()
        .create_bucket_configuration(cfg)
        .bucket(bucket_name)
        .send()
        .await?;
    println!("Created S3 bucket '{}'", bucket_name);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        panic!("Provide a URL to crawl, bucket name, and region");
    }

    let url = &args[1];
    let bucket_name = &args[2];
    let region = &args[3];

    let mut config = Configuration::new();
    if let Some(blacklist_url) = config.blacklist_url.as_mut() {
        blacklist_url.push(format!("{}/license/", url).into());
    }
    config.respect_robots_txt = true;
    config.subdomains = true;
    config.tld = false;
    config.delay = 0;
    config.request_timeout = None;
    config.channel_buffer = 100;
    config.user_agent = Some(Box::new("myapp/version".to_string().into()));

    let mut website: Website = Website::new(url);
    website.configuration = Box::new(config);

    println!("Creating S3 bucket '{}'", bucket_name);
    let sdk_config = load_from_env().await;
    let s3_client = S3Client::new(&sdk_config);
    create_s3_bucket(&s3_client, bucket_name, region).await?;

    for link in website.get_links() {
        println!("- {:?}", link.as_ref());
        let response = reqwest::get(link.as_ref()).await?;
        let body = response.bytes().await?.to_vec();

        let key = format!("{}{}", url, link.as_ref());
        println!("Uploading crawled data: {}", key);
        let byte_stream = ByteStream::from(body);
        let put_request = s3_client.put_object()
            .bucket(&*bucket_name.trim())
            .key(&key)
            .body(byte_stream);
        put_request.send().await?;
    }

    Ok(())
}