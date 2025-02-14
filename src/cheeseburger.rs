use once_cell::sync::Lazy;
use regex::Regex;
use serenity::all::{Context, CreateCommand, EventHandler, Message};
use serenity::async_trait;
use crate::error_codes::ErrorCodeHandler;

static CHEESEBURGER_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new("[Cc]heeseburger(s)?").expect("invalid regex"));
pub struct CheeseburgerHandler;

#[async_trait]
impl EventHandler for CheeseburgerHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot{
            return;
        }

        if CHEESEBURGER_REGEX.is_match(&msg.content){
            msg.reply(&ctx.http, "currently scratching the FUCK out of my bum bum idgaf").await.ok();
        }
    }
}