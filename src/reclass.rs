use std::ffi::c_void;

pub const MAX_PATH: usize = 260;

pub type ProcessId = usize;
pub type ProcessHandle = *mut c_void;

#[repr(C, packed)]
pub struct EnumerateProcessData {
    pub pid: ProcessId,
    pub name: [u16; MAX_PATH],
    pub path: [u16; MAX_PATH],
}
const _: [(); std::mem::size_of::<EnumerateProcessData>()] = [(); 0x418];

impl EnumerateProcessData {
    pub fn new(pid: ProcessId, name: &str, path: &str) -> Self {
        let name16 = name.encode_utf16().collect::<Vec<u16>>();
        let mut namebuf = [0u16; MAX_PATH];
        namebuf[..name16.len().min(MAX_PATH)]
            .copy_from_slice(&name16[..name16.len().min(MAX_PATH)]);

        let path16 = path.encode_utf16().collect::<Vec<u16>>();
        let mut pathbuf = [0u16; MAX_PATH];
        pathbuf[..path16.len().min(MAX_PATH)]
            .copy_from_slice(&path16[..path16.len().min(MAX_PATH)]);

        Self {
            pid,
            name: namebuf,
            path: pathbuf,
        }
    }
}

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

impl EnumerateRemoteSectionData {
    pub fn new(base_address: *mut c_void, size: usize) -> Self {
        Self {
            base_address,
            size,
            ty: 0,             // Unknown
            category: 0,       // Unknown
            protection: 1 | 2, // Read Write
            name: [0u16; 16],
            module_path: [0u16; MAX_PATH],
        }
    }
}

#[repr(C, packed)]
pub struct EnumerateRemoteModuleData {
    base_address: *mut c_void,
    size: usize,
    path: [u16; MAX_PATH],
}
const _: [(); std::mem::size_of::<EnumerateRemoteModuleData>()] = [(); 0x218];

impl EnumerateRemoteModuleData {
    pub fn new(base_address: *mut c_void, size: usize, path: &str) -> Self {
        let path16 = path.encode_utf16().collect::<Vec<u16>>();
        let mut pathbuf = [0u16; MAX_PATH];
        pathbuf[..path16.len().min(MAX_PATH)]
            .copy_from_slice(&path16[..path16.len().min(MAX_PATH)]);

        Self {
            base_address,
            size,
            path: pathbuf,
        }
    }
}

pub type EnumerateProcessCallback = extern "C" fn(*mut EnumerateProcessData);
pub type EnumerateRemoteSectionsCallback = extern "C" fn(*mut EnumerateRemoteSectionData);
pub type EnumerateRemoteModulesCallback = extern "C" fn(*mut EnumerateRemoteModuleData);
