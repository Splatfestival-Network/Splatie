
use crate::cheeseburger::CheeseburgerHandler;
use serenity::all::{ActivityData, ActivityType, Command, CommandId, Context, CreateCommand, CreateInputText, CreateQuickModal, EventHandler, Guild, GuildId, InputTextStyle, Interaction, Message, Ready};
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

        let main_field = CreateInputText::new(
            InputTextStyle::Paragraph,
            "Text",
            "MAIN_CONTENT"
        );

        let modal = CreateQuickModal::new("Miiverse Moderator Application Form")
            .field(main_field);

        let response = match cmd.quick_modal(&ctx, modal).await{
            Ok(v) => v,
            Err(e) => { eprintln!("{}", e); return }
        };

        let Some(response) = response else {
            return
        };


    }
}
