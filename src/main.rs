use std::env;

use chrono::{TimeZone, Timelike, Utc};
use chrono_tz::Europe::Athens;
use teloxide::{prelude::*, types::InputFile};
use tokio_stream::wrappers::UnboundedReceiverStream;

use std::collections::HashMap;
use std::net::Ipv4Addr;
use warp::{http::Response, Filter};

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting the bot...");

    let example1 = warp::get()
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| match p.get("name") {
            Some(name) => Response::builder().body(format!("Hello, {}. This HTTP triggered function executed successfully.", name)),
            None => Response::builder().body(String::from("This HTTP triggered function executed successfully. Pass a name in the query string for a personalized response.")),
        });

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    let serve = warp::serve(example1).run((Ipv4Addr::UNSPECIFIED, port));

    let bot = Bot::from_env();

    let dispatch = async move {
        let x = Dispatcher::new(bot).messages_handler(|rx| handle_messages(rx));
        x.dispatch().await
    };

    tokio::join!(dispatch, serve);
}

async fn handle_messages(rx: DispatcherHandlerRx<Bot, Message>) {
    UnboundedReceiverStream::new(rx)
        .for_each_concurrent(None, |msg| async move {
            log::info!("{:?}", msg);
            match &msg.update.kind {
                teloxide::types::MessageKind::Common(_) => {
                    let time = Utc::now().naive_utc();
                    let time = Athens.from_utc_datetime(&time);
                    let hour = time.hour();
                    let late_at_night = 0 >= hour && hour <= 6;
                    let debug_respond = env::var("TG_BOT_TRADEOFFER_DEBUG");
                    if debug_respond.is_ok() || late_at_night {
                        let mut resp =
                            msg.answer_photo(InputFile::File("resources/img.jpg".into()));
                        resp.reply_to_message_id = Some(msg.update.id);
                        resp.send().await.log_on_error().await;
                    }
                }
                _ => (),
            }
        })
        .await;
}
