use crate::api::description::get_description;

#[cfg(test)]
mod tests {

    use crate::api::description::get_description::get_description;

    use super::*;

    #[tokio::test]
    async fn test_description() {
        let exercise_description =
            get_description::get_description("rust".to_string(), "anagram".to_string())
                .await
                .unwrap();
        assert!(exercise_description.len() > 0);
    }
}
