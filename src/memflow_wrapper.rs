use std::sync::{Arc, Mutex, MutexGuard};

use memflow::*;
use memflow_win32::error::{Error, Result};
use memflow_win32::*;

pub type CachedConnectorInstance =
    CachedMemoryAccess<'static, ConnectorInstance, TimedCacheValidator>;

pub type CachedTranslate = CachedVirtualTranslate<DirectTranslate, TimedCacheValidator>;

pub type CachedWin32Kernel = memflow_win32::Kernel<CachedConnectorInstance, CachedTranslate>;

static mut MEMFLOW_INSTANCE: Option<Arc<Mutex<Memflow>>> = None;

pub unsafe fn lock_memflow<'a>() -> Result<MutexGuard<'a, Memflow>> {
    if MEMFLOW_INSTANCE.is_none() {
        MEMFLOW_INSTANCE = Some(Arc::new(Mutex::new(Memflow::try_init()?)));
    }

    if let Some(memflow) = MEMFLOW_INSTANCE.as_ref() {
        if let Ok(memflow) = memflow.lock() {
            Ok(memflow)
        } else {
            Err(Error::Other("unable to lock memflow"))
        }
    } else {
        Err(Error::Other("memflow is not properly initialized"))
    }
}

pub struct Memflow {
    pub kernel: CachedWin32Kernel,
}

impl Memflow {
    // TODO: add config file or gui to setup the connection
    pub fn try_init() -> Result<Self> {
        let inventory = unsafe { ConnectorInventory::try_new() }.unwrap();
        let connector = unsafe {
            /*inventory.create_connector(
                "daemon",
                &ConnectorArgs::parse("unix:/var/run/memflow.sock,id=win10").unwrap(),
            )*/

            inventory.create_connector("qemu_procfs", &ConnectorArgs::default())
        }?;

        let kernel = Kernel::builder(connector).build_default_caches().build()?;

        Ok(Self { kernel })
    }
}
