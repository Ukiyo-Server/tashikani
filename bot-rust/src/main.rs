mod commands;

use std::{collections::HashSet, env, sync::Arc};

use commands::GENERAL_GROUP;
use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::StandardFramework,
    http::Http,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use tracing::{error, info};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    tracing_subscriber::fmt::init();

    let token = env::var("TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    info!("Bot started");

    let mut client = Client::builder(token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }

    info!("Bot stopped");
}
