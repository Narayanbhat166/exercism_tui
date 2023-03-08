use serde::Deserialize;
use serde::Serialize;
use strum;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tracks {
    pub tracks: Vec<Track>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Track {
    pub slug: String,
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
    links_self: Option<String>,
    exercises: Option<String>,
    concepts: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Exercises {
    pub exercises: Vec<Exercise>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Exercise {
    pub slug: String,
    #[serde(rename = "type")]
    exercise_type: Type,
    pub title: String,
    icon_url: String,
    pub difficulty: Difficulty,
    pub blurb: String,
    is_external: bool,
    is_unlocked: bool,
    is_recommended: bool,
    links: Links,
}

#[derive(Serialize, Deserialize, Clone, strum::Display)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Easy,
    Hard,
    Medium,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Concept,
    Practice,
    Tutorial,
}

#[derive(Deserialize)]
pub struct GitHubResponse {
    pub content: Option<String>,
}
