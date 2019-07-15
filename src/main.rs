extern crate slack;

mod handlers;

use slack::RtmClient;
use std::{env, process};

fn main() {
    let api_key: String = api_key();
    let mut handler = handlers::Handler;
    let client = RtmClient::login_and_run(&api_key, &mut handler);

    match client {
        Ok(_) => {}
        Err(err) => panic!("Err: {}", err),
    }
}

fn api_key() -> String {
    match env::var("SLACK_API_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            println!("SLACK_API_TOKEN environment variable not set");
            process::exit(1);
        }
    }
}
