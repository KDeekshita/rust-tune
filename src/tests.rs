use super::*;

#[test]
fn search_empty_query_returns_empty() {
    let results = search_songs("");
    assert!(results.is_empty());
}

#[test]
fn search_by_title_case_insensitive() {
    let results = search_songs("blinding");
    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|s| s.title.to_lowercase().contains("blinding")));
}

#[test]
fn search_by_artist() {
    let results = search_songs("queen");
    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|s| s.artist.to_lowercase() == "queen"));
}

#[test]
fn search_by_album() {
    let results = search_songs("after hours");
    assert_eq!(results.len(), 2);
}

#[test]
fn search_no_matches_returns_empty() {
    let results = search_songs("nonexistent_song_xyz");
    assert!(results.is_empty());
}

#[test]
fn search_partial_match() {
    let results = search_songs("shape");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "Shape of You");
}
