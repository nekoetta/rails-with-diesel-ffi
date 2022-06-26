table! {
    somethings (id) {
        id -> Int4,
        user_id -> Int4,
        int -> Int4,
        sentence -> Text,
        date -> Nullable<Timestamptz>,
        nest -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(somethings -> users (user_id));

allow_tables_to_appear_in_same_query!(
    somethings,
    users,
);
