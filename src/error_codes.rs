use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;
use once_cell::sync::Lazy;
use regex::Regex;
use serenity::all::{Context, CreateActionRow, CreateButton, CreateEmbed, CreateEmbedFooter, CreateMessage, EditInteractionResponse, EditMessage, EventHandler, Http, Interaction, Message};
use serenity::all::CreateActionRow::Buttons;
use serenity::async_trait;
use tokio::time::sleep;

mod errors{
    include!(concat!(env!("OUT_DIR"), "/errors.rs"));
}

struct CategoryInfo{
    name: &'static str,
    description: &'static str,
    system: &'static str,
}

impl Default for CategoryInfo{
    fn default() -> Self {
        Self{
            name: "unknown",
            description: "unknown",
            system: "unknown"
        }
    }
}

struct ErrorInfo{
    name: &'static str,
    message: &'static str,
    short_description: &'static str,
    long_description: &'static str,
    short_solution: &'static str,
    long_solution: &'static str,
    support_link: &'static str,
}

impl Default for ErrorInfo{
    fn default() -> Self {
        Self{
            name: "unknown",
            message: "unknown",
            long_description: "unknown",
            long_solution: "unknown",
            short_description: "unknown",
            short_solution: "unknown",
            support_link: "unknown",
        }
    }
}

static ERROR_CODE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new("\\d\\d\\d-\\d\\d\\d\\d").expect("invalid regex"));




pub struct ErrorCodeHandler;

fn create_error_explain_message(str_code: &str, expanded: bool) -> Option<EditMessage>{
    let Ok(category) = str_code[0..3].parse() else {
        return None;
    };

    let Ok(error) = str_code[4..].parse() else {
        return None;
    };

    let (category_info, error_info) = errors::get_error_code_and_category(category, error);

    let mut embed = CreateEmbed::new()
        .title(str_code);

    if expanded{
        embed = embed.field("Module", category_info.name, true)
            .field("System", category_info.system, true)
            .field("Module Description", category_info.description, true)
            .field("Name", error_info.name, false)
            .field("Explanation", format!("```{}```", error_info.message), true)
            .field("Description", error_info.long_description, true)
            .field("Solution", error_info.long_solution, true)
        ;
    } else {
        embed = embed.field("Name", error_info.name, true)
            .field("Description", error_info.short_description, true)
            .field("Solution", error_info.short_solution, true)
    }

    let expand_button =
        CreateButton::new(format!("ERROR_EXPLAIN:{}", str_code))
            .label("Expand")
            .disabled(expanded);

    let message = EditMessage::new()
        .embed(embed)
        .components(vec![
            Buttons(
                vec![
                    expand_button
                ]
            )
        ])
        .content("");

    Some(message)
}

fn start_timed_explanation_collapse(mut message: Message, http: Arc<Http>, error_code: &str){
    let error_code: Box<str> = error_code.into();

    tokio::spawn(async move {
        sleep(Duration::from_secs(30)).await;

        let Some(new_message) = create_error_explain_message(&error_code, false) else {
            return
        };

        message.edit(&http, new_message).await.ok();
    });
}

#[async_trait]
impl EventHandler for ErrorCodeHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot{
            return;
        }

        if let Some(err_code) = ERROR_CODE_REGEX.find(&msg.content){
            let str_code = err_code.as_str();

            let Some(message) = create_error_explain_message(str_code, true) else {
                return
            };

            let response = CreateMessage::new()
                .content("loading")
                .reference_message(&msg);

            let Ok(mut msg) = msg.channel_id.send_message(&ctx.http, response).await else {
                return
            };

            let Some(edited) =  create_error_explain_message(str_code, true) else {
                return;
            };

            msg.edit(&ctx.http, edited).await.ok();

            start_timed_explanation_collapse(msg, ctx.http.clone(), str_code);
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Component(mut component) = interaction{
            if component.data.custom_id.starts_with("ERROR_EXPLAIN"){
                let Some((_, error_code)) = component.data.custom_id.split_once(":") else{
                    return;
                };

                let Some(msg) = create_error_explain_message(error_code, true) else {
                    return
                };

                component.message.edit(&ctx.http, msg).await.ok();

                component.defer(&ctx.http).await.ok();

                start_timed_explanation_collapse(*component.message, ctx.http.clone(), error_code);
            }
        }
    }
}