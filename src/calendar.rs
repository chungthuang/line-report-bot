use super::{BotConfig, LineClient, MessageObject};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::Request;

/*
    CalendarStartEvent models the request body sent from IFTTT when a Google Calendar
    event starts
*/
#[derive(Deserialize)]
struct CalendarStartEvent {
    title: String,
    description: String,
}

impl CalendarStartEvent {
    fn compose_message(self) -> String {
        return format!("{}\n填寫回報表單:{}", self.title, self.description);
    }
}

pub async fn calendar_start(req: Request, bot_config: BotConfig) -> Result<(), JsValue> {
    let line_client = LineClient {
        channel_access_token: bot_config.channel_access_token,
        target_group_id: bot_config.target_group_id,
    };

    let body = JsFuture::from(req.json()?).await?;
    let event: CalendarStartEvent = body.into_serde().map_err(|e| e.to_string())?;

    let report = MessageObject::Text {
        text: event.compose_message(),
    };
    line_client.push_message(report).await?;
    Ok(())
}
