use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use teloxide::prelude::*;

#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
struct Response {
    translatedText: String,
}

pub async fn handle_en(cx: UpdateWithCx<Message>, s: String) -> ResponseResult<Message> {
    if s.is_empty() {
        return cx
            .answer_str("add some Portoguese text to translate!")
            .await;
    } else {
        let data = get_translate(s, "pt".to_string(), "en".to_string()).await;
        return cx.answer_str(data.unwrap()).await;
    }
}
pub async fn handle_pr(cx: UpdateWithCx<Message>, s: String) -> ResponseResult<Message> {
    if s.is_empty() {
        return cx.answer_str("add some English text to translate!").await;
    } else {
        let data = get_translate(s, "en".to_string(), "pt".to_string()).await;
        return cx.answer_str(data.unwrap()).await;
    }
}

async fn get_translate(txt: String, source_lang: String, target_lang: String) -> Option<String> {
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

    let response = res.unwrap().text().await;
    let resp: Response = serde_json::from_str(response.unwrap().as_str()).unwrap();
    Some(resp.translatedText)
}
