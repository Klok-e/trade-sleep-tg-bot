use std::{future::Future, path::PathBuf, time::Instant};

use chrono::{Local, TimeZone, Timelike, Utc};
use chrono_tz::Europe::Athens;
use teloxide::{prelude::*, types::InputFile};
use tokio_stream::wrappers::UnboundedReceiverStream;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting the bot...");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(|rx| async { handle_messages(rx).await })
        .dispatch()
        .await;
}

async fn handle_messages(rx: DispatcherHandlerRx<Bot, Message>) {
    UnboundedReceiverStream::new(rx)
        .for_each_concurrent(None, |msg| async move {
            match &msg.update.kind {
                teloxide::types::MessageKind::Common(_) => {
                    let time = Utc::now().naive_utc();
                    let time = Athens.from_utc_datetime(&time);
                    let hour = time.hour();
                    let lat_at_night = 0 >= hour && hour <= 6;
                    if true {
                        msg.answer_photo(InputFile::File("resources/img.jpg".into()))
                            .send()
                            .await
                            .log_on_error()
                            .await;
                    }
                }
                _ => (),
            }
        })
        .await;
}
