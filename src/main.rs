#![allow(clippy::result_large_err)]

use clap::Parser;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_secretsmanager::{config::Region, Client, Error};

#[derive(Debug, Parser)]
struct Args {
    #[structopt(short, long)]
    region: Option<String>,

    #[structopt(short, long)]
    pwd_length: i64,
}

async fn gen_pwd(client: &Client, pwd_length: i64) -> Result<(), Error> {
    let resp = client
        .get_random_password()
        .password_length(pwd_length)
        .require_each_included_type(true)
        .send()
        .await?;

    println!("Value: {}", resp.random_password().unwrap_or("Nope!"));
    Ok(())
}

#[tokio::main]
async fn main() -> () {
    tracing_subscriber::fmt::init();
    let Args {
        region,
        pwd_length,
    } = Args::parse();

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-east-1"));

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    let _ = gen_pwd(&client, pwd_length).await;
}