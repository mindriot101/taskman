table! {
    use priority::PriorityMapping;
    use diesel::sql_types::*;

    tasks (id) {
        id -> Integer,
        description -> Text,
        priority -> Nullable<PriorityMapping>,
    }
}
