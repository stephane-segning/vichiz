use getset::*;
use serde::{Deserialize, Serialize};

#[derive(Getters, MutGetters, Setters, Serialize, Debug, Deserialize)]
pub struct RustSDKOptions {
    #[getset(get_copy = "pub", set = "pub", get_mut = "pub", get = "pub")]
    pub db_url: String,
}

impl RustSDKOptions {
    pub fn new<S: Into<String>>(db_url: S) -> Self {
        Self {
            db_url: db_url.into(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_sdk_options_new() {
        let db_url = "sqlite://test.db";

        let options = RustSDKOptions::new(db_url);

        assert_eq!(options.db_url, db_url);
    }
}
