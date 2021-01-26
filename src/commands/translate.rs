use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
struct Response {
    translatedText: String,
}

pub async fn get_translate(
    txt: String,
    source_lang: String,
    target_lang: String,
) -> Option<String> {
    let mut json_body = HashMap::new();
    json_body.insert("q", txt);
    json_body.insert("source", source_lang);
    json_body.insert("target", target_lang);

    let client = reqwest::Client::new();
    let res = client
        .post("https://libretranslate.com/translate")
        .header("Content-Type", "application/json")
        .json(&json_body)
        .send()
        .await
        .ok();

    if res.is_none() {
        return None;
    }

    // let response = res.unwrap();
    // let resp: Response = serde_json::from_str(response).unwrap();
    res.unwrap().text().await.ok()
}
