
diesel::table! {
    records (id) {
        id -> Binary,
        value -> Text,
        publisher -> Nullable<Text>,
        expires -> Nullable<BigInt>,
    }
}

diesel::table! {
    provider_records (id) {
        id -> Binary,
        provider -> Binary,
        expires -> Nullable<BigInt>,
        addresses -> Array<Binary>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    records,
    provider_records,
);
