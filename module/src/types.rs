use serde_json;

pub type Arguments = serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub name: String,
    pub kind: String,
    pub arguments: Arguments,
}
