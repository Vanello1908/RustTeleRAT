use config::Config;
use scrapper::scrap_telegram;

#[tokio::main]
async fn main(){
    let cfg = Config::init();
    scrap_telegram(&cfg.telegram_path, &cfg.telegram_zip_path, &cfg.local_path).unwrap();
}