use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

const USAGE: &str = "
Command: /start

This is an example usage information.
";

#[command]
pub async fn start(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    Ok(())
}
