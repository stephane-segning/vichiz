pub struct RustSDKOptions {
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
