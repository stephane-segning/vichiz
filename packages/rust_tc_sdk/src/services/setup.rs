use neon::prelude::*;
use neon_serde3::*;
use tokio::sync::Mutex;

use crate::models::connection_data::ConnectionData;
use crate::models::room_id::RoomId;
use crate::models::room_option::RoomOption;
use crate::models::rust_sdk_options::RustSDKOptions;
use crate::services::sdk::RustSDK;
use crate::services::state::{CONFIG, get_sdk, rt};

pub(crate) fn stop_sdk(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let clean_up = cx.argument::<JsBoolean>(0)?.value(&mut cx);
    let (def, prom) = cx.promise();
    let channel = cx.channel();

    rt().spawn(async move {
        let sdk = get_sdk().await;
        if clean_up {
            sdk.clean_up().await.unwrap();
        }
        def.settle_with(&channel, move |mut cx| {
            Ok(cx.undefined())
        })
    });

    Ok(prom)
}

pub(crate) fn start_sdk(mut cx: FunctionContext) -> JsResult<JsPromise> {
    log::info!("Starting SDK");
    let arg0 = cx.argument::<JsValue>(0)?;
    let channel = cx.channel();
    let (def, prom) = cx.promise();

    let options: RustSDKOptions = from_value(&mut cx, arg0)
        .or_else(|e| cx.throw_error(e.to_string()))
        .unwrap();

    rt().spawn(async move {
        CONFIG.get_or_init(|| {
            log::info!("Starting SDK");

            let sdk = RustSDK::new(options);

            log::info!("SDK started");
            Mutex::new(sdk)
        });

        def.settle_with(&channel, move |mut cx| {
            Ok(cx.undefined())
        })
    });

    Ok(prom)
}

pub(crate) fn create_room(mut cx: FunctionContext) -> JsResult<JsPromise> {
    log::info!("Creating room");
    let (def, prom) = cx.promise();
    let channel = cx.channel();
    let arg0 = cx.argument::<JsValue>(0)?;

    let room_option: RoomOption = from_value(&mut cx, arg0)
        .or_else(|e| cx.throw_error(e.to_string()))
        .unwrap();

    rt().spawn(async move {
        let sdk = get_sdk().await;
        let value = sdk.create_room(room_option).await.unwrap_or_else(|e| {
            log::error!("Failed to create room: {}", e);
            panic!("Failed to create room: {}", e);
        });

        log::info!("Room created");

        def.settle_with(&channel, move |mut cx| {
            let js_value = to_value(&mut cx, &value)
                .or_else(|e| cx.throw_error(e.to_string()))
                .unwrap();

            Ok(js_value)
        })
    });

    Ok(prom)
}

pub(crate) fn get_rooms(mut cx: FunctionContext) -> JsResult<JsPromise> {
    log::info!("Getting rooms");
    let (def, prom) = cx.promise();
    let channel = cx.channel();

    rt().spawn(async move {
        let sdk = get_sdk().await;
        let value = sdk.get_rooms().await.unwrap_or_else(|e| {
            log::error!("Failed to get rooms: {}", e);
            panic!("Failed to get rooms: {}", e);
        });

        log::info!("Rooms retrieved");

        def.settle_with(&channel, move |mut cx| {
            let js_value = to_value(&mut cx, &value)
                .or_else(|e| cx.throw_error(e.to_string()))
                .unwrap();

            Ok(js_value)
        })
    });

    Ok(prom)
}

pub(crate) fn get_room(mut cx: FunctionContext) -> JsResult<JsPromise> {
    log::info!("Getting room");
    let (def, prom) = cx.promise();
    let channel = cx.channel();
    let arg0 = cx.argument::<JsValue>(0)?;

    let room_id: RoomId = from_value(&mut cx, arg0)
        .or_else(|e| cx.throw_error(e.to_string()))
        .unwrap();

    rt().spawn(async move {
        let sdk = get_sdk().await;
        let value = sdk.get_room(room_id.id.as_str()).await.unwrap_or_else(|e| {
            log::error!("Failed to get room: {}", e);
            panic!("Failed to get room: {}", e);
        });

        log::info!("Room created");


        def.settle_with(&channel, move |mut cx| {
            let js_value = to_value(&mut cx, &value)
                .or_else(|e| cx.throw_error(e.to_string()))
                .unwrap();
            Ok(js_value)
        })
    });

    Ok(prom)
}

pub(crate) fn remove_room(mut cx: FunctionContext) -> JsResult<JsPromise> {
    log::info!("Remove room");
    let (def, prom) = cx.promise();
    let channel = cx.channel();
    let arg0 = cx.argument::<JsValue>(0)?;

    let room_id: RoomId = from_value(&mut cx, arg0)
        .or_else(|e| cx.throw_error(e.to_string()))
        .unwrap();

    rt().spawn(async move {
        let sdk = get_sdk().await;
        sdk.remove_room(room_id.id.as_str()).await.unwrap();

        log::info!("Room removed");

        def.settle_with(&channel, move |mut cx| {
            Ok(cx.undefined())
        })
    });

    Ok(prom)
}

pub(crate) fn launch_room(mut cx: FunctionContext) -> JsResult<JsPromise> {
    log::info!("Launching room");
    let (def, prom) = cx.promise();
    let channel = cx.channel();
    let arg0 = cx.argument::<JsValue>(0)?;

    let data: ConnectionData = from_value(&mut cx, arg0)
        .or_else(|e| cx.throw_error(e.to_string()))?;

    rt().spawn(async move {
        let mut sdk = get_sdk().await;
        if let Err(e) = sdk.start_room(data).await {
            log::error!("Failed to launch room: {}", e);
            panic!("Failed to launch room");
        }

        log::info!("Room launched");

        def.settle_with(&channel, move |mut cx| {
            Ok(cx.undefined())
        })
    });

    Ok(prom)
}

pub(crate) fn quit_room(mut cx: FunctionContext) -> JsResult<JsPromise> {
    log::info!("Quitting room");
    let (def, prom) = cx.promise();
    let channel = cx.channel();
    let arg0 = cx.argument::<JsValue>(0).unwrap();
    let room_id: RoomId = from_value(&mut cx, arg0)
        .or_else(|e| cx.throw_error(e.to_string()))
        .unwrap();

    rt().spawn(async move {
        let mut sdk = get_sdk().await;
        if let Err(e) = sdk.quit_room(room_id.id.as_str()).await {
            log::error!("Failed to quit room: {}", e);
            panic!("Failed to quit room: {}", e);
        }

        def.settle_with(&channel, move |mut cx| {
            Ok(cx.undefined())
        })
    });

    Ok(prom)
}

pub(crate) fn register_listener(mut cx: FunctionContext) -> JsResult<JsPromise> {
    log::info!("Registering listener");
    let (def, prom) = cx.promise();
    let channel = cx.channel();
    let cb = cx.argument::<JsFunction>(0).unwrap();
    let root = cb.root(&mut cx);

    rt().spawn(async move {
        let mut sdk = get_sdk().await;
        sdk.register_listener(root).await;

        log::info!("Listener registered");

        def.settle_with(&channel, move |mut cx| {
            Ok(cx.undefined())
        })
    });

    Ok(prom)
}
