use serde::{Deserialize, Serialize};
use serde_json::Value;
use teloxide::{
    prelude::{Request, ResponseResult, UpdateWithCx},
    types::Message,
    utils::html,
};

#[derive(Deserialize, Serialize)]
struct WikiBody {
    title: String,
    body: String,
}

// swiki for Short wiki
async fn get_swiki(s: String) -> Option<String> {
    if s.is_empty() {
        return Some(String::from("Consider to add text to search for!"));
    }
    let s: String = s
        .chars()
        .map(|x| match x {
            ' ' => '_',
            _ => x,
        })
        .collect();

    let url = format!("https://en.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&format=json&srlimit=1",s);
    let response = reqwest::get(&url).await.ok()?.text().await.ok();

    if response.is_none() {
        return None;
    }

    let response = response.unwrap().trim().to_string();

    let response: Value = serde_json::from_str(&response).unwrap();
    let parrsed: String = response["query"]["search"][0]["snippet"]
        .to_string()
        .replace("<span", "<b")
        .replace("</span>", "</b>")
        .replace("class=\\\"searchmatch\\\"", "")
        .trim_matches('"')
        .to_string();

    let wiki_body = WikiBody {
        title: format!(
            "<b>{}</b>",
            response["query"]["search"][0]["title"].to_string()
        ),
        body: format!("{}...", parrsed),
    };

    Some(wiki_body.body)
}

// lwiki for Long wiki
async fn get_lwiki(s: String) -> Option<String> {
    if s.is_empty() {
        return Some(String::from("Consider to add a text to search!"));
    }
    let s: String = s
        .chars()
        .map(|x| match x {
            ' ' => '_',
            _ => x,
        })
        .collect();
    let url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&prop=cirrusdoc&titles={}&format=json",
        s
    );

    let response = reqwest::get(&url).await.ok()?.text().await.ok();

    if response.is_none() {
        return None;
    }

    let response = response.unwrap().trim().to_string();

    let response: Value = serde_json::from_str(&response).unwrap();
    let text = &response["query"]["pages"]
        .as_object()
        .unwrap()
        .values()
        .next()
        .unwrap()["cirrusdoc"][0]["source"]["opening_text"]
        .to_string();

    let title = &response["query"]["pages"]
        .as_object()
        .unwrap()
        .values()
        .next()
        .unwrap()["title"]
        .to_string();

    if text == "null" {
        return Some(String::from("I couldnt find that!"));
    } else {
        return Some(format!(
            "{} \n\n {}",
            html::underline(&html::bold(&html::code_block(title.trim_matches('"')))),
            text.trim_matches('"')
        ));
    };
}
pub async fn handle_lwiki(cx: UpdateWithCx<Message>, s: String) -> ResponseResult<Message> {
    let lwiki_data = get_lwiki(s).await;
    if lwiki_data.is_none() {
        cx.answer_str("couldnt find that").await
    } else {
        cx.answer(lwiki_data.unwrap())
            .parse_mode(teloxide::types::ParseMode::HTML)
            .send()
            .await
    }
}

pub async fn handle_swiki(cx: UpdateWithCx<Message>, s: String) -> ResponseResult<Message> {
    let swiki_data = get_swiki(s).await;
    if swiki_data.is_none() {
        cx.answer_str("couldnt find that").await
    } else {
        cx.answer(swiki_data.unwrap())
            .parse_mode(teloxide::types::ParseMode::HTML)
            .send()
            .await
    }
}
