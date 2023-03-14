use spider::configuration::Configuration;
use spider::website::Website;
use spider::tokio;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please provide a URL to crawl");
    }
    let url = &args[1];

    let mut config = Configuration::new();
    config.blacklist_url.expect("blacklist_url is None").push(format!("{}/licenses/", url));
    config.respect_robots_txt = true;
    config.subdomains = true;
    config.tld = false;
    config.delay = 0;
    config.request_timeout = None;
    config.channel_buffer = 100;
    config.user_agent = "myapp/version".to_string();

    let mut website: Website = Website::with_configuration(url, config);
    website.crawl().await;

    for link in website.get_links() {
        println!("- {:?}", link.as_ref());
    }
}