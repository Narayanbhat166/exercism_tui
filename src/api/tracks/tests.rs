use crate::api::tracks::get_tracks;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tracks() {
        let tracks = get_tracks::get_tracks().await.unwrap();
        assert!(tracks.tracks.len() > 0);
    }
}
