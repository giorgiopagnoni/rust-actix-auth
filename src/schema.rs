table! {
    auth_users (id) {
        id -> Unsigned<Integer>,
        email -> Varchar,
        passwd -> Varchar,
        token -> Nullable<Varchar>,
    }
}
