use std::path::PathBuf;
use nokhwa::{pixel_format::{RgbAFormat, RgbFormat}, utils::{CameraIndex, RequestedFormat, RequestedFormatType}, Camera};

pub fn create_photo(path: &PathBuf) -> Result<(), &'static str>{
    let index = CameraIndex::Index(0); 
    let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestResolution);
    let res = Camera::new(index, requested);
    if res.is_err(){
        return Err("Camera error");
    }
    let mut camera = res.unwrap();
    let frame = camera.frame().unwrap();
    let decoded = frame.decode_image::<RgbAFormat>().unwrap();
    match decoded.save(path){
        Ok(_) => {},
        Err(_) => return Err("Photo write error")
    }
    Ok(())
}

