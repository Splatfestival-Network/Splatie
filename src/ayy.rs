use once_cell::sync::Lazy;
use regex::Regex;
use serenity::all::{Context, EventHandler, Message};
use serenity::async_trait;

static AY_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new("(^|\\s)[Aa][Yy]+($|\\s)").expect("invalid regex"));
pub struct AyyHandler;

#[async_trait]
impl EventHandler for AyyHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot{
            return;
        }

        if AY_REGEX.is_match(&msg.content){
            let mut output = "".to_owned();
            let mut lastpos = 0;
            for ay in AY_REGEX.find_iter(&msg.content){
                output += &msg.content[lastpos..ay.start()];

                lastpos = ay.end();

                let mut str = ay.as_str();

                if str.starts_with(' '){
                    str = &str[1..];
                    output.push(' ');
                }

                let mut chars = str.chars();

                let Some(first_char) = chars.next() else{
                    return
                };

                if first_char.is_lowercase(){
                    output.push_str("lma")
                } else {
                    output.push_str("LMA")
                }

                for char in chars{
                    if char == ' '{
                        output.push(' ');
                        break;
                    }

                    if char.is_lowercase(){
                        output.push('o');
                    } else {
                        output.push('O');
                    }


                }
            }

            output += &msg.content[lastpos..];

            msg.reply(&ctx.http, output).await.ok();
        }
    }
}