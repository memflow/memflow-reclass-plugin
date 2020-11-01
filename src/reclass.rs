use std::ffi::c_void;

pub const MAX_PATH: usize = 260;

pub type ProcessId = usize;
pub type ProcessHandle = *mut c_void;

#[repr(C, packed)]
pub struct EnumerateProcessData {
    id: ProcessId,
    name: [u16; MAX_PATH],
    path: [u16; MAX_PATH],
}
const _: [(); std::mem::size_of::<EnumerateProcessData>()] = [(); 0x418];

#[repr(C, packed)]
pub struct EnumerateRemoteSectionData {
    base_address: *mut c_void,
    size: usize,
    ty: i32,         // enum SectionType
    category: i32,   // enum SectionCategory
    protection: i32, // enum SectionProtection
    name: [u16; 16],
    module_path: [u16; MAX_PATH],
}
const _: [(); std::mem::size_of::<EnumerateRemoteSectionData>()] = [(); 0x244];

#[repr(C, packed)]
pub struct EnumerateRemoteModuleData {
    base_address: *mut c_void,
    size: usize,
    path: [u16; MAX_PATH],
}
const _: [(); std::mem::size_of::<EnumerateRemoteModuleData>()] = [(); 0x218];

pub type EnumerateProcessCallback = extern "C" fn(*mut EnumerateProcessData);
pub type EnumerateRemoteSectionsCallback = extern "C" fn(*mut EnumerateRemoteSectionData);
pub type EnumerateRemoteModulesCallback = extern "C" fn(*mut EnumerateRemoteModuleData);
