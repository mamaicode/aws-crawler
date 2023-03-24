use std::env;
use std::io::{self, Write};

use aws_config::load_from_env;
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::{Client as S3Client};

use spider::configuration::Configuration;
use spider::reqwest;
use spider::website::Website;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Please provide a URL to crawl");
    }

    let url = &args[1];

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

    println!("Enter the name of the S3 bucket to save the crawled data:");
    io::stdout().flush()?;
    let mut bucket_name = String::new();
    io::stdin().read_line(&mut bucket_name)?;

    let s3_client = S3Client::new(&load_from_env().await?);

    for link in website.get_links() {
        println!("- {:?}", link.as_ref());
        let mut response = reqwest::get(link.as_ref()).await?;
        let mut body = vec![];
        response.read_to_end(&mut body)?;

        let key = format!("{}{}", website.base_url(), link);
        let byte_stream = ByteStream::from(body);
        let put_request = s3_client.put_object()
            .bucket(&*bucket_name.trim())
            .key(&key)
            .body(byte_stream);
        put_request.send().await?;
    }

    Ok(())
}