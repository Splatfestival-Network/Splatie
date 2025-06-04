use sarcastic::byaml::{Byaml, Node};
use serenity::all::{ChannelId, Colour, Command, CommandId, CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedAuthor, CreateInputText, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, CreateQuickModal, EventHandler, InputTextStyle, Interaction, Ready, ResolvedValue, Timestamp};
use serenity::all::ResolvedValue::Attachment;
use serenity::async_trait;
use tokio::sync::OnceCell;
use std::fmt::Write;
use chrono::{DateTime, Utc};

#[derive(Default)]
pub struct FestFaxHandler{
    command_id: OnceCell<CommandId>
}

#[derive(Debug)]
struct TeamData{
    color: [f32; 4],
    name: String
}

fn read_team_data(node: &Node<'_>) -> Option<TeamData>{
    let Node::DictionaryNode(node) = node else{
        return None;
    };

    let Some(Node::StringValue(color)) = node.find("Color") else{
        return None;
    };

    let Some(color): Option<[f32; 4]> = color.split(',')
        .filter_map(|v| v.parse::<f32>().ok())
        .collect::<Vec<f32>>().try_into().ok() else {
        return None;
    };

    let Some(Node::DictionaryNode(names)) = node.find("Name") else{
        return None;
    };

    let Some(Node::StringValue(name)) = names.find("EUen") else{
        return None;
    };

    Some(TeamData{
        name: name.to_string(),
        color
    })
}

#[async_trait]
impl EventHandler for FestFaxHandler {
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        let command = CreateCommand::new("process-fest-in-fax-machine")
            .add_option(CreateCommandOption::new(CommandOptionType::String, "fes_tex_url", "Texture url")
                .required(true))
            .add_option(CreateCommandOption::new(CommandOptionType::String, "details", "Details")
                .required(true))
            .description("Announce a fest!");

        let cmd = Command::create_global_command(&ctx.http, command).await.expect("unable to register command");

        self.command_id.set(cmd.id).ok();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let Interaction::Command(command_interaction) = interaction else {
            return;
        };

        let Some(command_id) = self.command_id.get() else{
            return;
        };

        if command_interaction.data.id != *command_id{
            return
        }

        if command_interaction.user.id != 400291421799710720 {
            command_interaction.create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("nuh uh(cnd still isnt up so im using this to test for perms lol)")
                )).await.ok();
            return;
        }


        let options = command_interaction.data.options();


        let Ok(byaml_data) = reqwest::get("https://dl.app.spfn.cc/p01/data/1/Festival.byaml").await else{
            return;
        };

        let Ok(byaml_data) = byaml_data.bytes().await else{
            return;
        };

        macro_rules! byaml_handle_error {
            ($($stuff:tt)*) => {
                match $($stuff)*{
                    Ok(v) => v,
                    Err(e) => {
                        command_interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .content(format!("an error ocurred whilest reading the byaml: {}", e)))).await.ok();
                        return;
                    }
                }
            };
        }

        let byaml = byaml_handle_error!(Byaml::new(&byaml_data));
        let root_node = byaml_handle_error!(byaml.get_root_node());

        let Node::DictionaryNode(dict) = root_node else {
            return;
        };

        let Some(Node::ArrayNode(teams)) = dict.find("Teams") else {
            return
        };

        let mut iter = teams.into_iter()
            .filter_map(|v| v.ok());

        let Some(alpha) = iter.next() else {
            return;
        };

        let Some(bravo) = iter.next() else {
            return;
        };

        let Some(alpha) = read_team_data(&alpha) else{
            return;
        };
        let Some(bravo) = read_team_data(&bravo) else{
            return;
        };


        let Some(Node::DictionaryNode(times)) = dict.find("Time") else {
            return
        };

        let Some(Node::StringValue(start_time)) = times.find("Start") else {
            return
        };

        let Ok(start_time) = DateTime::parse_from_rfc3339(start_time) else{
            return;
        };

        let Some(Node::StringValue(end_time)) = times.find("End") else {
            return
        };

        let Ok(end_time) = DateTime::parse_from_rfc3339(end_time) else{
            return;
        };

        let Some(fest_image) = options.iter().find(|o| o.name == "fes_tex_url") else{
            return;
        };

        let ResolvedValue::String(fest_image) = fest_image.value else {
            return;
        };


        let Some(fest_file) = options.iter().find(|o| o.name == "details") else{
            return;
        };

        let ResolvedValue::String(details) = fest_file.value else {
            return;
        };


        command_interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .embed(
                    CreateEmbed::new()
                        .image(fest_image)
                        .field("Theme", format!("{} vs {}", alpha.name, bravo.name), false)
                        .field("Starts", format!("<t:{}:R>", start_time.timestamp()), false)
                        .field("End", format!("<t:{}:f>", end_time.timestamp()), true)
                        .field("Details", details, true)
                )
            )
        ).await.ok();
    }
}