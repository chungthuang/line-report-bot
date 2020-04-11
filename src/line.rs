use super::post::{post, PostError, Request};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct BotConfig {
    pub channel_secret: String,
    pub channel_access_token: String,
    pub target_group_id: String,
}

pub struct LineClient {
    pub channel_access_token: String,
    pub target_group_id: String,
}

#[derive(Serialize, Debug)]
pub struct TextMessage {
    r#type: String,
    text: String,
}

impl TextMessage {
    pub fn new(text: String) -> TextMessage {
        TextMessage {
            r#type: "text".to_string(),
            text: text,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct PushMessageBody {
    to: String,
    messages: Vec<TextMessage>,
}

impl LineClient {
    pub async fn push_message(&self, message: TextMessage) -> Result<(), PostError> {
        let mut headers = HashMap::new();
        headers.insert(
            "Authorization".to_string(),
            format!("Bearer {}", self.channel_access_token),
        );
        headers.insert("Content-type".to_string(), "application/json".to_string());
        let req = Request {
            url: "https://api.line.me/v2/bot/message/push".to_string(),
            headers: headers,
            body: PushMessageBody {
                to: self.target_group_id.clone(),
                messages: vec![message],
            },
        };
        post(req).await?;
        Ok(())
    }
}
