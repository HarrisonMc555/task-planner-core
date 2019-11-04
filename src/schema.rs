table! {
    tasks (id) {
        id -> Int4,
        title -> Varchar,
        due_date -> Timestamp,
        completed -> Bool,
    }
}
