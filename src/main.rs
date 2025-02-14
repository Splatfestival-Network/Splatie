mod error_codes;

use std::env;
use once_cell::sync::Lazy;
use regex::Regex;
use serenity::async_trait;
use serenity::prelude::*;





#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv::dotenv().ok();

    let token = env::var("PROFESSOR_TOKEN").expect("Token not specified");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;


    let mut client = Client::builder(&token, intents)
        .event_handler(error_codes::ErrorCodeHandler)
        .await.expect("unable to create client");

    client.start().await.expect("error running bot");
}
