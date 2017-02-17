use serde_json;

pub type Arguments = serde_json::Value;
pub type Data = serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Error(serde_json::Value);

impl From<serde_json::Value> for Error {
    fn from(value: serde_json::Value) -> Error {
        Error(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Error {
        Error(serde_json::Value::Null)
    }
}

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
