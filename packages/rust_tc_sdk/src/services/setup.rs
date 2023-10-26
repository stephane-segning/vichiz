use std::sync::RwLock;

use neon::prelude::*;
use neon_serde3::*;
use state::*;

use crate::models::connection_data::ConnectionData;
use crate::models::room_option::RoomOption;
use crate::models::rust_sdk_options::RustSDKOptions;
use crate::services::sdk::RustSDK;

static CONFIG: InitCell<RwLock<RustSDK>> = InitCell::new();

pub(crate) fn stop_sdk(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let clean_up = cx.argument::<JsBoolean>(0)?.value(&mut cx);

    if let Ok(sdk) = CONFIG.get().write() {
        if clean_up {
            sdk.clean_up().unwrap_or_else(|e| panic!("Failed to clean up SDK {}", e));
        }
    }

    Ok(cx.undefined())
}

pub(crate) fn start_sdk(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    match CONFIG.get().write() {
        Ok(_) => {
            panic!("SDK already started")
        }
        Err(..) => {
            let arg0 = cx.argument::<JsValue>(0)?;

            let options: RustSDKOptions = from_value(&mut cx, arg0)
                .or_else(|e| cx.throw_error(e.to_string()))
                .unwrap();

            let sdk = RustSDK::new(options);

            CONFIG.set(RwLock::new(sdk));
        }
    }

    Ok(cx.undefined())
}

pub(crate) fn create_room(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let arg0 = cx.argument::<JsValue>(0)?;

    let room_option: RoomOption = from_value(&mut cx, arg0)
        .or_else(|e| cx.throw_error(e.to_string()))
        .unwrap();

    if let Ok(sdk) = CONFIG.get().write() {
        // Use sdk to create_room
        sdk.create_room(room_option).unwrap_or_else(|e| {
            panic!("Failed to create room {}", e);
        });
    }

    Ok(cx.undefined())
}

pub(crate) fn launch_room(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let arg0 = cx.argument::<JsValue>(0)?;

    let data: ConnectionData = from_value(&mut cx, arg0)
        .or_else(|e| cx.throw_error(e.to_string()))
        .unwrap();

    if let Ok(sdk) = CONFIG.get().write() {
        sdk.start_room(data).expect("TODO: panic message");
    }

    Ok(cx.undefined())
}