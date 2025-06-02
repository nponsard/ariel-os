pub static STACK: OnceLock<trouble_host::Stack<'static, impl trouble_host::Controller>> =
    OnceLock::new();

pub async fn ble_stack() -> &'static trouble_host::Stack<'static, impl trouble_host::Controller> {
    STACK.get().await
}
