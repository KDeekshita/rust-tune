use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Song {
    pub id: u32,
    pub title: String,
    pub artist: String,
    pub album: String,
}

pub fn get_all_songs() -> Vec<Song> {
    vec![
        Song { id: 1, title: "Blinding Lights".into(), artist: "The Weeknd".into(), album: "After Hours".into() },
        Song { id: 2, title: "Save Your Tears".into(), artist: "The Weeknd".into(), album: "After Hours".into() },
        Song { id: 3, title: "Shape of You".into(), artist: "Ed Sheeran".into(), album: "÷ (Divide)".into() },
        Song { id: 4, title: "Bad Habits".into(), artist: "Ed Sheeran".into(), album: "=".into() },
        Song { id: 5, title: "Bohemian Rhapsody".into(), artist: "Queen".into(), album: "A Night at the Opera".into() },
        Song { id: 6, title: "Don't Stop Me Now".into(), artist: "Queen".into(), album: "Jazz".into() },
        Song { id: 7, title: "Blinding Lights".into(), artist: "Alesso".into(), album: "Forever".into() },
    ]
}
