use rand::{seq::SliceRandom, Rng};
use serde_json::Value;
use std::env;
use teloxide::{prelude::*, requests::RequestWithFile, types::InputFile};

pub async fn handle_video(cx: UpdateWithCx<Message>, s: String) -> ResponseResult<Message> {
    let video = get_video(s).await;
    if video.is_none() {
        return cx.answer_str("Error Happend").await;
    } else {
        return cx
            .answer_video(InputFile::Url(video.unwrap()))
            .send()
            .await
            .unwrap();
    }
}

async fn get_video(s: String) -> Option<String> {
    if s.is_empty() {
        return None;
    }

    let num = rand::thread_rng().gen_range(0..15);
    let url = format!(
        "https://api.pexels.com/videos/search?per_page=80&page={}&query={}",
        num, s
    );

    let client = reqwest::Client::new();
    let key = env::var("PEXELS_KEY").unwrap();
    let resp = client
        .get(&url)
        .header("Authorization", key)
        .send()
        .await
        .ok()
        .unwrap()
        .text()
        .await
        .ok();

    if resp.is_none() {
        return None;
    }

    let resp: Value = serde_json::from_str(&resp.unwrap()).unwrap();
    let videos = resp.as_object().unwrap()["videos"].as_array().unwrap();
    let video = videos
        .choose(&mut rand::thread_rng())
        .unwrap()
        .as_object()
        .unwrap()["video_files"]
        .as_array()
        .unwrap()[0]
        .as_object()
        .unwrap()["link"]
        .as_str()
        .unwrap()
        .trim_matches('"')
        .to_string();
    // let video = videos[0].as_object().unwrap()["video_files"]
    //     .as_array()
    //     .unwrap()[0]
    //     .as_object()
    //     .unwrap()["link"]
    //     .as_str()
    //     .unwrap()
    //     .trim_matches('"')
    //     .to_string();

    Some(video)
}
