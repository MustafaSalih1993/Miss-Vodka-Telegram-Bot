use rand::{seq::SliceRandom, Rng};
use serde_json::Value;
use std::env;
use teloxide::{prelude::*, requests::RequestWithFile, types::InputFile};
// TODO: orginize this mess

async fn get_rand_tumb() -> Option<String> {
    let key = env::var("TUMBLER").unwrap();
    let blogs = vec![
        "just4jk",
        "smoke-and-sexx",
        "dickvonstrangle",
        "daddys-little-sluts69",
        "moan-s",
        "ivansdirtygirl69",
        "brizar",
        "curiouskittenmask",
    ];
    let blog = blogs.choose(&mut rand::thread_rng()).unwrap();
    let url = format!(
        "https://api.tumblr.com/v2/blog/{}/posts/photo?api_key={}",
        blog, key
    );
    let response = reqwest::get(&url).await.ok()?.text().await.ok();

    if response.is_none() {
        return None;
    }

    let response = match response {
        Some(val) => val.as_str().trim().to_string(),
        None => return None,
    };
    let response: Value = serde_json::from_str(&response).unwrap();
    let posts_count: u64 = response["response"]["total_posts"].as_u64().unwrap();

    let num = rand::thread_rng().gen_range(0..(posts_count - 20) as u32);
    let url = format!(
        "https://api.tumblr.com/v2/blog/{}/posts/photo?api_key={}&offset={}",
        blog, key, num
    );
    let resp = reqwest::get(&url).await.ok()?.text().await.ok();
    let resp = match resp {
        Some(val) => val.as_str().trim().to_string(),
        None => return None,
    };

    let response: Value = serde_json::from_str(&resp).unwrap();
    let arr: Vec<Value> = match response["response"]["posts"].as_array() {
        Some(a) => a.to_vec(),
        None => return None,
    };

    let num = rand::thread_rng().gen_range(0..19);

    let photo = arr[num]["photos"][0]["original_size"]["url"].to_string();
    if photo == String::from("null") {
        return None;
    }

    Some(photo.trim_matches('"').to_string())
}

pub async fn handle_tumbler(cx: UpdateWithCx<Message>) -> ResponseResult<Message> {
    let photo = get_rand_tumb().await;
    if photo.is_none() {
        return cx.answer_str("Error Happend").await;
    } else {
        if photo.clone().unwrap().contains("community_guide") {
            return cx.answer_str("Error Happend").await;
        }

        if photo.clone().unwrap().ends_with(".gif") {
            return cx
                .answer_animation(InputFile::Url(photo.unwrap()))
                .send()
                .await
                .unwrap();
        } else {
            return teloxide::requests::RequestWithFile::send(
                &cx.answer_photo(InputFile::Url(photo.unwrap())),
            )
            .await
            .unwrap();
        }
    }
}
