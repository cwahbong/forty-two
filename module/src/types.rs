use serde_json;

pub type Arguments = serde_json::Value;
pub type Data = serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestEvent {
    pub name: String,
    pub arguments: Arguments,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseEvent {
    pub success: bool,
    pub data: Data,
}
