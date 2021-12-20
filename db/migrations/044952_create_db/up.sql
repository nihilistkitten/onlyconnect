CREATE TABLE "users" (
  "id" SERIAL PRIMARY KEY,
  "username" varchar UNIQUE NOT NULL,
  "email" varchar UNIQUE NOT NULL,
  "updated_at" timestamptz NOT NULL
);

CREATE TABLE "games" (
  "id" SERIAL PRIMARY KEY,
  "game_name" varchar NOT NULL,
  "user_id" int NOT NULL,
  "updated_at" timestamptz NOT NULL
);

CREATE TABLE "game_clues" (
  "clue_id" int NOT NULL,
  "game_id" int NOT NULL,
  "symbol" int NOT NULL,
  PRIMARY KEY ("clue_id", "game_id")
);

CREATE TABLE "connections" (
  "id" SERIAL PRIMARY KEY,
  "game_id" int NOT NULL,
  "answer" varchar NOT NULL,
  "user_id" int NOT NULL,
  "updated_at" timestamptz NOT NULL
);

CREATE TABLE "sequences" (
  "id" SERIAL PRIMARY KEY,
  "game_id" int NOT NULL,
  "answer" varchar NOT NULL,
  "user_id" int NOT NULL,
  "updated_at" timestamptz NOT NULL
);

CREATE TABLE "text_clues" (
  "clue_num" int NOT NULL,
  "q_id" int NOT NULL,
  "text" varchar NOT NULL,
  PRIMARY KEY ("clue_num", "q_id")
);

CREATE TABLE "image_clues" (
  "clue_num" int NOT NULL,
  "q_id" int NOT NULL,
  "image_file" bytea NOT NULL,
  PRIMARY KEY ("clue_num", "q_id")
);

CREATE TABLE "music_clues" (
  "clue_num" int NOT NULL,
  "q_id" int NOT NULL,
  "music_file" bytea NOT NULL,
  PRIMARY KEY ("clue_num", "q_id")
);

CREATE TABLE "puzzle_wall_conns" (
  "id" SERIAL PRIMARY KEY,
  "game_id" int NOT NULL,
  "wall_num" int NOT NULL,
  "answer" varchar NOT NULL,
  "user_id" int NOT NULL,
  "updated_at" timestamptz NOT NULL
);

CREATE TABLE "puzzle_wall_words" (
  "conn_id" int NOT NULL,
  "wordnum" int NOT NULL,
  "word" varchar NOT NULL,
  PRIMARY KEY ("conn_id", "wordnum")
);

CREATE TABLE "missing_vowels_rounds" (
  "id" SERIAL PRIMARY KEY,
  "game_id" int NOT NULL,
  "round_num" int NOT NULL,
  "round_name" varchar NOT NULL,
  "user_id" int NOT NULL,
  "updated_at" timestamptz NOT NULL
);

CREATE TABLE "missing_vowels_clues" (
  "round_id" int NOT NULL,
  "clue_num" int NOT NULL,
  "text" varchar NOT NULL,
  "answer" varchar NOT NULL,
  PRIMARY KEY ("round_id", "clue_num")
);

ALTER TABLE "games" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "game_clues" ADD FOREIGN KEY ("game_id") REFERENCES "games" ("id");

ALTER TABLE "text_clues" ADD FOREIGN KEY ("q_id") REFERENCES "connections" ("id");

ALTER TABLE "image_clues" ADD FOREIGN KEY ("q_id") REFERENCES "connections" ("id");

ALTER TABLE "music_clues" ADD FOREIGN KEY ("q_id") REFERENCES "connections" ("id");

ALTER TABLE "text_clues" ADD FOREIGN KEY ("q_id") REFERENCES "sequences" ("id");

ALTER TABLE "image_clues" ADD FOREIGN KEY ("q_id") REFERENCES "sequences" ("id");

ALTER TABLE "music_clues" ADD FOREIGN KEY ("q_id") REFERENCES "sequences" ("id");

ALTER TABLE "game_clues" ADD FOREIGN KEY ("clue_id") REFERENCES "puzzle_wall_conns" ("id");

ALTER TABLE "missing_vowels_clues" ADD FOREIGN KEY ("round_id") REFERENCES "missing_vowels_rounds" ("id");

ALTER TABLE "puzzle_wall_words" ADD FOREIGN KEY ("conn_id") REFERENCES "puzzle_wall_conns" ("id");

ALTER TABLE "game_clues" ADD FOREIGN KEY ("clue_id") REFERENCES "missing_vowels_rounds" ("id");

ALTER TABLE "connections" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "sequences" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "missing_vowels_rounds" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "puzzle_wall_conns" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

