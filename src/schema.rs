// @generated automatically by Diesel CLI.

diesel::table! {
    lookup (title) {
        #[max_length = 255]
        title -> Varchar,
        byteoffset -> Int4,
        length -> Int4,
    }
}

diesel::table! {
    redirect (redirect_from) {
        #[max_length = 255]
        redirect_from -> Varchar,
        #[max_length = 255]
        redirect_to -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    lookup,
    redirect,
);
