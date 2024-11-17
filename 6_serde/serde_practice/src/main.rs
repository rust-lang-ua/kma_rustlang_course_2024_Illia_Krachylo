use serde::{Deserialize, Serialize};
use serde_yaml::to_string as to_yaml;
use chrono::{DateTime, Utc};
use url::Url;
use uuid::Uuid;
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use toml::to_string as to_toml;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    email: String,
    birthdate: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PublicTariff {
    id: u32,
    price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PrivateTariff {
    client_price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Stream {
    user_id: Uuid,
    is_private: bool,
    settings: u32,
    shard_url: Url,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Debug, Serialize, Deserialize)]
struct Gift {
    id: u32,
    price: u32,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Debug {
    #[serde(with = "humantime_serde")]
    duration: Duration,
    at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
enum RequestType {
    #[serde(rename = "success")]
    Success,
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    #[serde(rename = "type")]
    request_type: RequestType,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: Debug,
}

fn main() {
    // let user = User {
    //     name: "Illia".to_string(),
    //     email: "illia@example.com".to_string(),
    //     birthdate: "2005-04-02".to_string()
    // };

    // let json = serde_json::to_string(&user).expect("Serialization error!");
    // println!("Serialized JSON: {}", json);

    // let deserialized_json: User = serde_json::from_str(&json).expect("Deserialization error!");
    // println!("Deserialized User: {:?}", deserialized_json);

    let mut file = File::open("./request.json").unwrap();
    let mut json_str = String::new();

    file.read_to_string(&mut json_str).unwrap();
    let request: Request = serde_json::from_str(&json_str).unwrap();

    let yaml_str = to_yaml(&request).unwrap();
    println!("YAML:\n{}", yaml_str);

    let toml_str = to_toml(&request).unwrap();
    println!("TOML:\n{}", toml_str);
}
