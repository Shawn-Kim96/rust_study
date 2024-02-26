// use reqwest::Result;
// use std::time::Duration;
// use reqwest::ClientBuilder;
use serde::{Serialize, Deserialize};
use reqwest::Error;

#[derive(Serialize, Deserialize, Debug)]
struct AtxAPIResult {
    message: String,
    datetime: String,
    weather: Weather,
    cloud: i32,
    wind: Wind,
    detail: Detail,
    address: Address,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    #[serde(rename = "type")]
    weather_type: i32,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    direction: String,
    degree: i32,
    speed: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Detail {
    temperature: f64,
    humidity: i32,
    pressure: i32,
    visibility: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    #[serde(rename = "areaLevel1")]
    area_level_1: String,
    #[serde(rename = "areaLevel2")]
    area_level_2: String,
    #[serde(rename = "areaLevel1")]
    area_level_3: String,
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!("https://atx-tap-api.int.42dot.io/tools/v1/weather/current?lat={lat}&lon={lon}",
                              lat = "37.566535",
                              lon = "126.9779692");
                            
    // println!("{:?}", request_url);
    let response = reqwest::get(&request_url).await?;
    
    // let atx_result = response.json().await?;
    // println!("{:?}", atx_result);
    
    let result = response.json().await?;
    println!("{:?}", result);
    Ok(())
}
