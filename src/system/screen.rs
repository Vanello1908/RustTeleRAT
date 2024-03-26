use std::path::PathBuf;
use image::RgbaImage;

pub fn capture_screen(path: &PathBuf) -> Result<(), &'static str>{
    let buf = win_screenshot::capture::capture_display().unwrap();
    let img = RgbaImage::from_raw(buf.width, buf.height, buf.pixels).unwrap();
    match img.save(path){
        Ok(_) => {return Ok(());},
        Err(_) => {return Err("Screenshot error");}
    };
}