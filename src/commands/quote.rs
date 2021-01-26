use serde_json::Value;
use teloxide::utils::html;

pub async fn get_random_quote() -> Option<String> {
    let url = format!("https://quote-garden.herokuapp.com/api/v3/quotes/random");
    // let res = match minreq::get(url).send() {
    //     Ok(resp) => match resp.as_str() {
    //         Ok(st) => st.trim().to_string(),
    //         Err(_) => "FUCK! Some Error Happend".to_string(),
    //     },
    //     Err(e) => e.to_string(),
    // };
    let res = reqwest::get(&url).await.ok()?.text().await.ok();

    let res = if res.is_none() {
        return None;
    } else {
        res.unwrap()
    };

    let res: Value = serde_json::from_str(&res).unwrap();
    let author = &res["data"].as_array().unwrap()[0]["quoteAuthor"]
        .to_string()
        .trim_matches('"')
        .to_string();
    let text = &res["data"].as_array().unwrap()[0]["quoteText"].to_string();

    Some(format!(
        "{} \n ( {} )",
        html::code_block(text),
        html::bold(&html::underline(author))
    ))
}
