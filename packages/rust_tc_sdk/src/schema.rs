// @generated automatically by Diesel CLI.

diesel::table! {
    noise_keys (id) {
        id -> Nullable<Text>,
        private -> Text,
        public -> Text,
    }
}

diesel::table! {
    rooms (id) {
        id -> Nullable<Text>,
        name -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    noise_keys,
    rooms,
);
