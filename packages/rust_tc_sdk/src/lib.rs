use neon::prelude::*;

use crate::services::setup::*;

mod models;
mod services;
mod entities;
mod schema;


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    setup_logger("debug");

    cx.export_function("startSdk", start_sdk)?;
    cx.export_function("stopSdk", stop_sdk)?;
    cx.export_function("createRoom", create_room)?;
    cx.export_function("removeRoom", remove_room)?;
    cx.export_function("launchRoom", launch_room)?;
    cx.export_function("quitRoom", quit_room)?;
    cx.export_function("getRoom", get_room)?;
    cx.export_function("getRooms", get_rooms)?;
    cx.export_function("registerListener", register_listener)?;
    Ok(())
}
