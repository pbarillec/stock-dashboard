// @generated automatically by Diesel CLI.

diesel::table! {
    assets (id) {
        id -> Integer,
        symbol -> Text,
        name -> Text,
        category -> Text,
        api_id -> Nullable<Text>,
    }
}

diesel::table! {
    transactions (id) {
        id -> Integer,
        asset -> Text,
        quantity -> Double,
        price -> Double,
        date -> Text,
        category -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(assets, transactions,);
