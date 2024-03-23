use crate::config::settings::{Config, Settings};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use log::LevelFilter;

use memflow::prelude::v1::*;

static mut MEMFLOW_INSTANCE: Option<Arc<Mutex<Memflow>>> = None;

pub unsafe fn lock_memflow<'a>() -> Result<MutexGuard<'a, Memflow>> {
    if MEMFLOW_INSTANCE.is_none() {
        match Memflow::try_init() {
            Ok(memflow) => {
                MEMFLOW_INSTANCE = Some(Arc::new(Mutex::new(memflow)));
            }
            Err(err) => {
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
        #[cfg(unix)]
        simple_logging::log_to(std::io::stdout(), LevelFilter::Info);
        #[cfg(not(unix))]
        simple_logging::log_to_file("memflow_reclass.log", LevelFilter::Info).ok();

        // load config file and set initial logging level
        let settings = Settings::new();
        log_level_from_str(settings.config().log_level.as_ref());

        let config = settings.config();

        // update logging level after showing the configuration dialog
        log_level_from_str(config.log_level.as_ref());

        // load connector
        let inventory = Inventory::scan();
        let os = {
            match inventory
                .builder()
                .connector(&config.connector)
                .args(config.args.parse()?)
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

    pub fn get_kernel_mut(&mut self) -> &mut OsInstanceArcBox<'static> {
        &mut self.os
    }

    pub fn get_process_mut(
        &mut self,
        handle: u32,
    ) -> Option<&mut IntoProcessInstanceArcBox<'static>> {
        self.handles.get_mut(&handle)
    }
}

fn log_level_from_str(log_level: &str) {
    match log_level.to_lowercase().as_ref() {
        "error" => log::set_max_level(LevelFilter::Error),
        "warn" => log::set_max_level(LevelFilter::Warn),
        "info" => log::set_max_level(LevelFilter::Info),
        "debug" => log::set_max_level(LevelFilter::Debug),
        "trace" => log::set_max_level(LevelFilter::Trace),
        _ => log::set_max_level(LevelFilter::Off),
    }
}
