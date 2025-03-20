use std::os::raw::c_void;
use std::thread;
use std::time::Duration;
/// This struct corresponds to the C definition of autoRustModuleConfig,
/// which originally contains a pointer to BSKLogger. Even if the logger is unused,
/// we include the field to maintain the expected layout.
#[repr(C)]
pub struct autoRustModuleConfig {
    pub bskLogger: *mut c_void,
}

#[unsafe(no_mangle)]
pub extern "C" fn SelfInit_autoRustModule(configData: *mut autoRustModuleConfig, moduleID: i64) {
    // Initialization logic here.
    // For example, you might set configData->bskLogger to NULL if it's unused.
    unsafe {
        if !configData.is_null() {
            (*configData).bskLogger = std::ptr::null_mut();
        }
    }
    println!(
        "Rust SelfInit_autoRustModule called with moduleID: {}",
        moduleID
    );
}

#[unsafe(no_mangle)]
pub extern "C" fn Reset_autoRustModule(
    configData: *mut autoRustModuleConfig,
    callTime: u64,
    moduleID: i64,
) {
    // Reset logic here.
    // For example, check if the required input messages are connected.
    println!(
        "Rust Reset_autoRustModule called with callTime: {} and moduleID: {}",
        callTime, moduleID
    );
}

#[unsafe(no_mangle)]
pub extern "C" fn Update_autoRustModule(
    configData: *mut autoRustModuleConfig,
    callTime: u64,
    moduleID: i64,
) {
    // Update logic here.
    // For example, zero the output buffers, read inputs, do calculations, then write outputs.
    thread::sleep(Duration::from_secs(1));
    println!(
        "Rust Update_autoRustModule called with callTime: {} and moduleID: {}",
        callTime, moduleID
    );
}
