table! {
    chatters (id) {
        id -> Int4,
        username -> Varchar,
        chat_date -> Date,
    }
}

table! {
    commands (id) {
        id -> Int4,
        command -> Varchar,
        response -> Text,
        used -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(chatters, commands,);
