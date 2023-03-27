<p align="center"><a href="https://www.rust-lang.org" target="_blank"><img src="https://img.shields.io/badge/Made%20With-Rust-000000?style=for-the-badge" alt="made with rust" /></a></a>
</p>

# **aws-crawler**
CLI program in Rust that crawls a website using the Spider crate and uploads the crawled data to an S3 bucket using the AWS SDK S3 crate.

# **How-to**
To use aws-crawler:
```
cargo run website_to_crawl bucket_name region
```
Please double-check that you have correctly set up your AWS credentials, and that you have the necessary permissions to create and write to an S3 bucket in the specified region. 