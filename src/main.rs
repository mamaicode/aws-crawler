use spider::{Config, Spider};
use std::error::Error;
use rusoto_s3::{S3Client, PutObjectRequest};
use rusoto_core::Region;
use clap::{Arg, App};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("my-crawler")
        .version("1.0")
        .author("Your Name <you@example.com>")
        .about("A CLI program for crawling websites and storing data in S3")
        .arg(Arg::with_name("url")
             .short("u")
             .long("url")
             .value_name("URL")
             .help("The URL to crawl")
             .required(true)
             .takes_value(true))
        .get_matches();

    let url = matches.value_of("url").unwrap();

    let s3_client = S3Client::new(Region::default());
    let bucket_name = "my-bucket";
    let config = Config::new().concurrency(4);
    let mut spider = Spider::new(config);
    spider.get(url).await?.extract_links();
    let links = spider.into_links();
    for link in links {
        let response = reqwest::get(&link)?;
        let body = response.text()?;
        // parse the HTML body and extract the data you need
        // create an S3 object and store the data in the S3 bucket
        let key = format!("crawl/{}", link);
        let request = PutObjectRequest {
            bucket: bucket_name.to_owned(),
            key: key.clone(),
            body: Some(body.into()),
            ..Default::default()
        };
        s3_client.put_object(request).await?;
    }
    Ok(())
}