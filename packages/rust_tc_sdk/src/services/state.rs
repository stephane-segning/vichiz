use state::InitCell;
use tokio::runtime::Runtime;
use tokio::sync::{Mutex, MutexGuard};

use crate::services::sdk::RustSDK;

static TOKIO_RUNTIME: InitCell<Mutex<Runtime>> = InitCell::new();

pub(crate) static CONFIG: InitCell<Mutex<RustSDK>> = InitCell::new();


pub(crate) fn rt() -> MutexGuard<'static, Runtime> {
    TOKIO_RUNTIME
        .get_or_init(|| {
            let runtime = Runtime::new().unwrap_or_else(|_| { panic!("Cannot create runtime") });
            return Mutex::new(runtime);
        })
        // .clone()
        .blocking_lock()
}

pub(crate) async fn get_sdk() -> MutexGuard<'static, RustSDK> {
    CONFIG.get().lock().await
}