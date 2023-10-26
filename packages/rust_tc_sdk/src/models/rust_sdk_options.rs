use derive_more::From;
use getset::*;
use serde::{Deserialize, Serialize};

#[derive(From, Getters, MutGetters, Setters, Serialize, Debug, Deserialize)]
pub struct RustSDKOptions {
    #[getset(get_copy = "pub", set = "pub", get_mut = "pub", get = "pub")]
    pub db_url: Option<String>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_sdk_options_new() {
        let db_url = "sqlite://test.db";

        let options = RustSDKOptions::from(Some(db_url.clone().to_string()));

        assert_eq!(options.db_url.unwrap(), db_url);
    }
}
