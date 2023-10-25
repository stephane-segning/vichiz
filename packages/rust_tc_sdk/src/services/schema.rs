use diesel::table;

table! {
    noise_keys (id) {
        id -> Text,
        private -> Binary,
        public -> Binary,
    }
}

table! {
    rooms (id) {
        id -> Text,
        name -> Text,
    }
}
