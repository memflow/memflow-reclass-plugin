use memflow::*;
use memflow_win32::*;
use memflow_win32::error::Result;

pub struct Memflow {
    // internal state
}

impl Memflow {
    // TODO: add config file or gui to setup the connection
    pub fn try_init() -> Result<Self> {
        let inventory = unsafe { ConnectorInventory::try_new() }.unwrap();
        let connector = unsafe {
            inventory.create_connector(
                "daemon",
                &ConnectorArgs::parse("unix:/var/run/memflow.sock,id=win10").unwrap(),
            )
        }?;
    
        let kernel = Kernel::builder(connector)
            .build_default_caches()
            .build()?;

            Ok(Self{
            })
        }
}
