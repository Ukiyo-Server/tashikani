use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

const USAGE: &str = "
Command: /time

This is an example usage information.
";

#[command]
pub async fn time(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    Ok(())
}
