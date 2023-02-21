use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tracks {
    pub tracks: Vec<Track>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Track {
    slug: String,
    pub title: String,
    course: bool,
    pub num_concepts: i64,
    pub num_exercises: i64,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Links {
    #[serde(rename = "self")]
    links_self: String,
    exercises: String,
    concepts: String,
}
