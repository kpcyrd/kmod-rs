//! Bindings to libkmod to manage linux kernel modules.
//!
//! # Example
//! ```
//! fn main() -> Result<(), Box<std::error::Error>>{
//!     // create a new kmod context
//!     let ctx = kmod::Context::new()?;
//!
//!     // get a kmod_list of all loaded modules
//!     for module in ctx.modules_loaded()? {
//!         let name = module.name().to_string_lossy();
//!         let refcount = module.refcount();
//!         let size = module.size();
//!
//!         let holders: Vec<_> = module.holders()
//!             .map(|x| x.name().to_string_lossy().into_owned())
//!             .collect();
//!
//!         println!("{:<19} {:8}  {} {:?}", name, size, refcount, holders);
//!     }
//!     Ok(())
//! }
//! ```

pub mod ctx;
pub mod errors;
pub mod modules;

pub use ctx::*;
pub use errno::Errno;
pub use errors::*;
pub use modules::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lsmod() {
        let ctx = Context::new().unwrap();

        for module in ctx.modules_loaded().unwrap() {
            let name = module.name().to_string_lossy();
            let refcount = module.refcount();
            let size = module.size();

            let holders: Vec<_> = module
                .holders()
                .map(|x| x.name().to_string_lossy().into_owned())
                .collect();

            println!("{:<19} {:8}  {} {:?}", name, size, refcount, holders);
        }
    }

    #[test]
    fn bad_name() {
        let ctx = Context::new().unwrap();
        let m = ctx
            .module_new_from_name("/lib/modules/5.1.12-300.fc30.x86_64/kernel/fs/cifs/cifs.ko.xz")
            .unwrap();
        println!("name: {:?}", m.name());
        println!("path: {:?}", m.path());
    }
}
