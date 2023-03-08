use base64::{engine::general_purpose, Engine};
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
    let url = format!("https://api.github.com/repos/exercism/{track_id}/contents/exercises/practice/{exercise_id}/.docs/instructions.md");
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(reqwest::header::USER_AGENT, "curl/7.86.0")
        .send()
        .await;

    Ok(match response {
        Ok(response) => {
            let response_text = response.text().await.unwrap();
            let github_response = serde_json::from_str::<models::GitHubResponse>(&response_text)
                .expect("Failed to deserialize github response");

            match github_response.content {
                Some(base64_encoded_text) => {
                    let base64_encoded_text = base64_encoded_text.replace('\n', "");

                    let decoded_text = general_purpose::STANDARD
                        .decode(base64_encoded_text)
                        .expect("Failed when decoding response");

                    String::from_utf8_lossy(&decoded_text).into()
                }
                None => String::from("Not available"),
            }
        }
        Err(_) => {
            // Call the api again?
            String::from("Not available")
        }
    })
}
