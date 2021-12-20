use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::db::schema::{
    connections, game_clues, games, image_clues, missing_vowels_clues, missing_vowels_rounds,
    music_clues, puzzle_wall_conns, puzzle_wall_words, sequences, text_clues, users,
};

#[derive(Queryable, Debug, Identifiable)]
pub struct Connection {
    pub id: i32,
    pub game_id: i32,
    pub answer: String,
    pub user_id: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(clue_id, game_id)]
pub struct GameClue {
    pub clue_id: i32,
    pub game_id: i32,
    pub symbol: i32,
}

#[derive(Queryable, Debug, Identifiable)]
pub struct Game {
    pub id: i32,
    pub game_name: String,
    pub user_id: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(clue_num, q_id)]
pub struct ImageClue {
    pub clue_num: i32,
    pub q_id: i32,
    pub image_file: Vec<u8>,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(round_id, clue_num)]
pub struct MissingVowelsClue {
    pub round_id: i32,
    pub clue_num: i32,
    pub text: String,
    pub answer: String,
}

#[derive(Queryable, Debug, Identifiable)]
pub struct MissingVowelsRound {
    pub id: i32,
    pub game_id: i32,
    pub round_num: i32,
    pub round_name: String,
    pub user_id: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(clue_num, q_id)]
pub struct MusicClue {
    pub clue_num: i32,
    pub q_id: i32,
    pub music_file: Vec<u8>,
}

#[derive(Queryable, Debug, Identifiable)]
pub struct PuzzleWallConn {
    pub id: i32,
    pub game_id: i32,
    pub wall_num: i32,
    pub answer: String,
    pub user_id: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(conn_id, wordnum)]
pub struct PuzzleWallWord {
    pub conn_id: i32,
    pub wordnum: i32,
    pub word: String,
}

#[derive(Queryable, Debug, Identifiable)]
pub struct Sequence {
    pub id: i32,
    pub game_id: i32,
    pub answer: String,
    pub user_id: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(clue_num, q_id)]
pub struct TextClue {
    pub clue_num: i32,
    pub q_id: i32,
    pub text: String,
}

#[derive(Queryable, Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub updated_at: DateTime<Utc>,
}
