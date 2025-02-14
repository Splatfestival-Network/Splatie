mod error_codes;
mod cheeseburger;
mod ayy;
mod miiverse_mod_application;

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
        .event_handler(ayy::AyyHandler)
        .event_handler(cheeseburger::CheeseburgerHandler)
        //.event_handler(miiverse_mod_application::MiiverseModApplicationHandler::default())
        .await.expect("unable to create client");

    client.start().await.expect("error running bot");
}
