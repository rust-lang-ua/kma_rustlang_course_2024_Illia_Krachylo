use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    email: String,
    birthdate: String,
}

fn main() {
    let user = User {
        name: "Illia".to_string(),
        email: "illia@example.com".to_string(),
        birthdate: "2005-04-02".to_string()
    };

    let json = serde_json::to_string(&user).expect("Serialization error!");
    println!("Serialized JSON: {}", json);

    let deserialized_json: User = serde_json::from_str(&json).expect("Deserialization error!");
    println!("Deserialized User: {:?}", deserialized_json);
}
