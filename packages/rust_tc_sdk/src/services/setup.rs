use std::sync::Mutex;
use env_logger::{Builder, Target};
use neon::prelude::*;
use neon_serde3::*;
use state::*;

use crate::models::connection_data::ConnectionData;
use crate::models::room_id::RoomId;
use crate::models::room_option::RoomOption;
use crate::models::rust_sdk_options::RustSDKOptions;
use crate::services::sdk::RustSDK;

static CONFIG: InitCell<Mutex<RustSDK>> = InitCell::new();

pub(crate) fn stop_sdk(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let clean_up = cx.argument::<JsBoolean>(0)?.value(&mut cx);

    let sdk = CONFIG.get().lock().expect("Failed to lock SDK");
    if clean_up {
        sdk.clean_up().unwrap_or_else(|e| panic!("Failed to clean up SDK: {}", e));
    }

    Ok(cx.undefined())
}

pub(crate) fn start_sdk(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let arg0 = cx.argument::<JsValue>(0)?;
    let channel = cx.channel();
    CONFIG.get_or_init(|| {
        log::info!("Starting SDK");

        let options: RustSDKOptions = from_value(&mut cx, arg0)
            .or_else(|e| cx.throw_error(e.to_string()))
            .unwrap();

        let sdk = RustSDK::new(channel, options);

        log::info!("SDK started");
        Mutex::new(sdk)
    });

    Ok(cx.undefined())
}

pub(crate) fn create_room(mut cx: FunctionContext) -> JsResult<JsValue> {
    log::info!("Creating room");
    let arg0 = cx.argument::<JsValue>(0)?;

    let room_option: RoomOption = from_value(&mut cx, arg0)
        .or_else(|e| cx.throw_error(e.to_string()))
        .unwrap();

    let sdk = CONFIG.get().lock().expect("Failed to lock SDK");
    let value = sdk.create_room(room_option).unwrap_or_else(|e| {
        panic!("Failed to create room: {}", e);
    });

    log::info!("Room created");

    let js_value = to_value(&mut cx, &value)
        .or_else(|e| cx.throw_error(e.to_string()))
        .unwrap();
    Ok(js_value)
}

pub(crate) fn get_rooms(mut cx: FunctionContext) -> JsResult<JsValue> {
    log::info!("Getting rooms");

    let sdk = CONFIG.get().lock().expect("Failed to lock SDK");
    let value = sdk.get_rooms().unwrap_or_else(|e| {
        panic!("Failed to get rooms: {}", e);
    });

    log::info!("Rooms retrieved");

    let js_value = to_value(&mut cx, &value)
        .or_else(|e| cx.throw_error(e.to_string()))
        .unwrap();
    Ok(js_value)
}

pub(crate) fn get_room(mut cx: FunctionContext) -> JsResult<JsValue> {
    log::info!("Getting room");
    let arg0 = cx.argument::<JsValue>(0)?;

    let room_id: RoomId = from_value(&mut cx, arg0)
        .or_else(|e| cx.throw_error(e.to_string()))
        .unwrap();

    let sdk = CONFIG.get().lock().expect("Failed to lock SDK");
    let value = sdk.get_room(room_id.id.as_str()).unwrap_or_else(|e| {
        panic!("Failed to get room: {}", e);
    });

    log::info!("Room created");

    let js_value = to_value(&mut cx, &value)
        .or_else(|e| cx.throw_error(e.to_string()))
        .unwrap();
    Ok(js_value)
}

pub(crate) fn remove_room(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    log::info!("Remove room");
    let arg0 = cx.argument::<JsValue>(0)?;

    let room_id: RoomId = from_value(&mut cx, arg0)
        .or_else(|e| cx.throw_error(e.to_string()))
        .unwrap();

    let sdk = CONFIG.get().lock().expect("Failed to lock SDK");
    sdk.remove_room(room_id.id.as_str()).unwrap_or_else(|e| {
        panic!("Failed to remove room: {}", e);
    });

    log::info!("Room removed");
    Ok(cx.undefined())
}

pub(crate) fn launch_room(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    log::info!("Launching room");
    let arg0 = cx.argument::<JsValue>(0)?;

    let data: ConnectionData = from_value(&mut cx, arg0)
        .or_else(|e| cx.throw_error(e.to_string()))?;

    let mut sdk = CONFIG.get().lock().expect("Failed to lock SDK");
    if let Err(e) = sdk.start_room(data) {
        panic!("Failed to launch room: {}", e);
    }

    log::info!("Room launched");
    Ok(cx.undefined())
}

pub(crate) fn quit_room(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    log::info!("Quitting room");
    let arg0 = cx.argument::<JsValue>(0)?;

    let room_id: RoomId = from_value(&mut cx, arg0)
        .or_else(|e| cx.throw_error(e.to_string()))
        .unwrap();

    let mut sdk = CONFIG.get().lock().expect("Failed to lock SDK");
    if let Err(e) = sdk.quit_room(room_id.id.as_str()) {
        panic!("Failed to quit room: {}", e);
    }

    log::info!("Room quit");
    Ok(cx.undefined())
}

pub(crate) fn register_listener(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    log::info!("Registering listener");

    let cb = cx.argument::<JsFunction>(0)?;
    let mut sdk = CONFIG.get().lock().expect("Failed to lock SDK");
    sdk.register_listener(cb.root(&mut cx));

    log::info!("Listener registered");
    Ok(cx.undefined())
}
