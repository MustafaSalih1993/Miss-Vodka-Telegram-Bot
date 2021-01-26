use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
struct WikiBody {
    title: String,
    body: String,
}

// swiki for Short wiki
fn get_swiki(s: String) -> String {
    if s.is_empty() {
        return String::from("Consider adding a text to search you dumb fuck!");
    }
    let s: String = s
        .chars()
        .map(|x| match x {
            ' ' => '_',
            _ => x,
        })
        .collect();
    let url = format!("https://en.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&format=json&srlimit=1",s);
    let response = match minreq::get(url).send() {
        Ok(resp) => match resp.as_str() {
            Ok(s) => s.trim().to_string(),
            _ => "Error".to_string(),
        },
        _ => "Error".to_string(),
    };
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

    wiki_body.body
}

use teloxide::utils::html;
// lwiki for Long wiki
fn get_lwiki(s: String) -> String {
    if s.is_empty() {
        return String::from("Consider adding a text to search you dumb fuck!");
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
    let response = match minreq::get(url).send() {
        Ok(resp) => match resp.as_str() {
            Ok(s) => s.trim().to_string(),
            _ => "Error".to_string(),
        },
        _ => "Error".to_string(),
    };
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
        return String::from("I didnt find shit about what you want!");
    } else {
        return format!(
            "{} \n\n {}",
            html::underline(&html::bold(&html::code_block(title.trim_matches('"')))),
            text.trim_matches('"')
        );
    };
}
