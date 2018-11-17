#![allow(proc_macro_derive_resolution_fallback)]

table! {
    client_games (id) {
        id -> Nullable<Int8>,
        client_id -> Int8,
        game_id -> Int8,
    }
}

table! {
    clients (id) {
        id -> Nullable<Int8>,
        name -> Varchar,
        rank -> Nullable<Int8>,
        online -> Bool,
        last_login -> Nullable<Timestamp>,
    }
}

table! {
    games (id) {
        id -> Nullable<Int8>,
        started_at -> Nullable<Timestamp>,
        ended_at -> Nullable<Timestamp>,
    }
}

joinable!(client_games -> clients (client_id));
joinable!(client_games -> games (game_id));

allow_tables_to_appear_in_same_query!(
    client_games,
    clients,
    games,
);
