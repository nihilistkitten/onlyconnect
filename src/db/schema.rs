table! {
    connections (id) {
        id -> Int4,
        game_id -> Int4,
        answer -> Varchar,
        user_id -> Int4,
        updated_at -> Timestamptz,
    }
}

table! {
    game_clues (clue_id, game_id) {
        clue_id -> Int4,
        game_id -> Int4,
        symbol -> Int4,
    }
}

table! {
    games (id) {
        id -> Int4,
        game_name -> Varchar,
        user_id -> Int4,
        updated_at -> Timestamptz,
    }
}

table! {
    image_clues (clue_num, q_id) {
        clue_num -> Int4,
        q_id -> Int4,
        image_file -> Bytea,
    }
}

table! {
    missing_vowels_clues (round_id, clue_num) {
        round_id -> Int4,
        clue_num -> Int4,
        text -> Varchar,
        answer -> Varchar,
    }
}

table! {
    missing_vowels_rounds (id) {
        id -> Int4,
        game_id -> Int4,
        round_num -> Int4,
        round_name -> Varchar,
        user_id -> Int4,
        updated_at -> Timestamptz,
    }
}

table! {
    music_clues (clue_num, q_id) {
        clue_num -> Int4,
        q_id -> Int4,
        music_file -> Bytea,
    }
}

table! {
    puzzle_wall_conns (id) {
        id -> Int4,
        game_id -> Int4,
        wall_num -> Int4,
        answer -> Varchar,
        user_id -> Int4,
        updated_at -> Timestamptz,
    }
}

table! {
    puzzle_wall_words (conn_id, wordnum) {
        conn_id -> Int4,
        wordnum -> Int4,
        word -> Varchar,
    }
}

table! {
    sequences (id) {
        id -> Int4,
        game_id -> Int4,
        answer -> Varchar,
        user_id -> Int4,
        updated_at -> Timestamptz,
    }
}

table! {
    text_clues (clue_num, q_id) {
        clue_num -> Int4,
        q_id -> Int4,
        text -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        updated_at -> Timestamptz,
    }
}

joinable!(connections -> users (user_id));
joinable!(game_clues -> games (game_id));
joinable!(game_clues -> missing_vowels_rounds (clue_id));
joinable!(game_clues -> puzzle_wall_conns (clue_id));
joinable!(games -> users (user_id));
joinable!(image_clues -> connections (q_id));
joinable!(image_clues -> sequences (q_id));
joinable!(missing_vowels_clues -> missing_vowels_rounds (round_id));
joinable!(missing_vowels_rounds -> users (user_id));
joinable!(music_clues -> connections (q_id));
joinable!(music_clues -> sequences (q_id));
joinable!(puzzle_wall_conns -> users (user_id));
joinable!(puzzle_wall_words -> puzzle_wall_conns (conn_id));
joinable!(sequences -> users (user_id));
joinable!(text_clues -> connections (q_id));
joinable!(text_clues -> sequences (q_id));

allow_tables_to_appear_in_same_query!(
    connections,
    game_clues,
    games,
    image_clues,
    missing_vowels_clues,
    missing_vowels_rounds,
    music_clues,
    puzzle_wall_conns,
    puzzle_wall_words,
    sequences,
    text_clues,
    users,
);
