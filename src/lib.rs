mod reclass;
use reclass::*;

mod memflow_wrapper;
use memflow_wrapper::*;

use std::ffi::c_void;
use std::ptr;
use std::sync::{Arc, Mutex};

use gtk::prelude::*;
use gtk::{ButtonsType, DialogFlags, MessageDialog, MessageType, Window};

static mut MEMFLOW_INSTANCE: Option<Arc<Mutex<Memflow>>> = None;

#[no_mangle]
pub extern "C" fn EnumerateProcesses(callback: EnumerateProcessCallback) {
    // fancy test
    std::thread::spawn(move || {
        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
            return;
        }
        MessageDialog::new(
            None::<&Window>,
            DialogFlags::empty(),
            MessageType::Info,
            ButtonsType::Ok,
            "Hello World",
        )
        .run();
    });
}

#[no_mangle]
pub extern "C" fn EnumerateRemoteSectionsAndModules(
    handle: ProcessHandle,
    callback_section: EnumerateProcessCallback,
    callback_module: EnumerateRemoteModulesCallback,
) {
    //
}

#[no_mangle]
pub extern "C" fn OpenRemoteProcess(id: ProcessId, desired_access: i32) -> ProcessHandle {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn IsProcessValid(handle: ProcessHandle) -> bool {
    false
}

#[no_mangle]
pub extern "C" fn CloseRemoteProcess(handle: ProcessHandle) {
    //
}

#[no_mangle]
pub extern "C" fn ReadRemoteMemory(
    handle: ProcessHandle,
    address: *mut c_void,
    buffer: *mut c_void,
    offset: i32,
    size: i32,
) -> bool {
    false
}

#[no_mangle]
pub extern "C" fn WriteRemoteMemory(
    handle: ProcessHandle,
    address: *mut c_void,
    buffer: *mut c_void,
    offset: i32,
    size: i32,
) -> bool {
    false
}

#[no_mangle]
pub extern "C" fn ControlRemoteProcess(handle: ProcessHandle, action: i32) {
    //
}

#[no_mangle]
pub extern "C" fn AttachDebuggerToProcess(id: ProcessId) -> bool {
    false
}

#[no_mangle]
pub extern "C" fn DetachDebuggerFromProcess(id: ProcessId) {}

#[no_mangle]
pub extern "C" fn AwaitDebugEvent(event: *mut c_void /*DebugEvent*/, timeout: i32) -> bool {
    false
}

#[no_mangle]
pub extern "C" fn HandleDebugEvent(event: *mut c_void /*DebugEvent*/) {
    //
}

#[no_mangle]
pub extern "C" fn SetHardwareBreakpoint(
    id: ProcessId,
    address: *mut c_void,
    reg: i32,
    ty: i32,
    size: i32,
    set: bool,
) -> bool {
    false
}
