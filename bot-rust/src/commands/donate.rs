use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

const USAGE: &str = "
Command: /donate

This is an example usage information.
";

#[command]
pub async fn donate(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    Ok(())
}
