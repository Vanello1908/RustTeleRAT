use serde_json::json;

pub fn connected() -> String{
    return whoami::username();
}

pub async fn get_ip() -> Result<String, &'static str>{
    let res = reqwest::get("https://www.cloudflare.com/cdn-cgi/trace").await;
    match res {
        Ok(_) => {}
        Err(_) => {return Err("Getting ip error");}
    }
    let response: reqwest::Response = res.unwrap();
    let text: String = response.text().await.unwrap();
    let strings: Vec<Vec<&str>> = text.split('\n').map(|string| {string.split("=").collect()}).collect();
    return Ok(strings[2][1].to_string());

}

pub async fn get_geolocation(ip: &String) -> Result<String, &'static str>{
    let url = format!("https://ipapi.co/{}/json/", ip).to_string();
    let res = reqwest::get(url).await;
    match res {
        Ok(_) => {}
        Err(_) => {return Err("Getting geolocation error");}
    }
    let response = res.unwrap();
    let text = response.text().await.unwrap();
    let json = json!(text);
    dbg!(&json);
    return Ok(format!("
        Country: {}
        Region: {}
        City: {}",
        json.get("conutry_name").unwrap(), json.get("region").unwrap(), json.get("city").unwrap()
    ));

}


pub async fn who() -> Result<String, &'static str> {
    let ip = get_ip().await?;
    let geolocation = String::from(""); //get_geolocation(&ip).await?;
    let data: String = format!("
            Ip: {}\n\
            {}\n\
            User's Name: {}\n\
            User's Username: {}\n\
            Device's Pretty Name: {}\n\
            Device's Platform: {}\n\
            Device's OS Distro: {}\n\
            Device's CPU Arch: {}\n",
            &ip, &geolocation,
            whoami::realname(), whoami::username(), whoami::devicename(), whoami::platform(), whoami::distro(), whoami::arch()
        ).to_string();
    return Ok(data);
}