use super::post::{post, PostError, Request};
use serde::Serialize;
use serde_json::Value;
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
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum MessageObject {
    Text { text: String },
    Flex(Value),
}

#[derive(Serialize, Debug)]
pub struct TextMessage {
    text: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReplyMessageBody {
    reply_token: String,
    messages: Vec<MessageObject>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PushMessageBody {
    to: String,
    messages: Vec<MessageObject>,
}

impl LineClient {
    pub async fn reply_message(
        &self,
        message: MessageObject,
        reply_token: String,
    ) -> Result<(), PostError> {
        let body = ReplyMessageBody {
            reply_token: reply_token,
            messages: vec![message],
        };
        self.post("https://api.line.me/v2/bot/message/reply", body)
            .await?;
        Ok(())
    }

    pub async fn push_message(&self, message: MessageObject) -> Result<(), PostError> {
        let body = PushMessageBody {
            to: self.target_group_id.clone(),
            messages: vec![message],
        };
        self.post("https://api.line.me/v2/bot/message/push", body)
            .await?;
        Ok(())
    }

    async fn post<T>(&self, url: &str, body: T) -> Result<(), PostError>
    where
        T: Serialize,
    {
        let mut headers = HashMap::new();
        headers.insert(
            "Authorization".to_string(),
            format!("Bearer {}", self.channel_access_token),
        );
        headers.insert("Content-type".to_string(), "application/json".to_string());
        let req = Request {
            url: url.to_string(),
            headers: headers,
            body: body,
        };
        post(req).await?;
        Ok(())
    }
}
