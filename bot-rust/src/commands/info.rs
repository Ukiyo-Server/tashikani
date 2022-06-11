use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

const USAGE: &str = "
Command: /info

This is an example usage information.
";

#[command]
pub async fn info(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    Ok(())
}
