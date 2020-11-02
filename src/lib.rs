mod reclass;
use reclass::*;

mod memflow_wrapper;
use memflow_wrapper::*;

use std::ffi::c_void;
use std::ptr;
use std::slice;

use memflow::*;
use memflow_win32::error::{Error, Result};
use memflow_win32::*;

#[no_mangle]
pub extern "C" fn EnumerateProcesses(callback: EnumerateProcessCallback) {
    if let Ok(mut memflow) = unsafe { lock_memflow() } {
        if let Ok(proc_list) = memflow.kernel.process_info_list() {
            for proc_info in proc_list.iter() {
                let mut proc =
                    Win32Process::with_kernel_ref(&mut memflow.kernel, proc_info.to_owned());
                if let Ok(main_module) = proc.main_module_info() {
                    let mut proc_data = EnumerateProcessData::new(proc_info.pid as usize);
                    let path = main_module.path.encode_utf16().collect::<Vec<u16>>();
                    let name = main_module.name.encode_utf16().collect::<Vec<u16>>();
                    unsafe {
                        proc_data.name[..name.len().min(MAX_PATH)]
                            .copy_from_slice(&name[..name.len().min(MAX_PATH)]);
                        proc_data.path[..path.len().min(MAX_PATH)]
                            .copy_from_slice(&path[..path.len().min(MAX_PATH)]);
                        (callback)(&mut proc_data)
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
    // TODO:
}

#[no_mangle]
pub extern "C" fn OpenRemoteProcess(id: ProcessId, _desired_access: i32) -> ProcessHandle {
    if let Ok(mut memflow) = unsafe { lock_memflow() } {
        match memflow.open_process(id as u32) {
            Ok(handle) => handle as ProcessHandle,
            Err(_) => ptr::null_mut(),
        }
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn IsProcessValid(handle: ProcessHandle) -> bool {
    if let Ok(mut memflow) = unsafe { lock_memflow() } {
        memflow.is_process_alive(handle as u32)
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn CloseRemoteProcess(handle: ProcessHandle) {
    if let Ok(mut memflow) = unsafe { lock_memflow() } {
        memflow.close_process(handle as u32);
    }
}

#[no_mangle]
pub extern "C" fn ReadRemoteMemory(
    handle: ProcessHandle,
    address: *mut c_void,
    buffer: *mut c_void,
    offset: i32,
    size: i32,
) -> bool {
    if let Ok(mut memflow) = unsafe { lock_memflow() } {
        if let Some(proc) = memflow.get_process_mut(handle as u32) {
            let slice = unsafe { slice::from_raw_parts_mut(buffer as *mut u8, size as usize) };
            proc.virt_mem
                .virt_read_raw_into((address as u64).wrapping_add(offset as u64).into(), slice)
                .is_ok()
        } else {
            false
        }
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn WriteRemoteMemory(
    handle: ProcessHandle,
    address: *mut c_void,
    buffer: *mut c_void,
    offset: i32,
    size: i32,
) -> bool {
    if let Ok(mut memflow) = unsafe { lock_memflow() } {
        if let Some(proc) = memflow.get_process_mut(handle as u32) {
            let slice = unsafe { slice::from_raw_parts_mut(buffer as *mut u8, size as usize) };
            proc.virt_mem
                .virt_write_raw((address as u64).wrapping_add(offset as u64).into(), slice)
                .is_ok()
        } else {
            false
        }
    } else {
        false
    }
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
