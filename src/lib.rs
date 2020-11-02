mod reclass;
use reclass::*;

mod memflow_wrapper;
use memflow_wrapper::*;

use std::ffi::c_void;
use std::ptr;

use memflow::*;
use memflow_win32::error::{Error, Result};
use memflow_win32::*;

#[no_mangle]
pub extern "C" fn EnumerateProcesses(callback: EnumerateProcessCallback) {
    if let Ok(mut memflow) = unsafe { lock_memflow() } {
        if let Ok(proc_list) = memflow.kernel.process_info_list() {
            for proc_info in proc_list.iter() {
                let mut proc = Win32Process::with_kernel_ref(&mut memflow.kernel, proc_info.to_owned());

                // TODO: replace by new main_module_info() function
                if let Ok(module_list) = proc.module_list() {
                    if let Ok(module) = module_list
                        .into_iter()
                        .find(|module| {
                            module.name()[..module.name().len().min(IMAGE_FILE_NAME_LENGTH - 1)]
                                .to_lowercase()
                                == proc_info.name.to_lowercase()
                        })
                        .ok_or_else(|| Error::ModuleInfo)
                    {
                        let mut proc_data = EnumerateProcessData::new(proc_info.pid as usize);
                        let name = module.name.encode_utf16().collect::<Vec<u16>>();
                        unsafe {
                            if name.len() >= MAX_PATH {
                                proc_data.name.copy_from_slice(&name[..MAX_PATH]);
                                proc_data.path.copy_from_slice(&name[..MAX_PATH]);
                            } else {
                                proc_data.name[..name.len()].copy_from_slice(&name[..]);
                                proc_data.path[..name.len()].copy_from_slice(&name[..]);
                            }

                            (callback)(&mut proc_data)
                        }
                    }
                }
            }
        }
    }
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
