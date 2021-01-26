use serde_json::Value;
use std::env;

pub async fn get_gif(txt: String) -> Option<String> {
    let key = env::var("TENOR_KEY").unwrap();
    // need somthing better
    let txt = txt
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("%20");

    println!("{:?}", txt);
    let url = format!("https://api.tenor.com/v1/random?q={}&key={}&contentfilter=off&limit=1&media_filter=minimal",txt,key);
    // let response = match minreq::get(url).send() {
    //     Ok(resp) => match resp.as_str() {
    //         Ok(s) => s.trim().to_string(),
    //         _ => "Error".to_string(),
    //     },
    //     _ => "Error".to_string(),
    // };
    let response = reqwest::get(&url).await.ok()?.text().await.ok();

    let response: Value = match serde_json::from_str(&response.unwrap()) {
        Ok(val) => val,
        _ => return None,
    };
    if response["results"].as_array().unwrap().is_empty() {
        return Some(String::from("Not Found, Try Search Somthing Else!"));
    }
    let target = &response["results"][0]["media"][0]["gif"]["url"];

    Some(target.to_string().trim_matches('"').to_string())
}
