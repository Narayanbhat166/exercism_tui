use reqwest;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tracks {
    pub tracks: Vec<Track>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Track {
    slug: String,
    title: String,
    course: bool,
    num_concepts: i64,
    num_exercises: i64,
    web_url: String,
    icon_url: String,
    tags: Vec<String>,
    last_touched_at: Option<String>,
    is_new: bool,
    links: Links,
    is_joined: Option<bool>,
    num_learnt_concepts: Option<i64>,
    num_completed_exercises: Option<i64>,
    num_solutions: Option<i64>,
    has_notifications: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {
    #[serde(rename = "self")]
    links_self: String,
    exercises: String,
    concepts: String,
}

// Use a unified error for both reqwest error and parsing error
pub async fn get_tracks() -> Result<Tracks, reqwest::Error> {
    let client = reqwest::Client::new();
    let body = client
        .get("https://api.exercism.io/v2/tracks")
        .header(reqwest::header::AUTHORIZATION, "")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("body = {:?}", body);
    let tracks = serde_json::from_str::<Tracks>(&body).unwrap();

    Ok(tracks)
}
