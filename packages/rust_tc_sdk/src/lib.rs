use std::sync::RwLock;

use neon::prelude::*;
use state::*;

use crate::models::rust_sdk_options::RustSDKOptions;
use crate::services::sdk::RustSDK;

mod models;
mod services;
mod entities;

static CONFIG: InitCell<RwLock<RustSDK>> = InitCell::new();

fn stop_sdk(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let clean_up = cx.argument::<JsBoolean>(0)?.value(&mut cx);

    if let Ok(sdk) = CONFIG.get().write() {
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

    CONFIG.set(RwLock::new(sdk));

    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("start_sdk", start_sdk)?;
    cx.export_function("stop_sdk", stop_sdk)?;
    Ok(())
}
