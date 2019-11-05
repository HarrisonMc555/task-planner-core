table! {
    tasks (id) {
        id -> Int4,
        title -> Varchar,
        due_date -> Nullable<Timestamp>,
        completed -> Bool,
    }
}
