use serenity::all::{ChannelId, Colour, Command, CommandId, Context, CreateCommand, CreateEmbed, CreateEmbedAuthor, CreateInputText, CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage, CreateMessage, CreateQuickModal, EventHandler, InputTextStyle, Interaction, Message, Ready, Timestamp};
use serenity::all::Change::Color;
use serenity::async_trait;
use tokio::sync::OnceCell;

#[derive(Default)]
pub struct EmergencyReportHandler{
    command_id: OnceCell<CommandId>
}

#[async_trait]
impl EventHandler for EmergencyReportHandler {
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        let command = CreateCommand::new("emergency-report")
            .description("Report an emergency (e.g. server down) ONLY if its happening for everyone not if its just for you.");

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

        let main_field = CreateInputText::new(
            InputTextStyle::Paragraph,
            "What is happening",
            "MAIN_CONTENT"
        ).required(true).min_length(10);

        let modal = CreateQuickModal::new("Emergency Report Form")
            .field(main_field);

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
                .content("report has been sent to the admins")
        )).await.ok();

        let text = &response.inputs[0];

        let emergency_channel = ChannelId::new(1379768148420591666);

        let mut author = CreateEmbedAuthor::from(response.interaction.user);

        let embed = CreateEmbed::new()
            .title("AN EMERGENCY HAS BEEN REPORTED")
            .description(text)
            .timestamp(Timestamp::now())
            .author(author)
            .color(Colour::RED);

        let message = CreateMessage::new()
            .content("@everyone")
            .add_embed(embed);

        emergency_channel.send_message(&ctx.http, message).await.ok();
    }
}