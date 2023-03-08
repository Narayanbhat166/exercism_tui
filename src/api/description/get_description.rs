use reqwest;

use crate::api::models;

// #[cfg(feature = "mock")]
// pub async fn get_exercises(_track_id: String) -> Result<models::Exercises, reqwest::Error> {
//     let file_contents = std::include_bytes!("mock_response.json");
// Simulate the api call delay
//     std::thread::sleep(std::time::Duration::from_secs(1));
//     let mock_exercises = serde_json::from_slice::<models::Exercises>(file_contents).unwrap();
//     Ok(mock_exercises)
// }

// Use a unified error for both reqwest error and parsing error
pub async fn get_description(
    track_id: String,
    exercise_id: String,
) -> Result<String, reqwest::Error> {
    let url = format!("https://raw.githubusercontent.com/exercism/{track_id}/main/exercises/practice/{exercise_id}/.docs/instructions.md");
    // let auth_key = std::env::var("AUTH_TOKEN").expect("AUTH_TOKEN must be provided");
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        // .header(reqwest::header::AUTHORIZATION, auth_key)
        .send()
        .await
        .unwrap();

    let text = response.text().await.unwrap();

    Ok(text)
}
