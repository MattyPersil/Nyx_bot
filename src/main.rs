use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::Client;
use std::env;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "Bonjour" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "Salut !").await {
                eprintln!("Erreur en envoyant le message : {:?}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN env variable");;
    
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Erreur lors de la création du client");

    if let Err(e) = client.start().await {
        eprintln!("Erreur du bot : {:?}", e);
    }
}
