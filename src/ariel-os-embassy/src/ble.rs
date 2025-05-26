use embassy_sync::once_lock::OnceLock;

use crate::cell::SameExecutorCell;

use apache_nimble::controller::NimbleController;
use trouble_host::Host;
use trouble_host::Stack;
pub static STACK: OnceLock<SameExecutorCell<Stack<'static, NimbleController>>> = OnceLock::new();

pub async fn ble_stack() -> Host<'static, NimbleController> {
    STACK.get().await.get_async().await.unwrap().build()
}
