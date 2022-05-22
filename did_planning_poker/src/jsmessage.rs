use didcomm_rs::Message;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JsMessage {
    pub id: String,
    #[serde(rename = "type")]
    pub m_type: String,
    pub thid: Option<String>,
}

impl Into<JsMessage> for Message {
    fn into(self) -> JsMessage {
        JsMessage {
            id: self.get_didcomm_header().id.to_string(),
            m_type: self.get_didcomm_header().m_type.to_string(),
            thid: self.get_didcomm_header().thid.clone(),
        }
    }
}
