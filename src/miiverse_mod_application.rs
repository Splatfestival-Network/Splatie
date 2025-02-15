
use crate::cheeseburger::CheeseburgerHandler;
use serenity::all::{ActivityData, ActivityType, ChannelId, Colour, Command, CommandId, Context, CreateCommand, CreateEmbed, CreateEmbedAuthor, CreateInputText, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, CreateQuickModal, EventHandler, Guild, GuildId, InputTextStyle, Interaction, Message, Ready, Timestamp};
use serenity::async_trait;
use tokio::sync::OnceCell;

#[derive(Default)]
pub struct MiiverseModApplicationHandler{
    command_id: OnceCell<CommandId>
}

#[async_trait]
impl EventHandler for MiiverseModApplicationHandler {
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        let command = CreateCommand::new("miivserse-mod-application")
            .description("Apply for miiverse moderator");

        let cmd = Command::create_global_command(&ctx.http, command).await.expect("unable to register command");

        self.command_id.set(cmd.id).ok();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let Interaction::Command(cmd) = interaction else {
            return;
        };

        let Some(id) = self.command_id.get() else {
            return;
        };

        if cmd.data.id != *id{
            return;
        }

        let modal = CreateQuickModal::new("Miiverse Moderator Application Form")
            .short_field("Whats your timezone?")
            .short_field("How long are you usually availible every day?")
            .paragraph_field("Why should we pick you?");

        let response = match cmd.quick_modal(&ctx, modal).await{
            Ok(v) => v,
            Err(e) => { eprintln!("{}", e); return }
        };

        let Some(response) = response else {
            return
        };

        response.interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .ephemeral(true)
                .content("application has been sent")
        )).await.ok();


        let emergency_channel = ChannelId::new(1324479126991802528);

        let mut author = CreateEmbedAuthor::from(response.interaction.user);

        let embed = CreateEmbed::new()
            .title("New Miiverse application")
            .field("Whats your timezone?", &response.inputs[0], false)
            .field("How long are you usually availible every day?", &response.inputs[1], false)
            .field("Why should we pick you?", &response.inputs[2], false)
            .timestamp(Timestamp::now())
            .author(author)
            .color(Colour::DARK_GREEN);

        let message = CreateMessage::new()
            .add_embed(embed);

        emergency_channel.send_message(&ctx.http, message).await.ok();


    }
}
