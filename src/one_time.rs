#![windows_subsystem = "windows"]

#[tokio::main]
async fn main(){
    lib::send_all().await;
}