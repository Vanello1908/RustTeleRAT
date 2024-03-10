#![windows_subsystem = "windows"]

#[tokio::main]
async fn main(){
    lib::start().await;
}
