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
