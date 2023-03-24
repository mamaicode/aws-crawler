use spider::configuration::Configuration;
use spider::website::Website;

use tokio::{fs::File, io::AsyncReadExt};

use std::{env, io::Read};

use aws_sdk_s3::{ByteStream, Client, Region};

#[tokio::main]
async fn main() {
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

    // to fix
    let s3_client = Client::new(Region::default());

    // to fix
    for link in website.get_links() {
        println!("- {:?}", link.as_ref());
        let mut response = reqwest::get(link.as_ref()).await?;
        let mut body = vec![];
        response.read_to_end(&mut body)?;

        let key = format!("{}{}", website.base_url(), link);
        let byte_stream = ByteStream::from(body);
        let put_request = s3_client.put_object()
            .bucket(bucket_name)
            .key(key)
            .body(byte_stream);
        put_request.send().await?;
    }

    Ok(())
}