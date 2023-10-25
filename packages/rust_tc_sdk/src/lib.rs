use neon::prelude::*;
use state::InitCell;

use crate::services::sdk::{RustSDK, RustSDKOptions};

mod models;
mod services;
mod entities;

static mut CONFIG: InitCell<RustSDK> = InitCell::new();

unsafe fn stop_sdk(mut cx: FunctionContext) -> NeonResult<()> {
  let clean_up = cx.argument::<Option<bool>>(0)?;

  match CONFIG.take() {
    None => {}
    Some(mut sdk) => {
      if clean_up.unwrap_or(false) {
        sdk.clean_up().expect("Could not clean up SDK");
      }

      CONFIG.reset();
    }
  }

  Ok(())
}

unsafe fn start_sdk(mut cx: FunctionContext) -> NeonResult<()> {
  CONFIG.get_or_init(|| {
    let mut db_url = cx.argument::<Option<String>>(0)?;

    let options = RustSDKOptions {
      db_url: db_url.take(),
    };
    RustSDK::new(options)
  });

  Ok(())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
  cx.export_function("start_sdk", start_sdk)?;
  cx.export_function("stop_sdk", stop_sdk)?;
  Ok(())
}
