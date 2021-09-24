use crate::gui::{
    alert,
    settings::{Config, Settings},
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use log::Level;

use memflow::prelude::v1::*;

static mut MEMFLOW_INSTANCE: Option<Arc<Mutex<Memflow>>> = None;

pub unsafe fn lock_memflow<'a>() -> Result<MutexGuard<'a, Memflow>> {
    if MEMFLOW_INSTANCE.is_none() {
        match Memflow::try_init() {
            Ok(memflow) => {
                MEMFLOW_INSTANCE = Some(Arc::new(Mutex::new(memflow)));
            }
            Err(err) => {
                alert::show_error(
                    "Unable to load memflow",
                    "Memflow failed to initialize some of its components",
                    err,
                );
                return Err(err.log_error("unable to initialize memflow"));
            }
        };
    }

    if let Some(memflow) = MEMFLOW_INSTANCE.as_ref() {
        if let Ok(memflow) = memflow.lock() {
            Ok(memflow)
        } else {
            Err(Error(ErrorOrigin::Other, ErrorKind::NotFound).log_error("unable to lock memflow"))
        }
    } else {
        Err(Error(ErrorOrigin::Other, ErrorKind::NotFound)
            .log_error("memflow is not properly initialized"))
    }
}

pub struct Memflow {
    pub config: Config,
    pub os: OsInstanceArcBox<'static>,
    pub handles: HashMap<u32, IntoProcessInstanceArcBox<'static>>,
}

impl Memflow {
    pub fn try_init() -> Result<Self> {
        // setup logging
        simple_logger::SimpleLogger::new()
            .with_level(Level::Debug.to_level_filter())
            .init()
            .ok();

        // load config file
        let mut settings = Settings::new();
        settings.configure();
        if let Err(err) = settings.persist() {
            alert::show_error(
                "Unable to save settings",
                "The configuration file could not be written",
                err,
            );
        }
        let config = settings.config();

        // load connector
        let inventory = Inventory::scan();
        let os = {
            match inventory
                .builder()
                .connector(&config.connector)
                .args(Args::parse(&config.args)?)
                .os("win32")
                .build()
            {
                Ok(os) => os,
                Err(err) => {
                    return Err(err);
                }
            }
        };

        Ok(Self {
            config,
            os,
            handles: HashMap::new(),
        })
    }

    pub fn open_process(&mut self, pid: u32) -> Result<u32> {
        let proc = self.os.clone().into_process_by_pid(pid)?;
        self.handles.insert(pid, proc);
        Ok(pid)
    }

    pub fn close_process(&mut self, handle: u32) {
        self.handles.remove(&handle);
    }

    pub fn get_process_mut(
        &mut self,
        handle: u32,
    ) -> Option<&mut IntoProcessInstanceArcBox<'static>> {
        self.handles.get_mut(&handle)
    }
}
