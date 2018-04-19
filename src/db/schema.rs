table! {
    tasks (id) {
        id -> Integer,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
