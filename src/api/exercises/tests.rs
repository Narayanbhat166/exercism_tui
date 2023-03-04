use crate::api::exercises::get_exercises;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_exercisesq() {
        let exercises = get_exercises::get_exercises("rust".to_string())
            .await
            .unwrap();
        assert!(exercises.exercises.len() > 0);
    }
}
