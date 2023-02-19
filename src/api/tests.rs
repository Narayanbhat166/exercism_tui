use super::tracks;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tracks() {
        let tracks = tracks::get_tracks().await.unwrap();
        assert!(tracks.tracks.len() > 0);
    }
}
