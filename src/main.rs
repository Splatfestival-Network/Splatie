mod ayy;
mod cheeseburger;
mod emergency_report;
mod error_codes;
mod fest_fax;
mod miiverse_mod_application;
mod too_fat;

use once_cell::sync::Lazy;
use regex::Regex;
use serenity::async_trait;
use serenity::prelude::*;
use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv::dotenv().ok();

    let token = env::var("PROFESSOR_TOKEN").expect("Token not specified");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(error_codes::ErrorCodeHandler)
        .event_handler(ayy::AyyHandler)
        .event_handler(cheeseburger::CheeseburgerHandler)
        .event_handler(too_fat::TooFatHandler)
        .event_handler(emergency_report::EmergencyReportHandler::default())
        .event_handler(miiverse_mod_application::MiiverseModApplicationHandler::default())
        //.event_handler(fest_fax::FestFaxHandler::default())
        .await
        .expect("unable to create client");

    client.start().await.expect("error running bot");
}
