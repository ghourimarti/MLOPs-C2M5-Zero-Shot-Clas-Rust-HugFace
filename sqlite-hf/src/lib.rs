/*
hugging face RUST library to songs and puts them into a sqlite database
 */

 use rust_bert::pipelines::sequence_classification::Label;
use rust_bert::pipelines::zero_shot_classification::ZeroShotClassificationModel;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use rusqlite::{params, Connection, Result};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// create zero shot classification candidate
// create zero shot classification candidates
fn create_db() -> sqlite::Connection{
    let db = sqlite::open("songs.db").unwrap();
    db.execute("CREATE TABLE zeroshotcandidates (id INTEGER PRIMARY KEY, label TEXT)")
        .unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('rock')")
        .unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('pop')")
        .unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('hip hop')")
        .unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('country')")
        .unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('latin')")
        .unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('r&b')")
        .unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('jazz')")
        .unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('electronic')")
        .unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('classical')")
        .unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('reggae')")
        .unwrap();
    db
}

// return all zero shot classification candidates as a vector of strings
pub fn get_all_zeroshotcandidates() -> Vec<String> {
    let db = create_db();
    let query = "SELECT label FROM zeroshotcandidates";
    let mut candidates: Vec<String> = Vec::new();
    db.iterate(query, |pairs| {
        for &(_column, value) in pairs.iter() {
            let value = value.unwrap();
            candidates.push(value.to_string());
        }
        true
    })
    .unwrap();
    candidates
}

// READ LYRICS FROM A FILE AND RETURN A VECTOR OF STRINGS
pub fn read_lyrics_from_file(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("Failed to open").unwrap();
    let reader = BufReader::new(file);
    let mut lyrics = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        lyrics.push(line);
    }

    lyrics
}

/*
use hugging face to classify lyrics using zero shot classification
Accept a vector of strings and grab candidtaes from the in memory sqlite database
*/
pub fn classify_lyrics(lyrics: Vec<String>) -> Vec<(String, Vec<Label>)> {
    let candidates = get_all_zeroshotcandidates();
    let candidate_labels: Vec<String> = candidates.iter().map(|c| c.to_string()).collect();
    // join lyrics into single string
    let lyrics : String = lyrics.join(" ");
    // convert to type std::convert::AsRef<&str>
    let lyrics : &str = lyrics.as_str();
    // create zero sshot classification model
    let model = ZeroShotClassificationModel::new(Default::default()).unwrap();
    let mut results = Vec::new();

    for lyric in lyrics {
        let result = model.predict(&lyric, &candidates, 1, 0.0);
        results.push((lyric, result));
    }

    results
}





// fn main() {
//     let db_path = "songs.db";
//     let conn = Connection::open(db_path).unwrap();

//     // Create the table if it doesn't exist
//     conn.execute(
//         "CREATE TABLE IF NOT EXISTS songs (
//             id INTEGER PRIMARY KEY,
//             title TEXT NOT NULL,
//             artist TEXT NOT NULL,
//             album TEXT NOT NULL,
//             genre TEXT NOT NULL
//         )",
//         [],
//     )
//     .unwrap();

//     // Read the JSON file
//     let file_path = "songs.json";
//     let file = File::open(file_path).unwrap();
//     let reader = std::io::BufReader::new(file);
//     let songs: Vec<Song> = serde_json::from_reader(reader).unwrap();

//     // Insert the songs into the database
//     for song in songs {
//         conn.execute(
//             "INSERT INTO songs (title, artist, album, genre) VALUES (?1, ?2, ?3, ?4)",
//             params![song.title, song.artist, song.album, song.genre],
//         )
//         .unwrap();
//     }
// }

// #[derive(Deserialize)]
// struct Song {
//     title: String,
//     artist: String,
//     album: String,
//     genre: String,
// }

