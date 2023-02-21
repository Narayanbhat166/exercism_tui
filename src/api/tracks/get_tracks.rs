use reqwest;

use crate::api::models;

#[cfg(feature = "mock")]
pub async fn get_tracks() -> Result<models::Tracks, reqwest::Error> {
    let file_contents = std::include_bytes!("mock_response.json");
    // Simulate the api call delay
    std::thread::sleep(std::time::Duration::from_secs(1));
    let mock_tracks = serde_json::from_slice::<models::Tracks>(file_contents).unwrap();
    Ok(mock_tracks)
}

#[cfg(feature = "live")]
// Use a unified error for both reqwest error and parsing error
pub async fn get_tracks() -> Result<models::Tracks, reqwest::Error> {
    let auth_key = std::env::var("AUTH_TOKEN").expect("AUTH_TOKEN must be provided");
    let client = reqwest::Client::new();
    let body = client
        .get("https://api.exercism.io/v2/tracks")
        .header(reqwest::header::AUTHORIZATION, auth_key)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let tracks = serde_json::from_str::<models::Tracks>(&body).unwrap();

    Ok(tracks)
}
