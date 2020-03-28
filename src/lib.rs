extern crate cfg_if;
extern crate ring;
#[macro_use]
extern crate serde_derive;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use ring::error::Unspecified;
use ring::hmac;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

type Headers = HashMap<String, String>;

#[derive(Deserialize, Debug)]
struct BotConfig {
    channel_secret: String,
}

#[wasm_bindgen]
pub fn collect_report(
    event: JsValue,
    headers: JsValue,
    bot_config: JsValue,
) -> Result<JsValue, JsValue> {
    let config: BotConfig = bot_config.into_serde().map_err(|e| e.to_string())?;
    let headers: Headers = headers.into_serde().map_err(|e| e.to_string())?;
    let signature = headers
        .get("X-Line-Signature")
        .ok_or("Request headers didn't include X-Line-Signature")?;
    verify_request(
        signature.to_string(),
        config.channel_secret,
        event
            .as_string()
            .ok_or("Request body can't be encoded as string")?
            .as_bytes(),
    )
    .map_err(|_| "Failed to verify request signature")?;
    Ok(JsValue::TRUE)
}

/*
    Verifies request comes from Line
    https://developers.line.biz/en/docs/messaging-api/receiving-messages/#verifying-signatures
*/
fn verify_request(
    signature: String,
    channel_secret: String,
    body: &[u8],
) -> Result<(), Unspecified> {
    let key = hmac::Key::new(hmac::HMAC_SHA256, channel_secret.as_bytes());
    hmac::verify(&key, body, signature.as_bytes())?;
    Ok(())
}
