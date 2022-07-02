use std::env;
use teloxide::{dispatching::update_listeners::webhooks, prelude::*};
use url::Url;

#[tokio::main]
async fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let bot = Bot::new(&token).auto_send();

    let port: u16 = env::var("PORT")
        .expect("PORT env variable is not set")
        .parse()
        .expect("PORT env variable value is not an integer");
    println!("Port is set to: {}", port);

    let addr = ([0, 0, 0, 0], port).into();

    let host = env::var("HOST").expect("HOST env variable is not set");
    let url = Url::parse(&format!("https://{host}/webhooks/{token}")).unwrap();
    println!("Url is set to {}", url);

    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("Couldn't setup webhook");

    println!("Setup complete!");

    teloxide::repl_with_listener(
        bot,
        |message: Message, bot: AutoSend<Bot>| async move {
            let was_sent_by_premium = message.from().map_or(false, |u| u.is_premium);
            if message.sticker().map_or(false, |s| s.premium_animation.is_some()) {
                let user = message.from().unwrap();
                bot.delete_message(message.chat.id, message.id).await?;
                bot.send_message(
                    message.chat.id,
                    format!(
                        "Ciao {}, le stronzate premium non le usi qua dentro",
                        user.mention().unwrap_or(user.full_name())
                    )
                ).await?;
            }
            respond(())
        },
        listener,
    )
    .await;
}
