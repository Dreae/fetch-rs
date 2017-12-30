use stdweb::unstable::TryFrom;
use serde::de::DeserializeOwned;
use serde_json;

use status::StatusCode;

#[derive(Serialize, Deserialize)]
pub struct Response {
    status_code: u16,
    status_text: String,
    text: String
}

impl Response {
    pub fn status(&self) -> StatusCode {
        StatusCode::from_u16(self.status_code)
    }

    pub fn json<T: DeserializeOwned>(&self) -> serde_json::Result<T> {
        serde_json::from_str(&self.text)
    }

    pub fn text(&self) -> &str {
        return &self.text;
    }
}

js_deserializable!(Response);