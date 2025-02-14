use std::time::Duration;
use serenity::all::{Context, EventHandler, Message};
use serenity::async_trait;
use tokio::time::sleep;

pub struct TooFatHandler;

#[async_trait]
impl EventHandler for TooFatHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.contains("151-0204") {
            tokio::spawn(async move {
                sleep(Duration::from_secs(1)).await;

                msg.reply(&ctx.http, "in other words your too fat").await.ok();
            });
        }
    }
}