// @generated automatically by Diesel CLI.

diesel::table! {
    assets (id) {
        id -> Nullable<Integer>,
        symbol -> Text,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Text,
    }
}

diesel::table! {
    transactions (id) {
        id -> Nullable<Integer>,
        asset -> Text,
        quantity -> Float,
        price -> Float,
        date -> Text,
        #[sql_name = "type"]
        type_ -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    assets,
    transactions,
);
