use dirs::data_local_dir;

pub fn get_database_path() -> Option<String> {
    data_local_dir().map(|mut path| {
        path.push("db");
        path.push("database.db");
        path.to_str().unwrap().to_string()
    })
}
