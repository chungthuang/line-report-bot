extern crate cfg_if;
extern crate hmac;
#[macro_use]
extern crate serde_derive;
extern crate sha2;
extern crate url;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate web_sys;

mod calendar;
mod flex_messages;
mod line;
mod post;
mod route;
mod utils;
mod webhooks;

use calendar::calendar_start;
use cfg_if::cfg_if;
use flex_messages::new_user_message;
use hmac::{Hmac, Mac};
use line::{BotConfig, LineClient, MessageObject, TextMessage};
use route::Route;
use sha2::Sha256;
use url::Url;
use utils::console_log;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::Request;
use webhooks::webhooks;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub async fn collect_report(
    req: JsValue,
    bot_config: JsValue,
    forms_config: JsValue,
) -> Result<JsValue, JsValue> {
    let bot_config: BotConfig = bot_config.into_serde().map_err(|e| e.to_string())?;
    let forms_config: FormsConfig = forms_config.into_serde().map_err(|e| e.to_string())?;
    let req = Request::from(req);
    let url_str = req.url();
    let url = Url::parse(&url_str).map_err(|_| format!("{:?} is not a valid url", url_str))?;

    let path = url.path();
    let result = match Route::from(path) {
        Route::CalendarStart => calendar_start(req, bot_config).await,
        Route::CalendarEnd => report(req, bot_config).await,
        Route::Webhooks => webhooks(req, bot_config, forms_config).await,
        Route::Submit => submit(req, bot_config).await,
        Route::Unhandled => Err(unhandled(path)),
    }?;

    Ok(JsValue::TRUE)
}

async fn report(req: Request, bot_config: BotConfig) -> Result<(), JsValue> {
    let signature = req.headers().get("X-Line-Signature")?;
    let body = JsFuture::from(req.json()?).await?;
    console_log(&format!("req body {:?}", body));
    /*verify_request(
        signature.to_string(),
        config.channel_secret,
        event
            .as_string()
            .ok_or("Request body can't be encoded as string")?
            .as_bytes(),
    )?;*/
    Ok(())
}

async fn submit(req: Request, bot_config: BotConfig) -> Result<(), JsValue> {
    let body = JsFuture::from(req.text()?).await?;
    console_log(&format!("receive submission {:?}", body));
    let line_client = LineClient {
        channel_access_token: bot_config.channel_access_token,
        target_group_id: bot_config.target_group_id,
    };

    let report = MessageObject::Text {
        text: body
            .as_string()
            .ok_or("/submit receive body that is not text")?,
    };
    line_client.push_message(report).await?;
    console_log("pushed message");
    Ok(())
}

fn unhandled(path: &str) -> JsValue {
    JsValue::from_str(&format!("No handler defined for {:?}", path))
}
/*
    Verifies request comes from Line
    https://developers.line.biz/en/docs/messaging-api/receiving-messages/#verifying-signatures
*/
fn verify_request(signature: String, channel_secret: String, body: &[u8]) -> Result<(), JsValue> {
    let mut mac =
        Hmac::<Sha256>::new_varkey(channel_secret.as_bytes()).map_err(|_| "Invalid sign key")?;
    mac.input(body);
    mac.verify(signature.as_bytes())
        .map_err(|_| "Request signature mismatch")?;
    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct FormsConfig {
    new_user_form: String,
}
