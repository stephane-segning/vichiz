use dirs::{data_dir};

pub fn get_database_path() -> Option<String> {
    log::info!("Getting database path");
    data_dir().map(|mut path| {
        path.push("db");
        path.push("database.db");
        path.to_str().unwrap().to_string()
    })
}
