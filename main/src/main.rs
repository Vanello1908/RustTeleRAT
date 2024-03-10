use config::Config;
use scrapper::scrap_telegram;
use utils::check_one_current_process;

#[tokio::main]
async fn main(){
    check_one_current_process();
    loop {}
}