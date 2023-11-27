// @generated automatically by Diesel CLI.

diesel::table! {
    games (id) {
        id -> Integer,
        winner -> Integer,
        #[max_length = 45]
        timestamp -> Nullable<Varchar>,
    }
}

diesel::table! {
    players (id) {
        id -> Integer,
        #[max_length = 45]
        name -> Varchar,
        registered -> Nullable<Datetime>,
    }
}

diesel::joinable!(games -> players (winner));

diesel::allow_tables_to_appear_in_same_query!(
    games,
    players,
);
