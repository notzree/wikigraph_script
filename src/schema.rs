// @generated automatically by Diesel CLI.

diesel::table! {
    lookup (title) {
        #[max_length = 255]
        title -> Varchar,
        byteoffset -> Int4,
        length -> Int4,
    }
}
