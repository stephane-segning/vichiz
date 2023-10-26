// @generated automatically by Diesel CLI.

diesel::table! {
    noise_keys (id) {
        id -> Text,
        private -> Binary,
        public -> Binary,
    }
}

diesel::table! {
    rooms (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    noise_keys,
    rooms,
);
