pub use log::{debug, error, info, trace, warn};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not setup kmod context")]
    NewCtx,
    #[error("Could not insert kernel module: {0}")]
    InsertModule(errno::Errno),
    #[error("Could not insert kernel module")]
    InsertModuleUnknown,
    #[error("Could not remove kernel module: {0}")]
    RemoveModule(errno::Errno),
    #[error("Could not find kernel module by name")]
    ModuleFromName,
    #[error("Could not find kernel module by lookup")]
    ModuleFromLookup,
    #[error("Could not load kernel module from path: {0}")]
    ModuleFromPath(errno::Errno),
    #[error("Could not access list of loaded modules")]
    LoadedModules,
    #[error("Input contains null bytes and can't be passed to the kernel")]
    Null(#[from] std::ffi::NulError),
}

pub type Result<T> = std::result::Result<T, Error>;
