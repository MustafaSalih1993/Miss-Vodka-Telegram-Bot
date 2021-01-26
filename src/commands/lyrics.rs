use serde_json::Value;
use std::env;
use teloxide::{prelude::*, utils::html};

async fn get_lyrics(s: String) -> Option<String> {
    // parsing starts here
    if s.is_empty() {
        return Some(String::from("artist or song name required you asshole!"));
    };

    let mut s = s.split('-');
    let artist: String = s
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|c| match c {
            ' ' => '_',
            _ => c,
        })
        .collect();
    let song: String = s
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|c| match c {
            ' ' => '_',
            _ => c,
        })
        .collect();

    if artist.is_empty() || song.is_empty() {
        return Some(String::from("artist or song should not be empty"));
    }
    // parsing ends here

    let key = match env::var("MUSIX_KEY") {
        Ok(key) => key,
        Err(_) => return None,
    };
    // getting track id
    let url = format!(
        "http://api.musixmatch.com/ws/1.1/track.search?q_artist={}&q_track={}&apikey={}",
        artist, song, key
    );
    let response = reqwest::get(&url)
        .await
        .ok()?
        .text()
        .await
        .ok()
        .unwrap()
        .trim()
        .to_string();

    let response: Value = match serde_json::from_str(&response) {
        Ok(data) => data,
        _ => return Some(String::from("Shit happend while parsing lyrics")),
    };
    match response["message"]["body"]["track_list"][0]["track"]["has_lyrics"].as_u64() {
        Some(val) => {
            if val == 0 {
                return Some(String::from("Song dont have lyrics."));
            }
        }
        _ => return Some(String::from("Not Found!")),
    };

    let track_id = &response["message"]["body"]["track_list"][0]["track"]["track_id"]
        .as_u64()
        .unwrap();
    let track_artist =
        &response["message"]["body"]["track_list"][0]["track"]["artist_name"].to_string();

    // getting track lyrics
    let url = format!(
        "http://api.musixmatch.com/ws/1.1/track.lyrics.get?track_id={}&apikey={}",
        track_id, key
    );
    let response = reqwest::get(&url)
        .await
        .ok()?
        .text()
        .await
        .ok()
        .unwrap()
        .trim()
        .to_string();

    let response: Value = serde_json::from_str(&response).unwrap();
    let lyrics: String = response["message"]["body"]["lyrics"]["lyrics_body"]
        .to_string()
        .replace("\\n", "\n")
        .lines()
        .filter(|l| !l.starts_with("***") && !l.starts_with("("))
        .collect();

    Some(format!(
        "{}\n\n{}",
        html::bold(&html::underline(&track_artist.trim_matches('"'))),
        html::italic(&lyrics.trim_matches('"'))
    ))
}
pub async fn handle_lyrics(cx: UpdateWithCx<Message>, s: String) -> ResponseResult<Message> {
    let lyrics_data = get_lyrics(s).await;
    if lyrics_data.is_some() {
        cx.answer(lyrics_data.unwrap())
            .parse_mode(teloxide::types::ParseMode::HTML)
            .send()
            .await
    } else {
        cx.answer_str("something wrong, try somthing else").await
    }
}
