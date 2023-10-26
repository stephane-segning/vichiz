use neon::prelude::*;

use crate::services::setup::*;

mod models;
mod services;
mod entities;
mod schema;


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("startSdk", start_sdk)?;
    cx.export_function("stopSdk", stop_sdk)?;
    cx.export_function("createRoom", create_room)?;
    cx.export_function("launchRoom", launch_room)?;
    Ok(())
}
