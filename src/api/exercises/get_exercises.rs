use reqwest;

use crate::api::models;

#[cfg(feature = "mock")]
pub async fn get_exercises(track_id: String) -> Result<models::Exercises, reqwest::Error> {
    let file_contents = std::include_bytes!("mock_response.json");
    // Simulate the api call delay
    std::thread::sleep(std::time::Duration::from_secs(1));
    let mock_exercises = serde_json::from_slice::<models::Exercises>(file_contents).unwrap();
    Ok(mock_exercises)
}

#[cfg(feature = "live")]
// Use a unified error for both reqwest error and parsing error
pub async fn get_exercises(track_id: String) -> Result<models::Exercises, reqwest::Error> {
    let auth_key = std::env::var("AUTH_TOKEN").expect("AUTH_TOKEN must be provided");
    let client = reqwest::Client::new();
    let body = client
        .get("https://api.exercism.io/v2/tracks/{}/exercises")
        .header(reqwest::header::AUTHORIZATION, auth_key)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let exercises = serde_json::from_str::<models::Exercises>(&body).unwrap();

    Ok(exercises)
}
