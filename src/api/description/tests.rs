#[cfg(test)]
mod tests {

    use crate::api::description::get_description::get_description;
    use crate::custom_widgets::description::parse_markdown;

    #[tokio::test]
    async fn test_description() {
        let exercise_description = get_description("rust".to_string(), "clock".to_string())
            .await
            .unwrap();

        let parsed_markdown =
            minimad::parse_text(&exercise_description, minimad::Options::default());
        dbg!(parsed_markdown);
        // dbg!(exercise_description.clone());
        assert!(exercise_description.len() > 0);
    }
}
