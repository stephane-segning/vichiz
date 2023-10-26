use neon::prelude::*;
use std::sync::{Mutex};
use lazy_static::lazy_static;

use crate::models::rust_sdk_options::RustSDKOptions;
use crate::services::sdk::{RustSDK};

mod models;
mod services;
mod entities;

lazy_static! {
    static ref CONFIG: Mutex<Option<RustSDK>> = Mutex::new(None);
}

fn stop_sdk(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let clean_up = cx.argument::<JsBoolean>(0)?.value(&mut cx);

  let mut config_guard = CONFIG.lock().or_else(|e| panic!("Failed to lock config {}", e));
  if let Some(sdk) = &mut *config_guard {
    if clean_up {
      sdk.clean_up().unwrap_or_else(|e| panic!("Failed to clean up SDK {}", e));
    }
  }

  Ok(cx.undefined())
}

fn start_sdk(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let db_url = cx.argument::<JsString>(0)?.value(&mut cx);

  let options = RustSDKOptions::new(db_url);

  let sdk = RustSDK::new(options);
  let mut config_guard = CONFIG.lock().or_else(|e| panic!("Failed to lock config {}", e));
  *config_guard = Some(sdk);

  Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
  cx.export_function("start_sdk", start_sdk)?;
  cx.export_function("stop_sdk", stop_sdk)?;
  Ok(())
}
