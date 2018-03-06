//! Bindings to libkmod to manage linux kernel modules.
//!
//! # Example
//! ```
//! extern crate kmod;
//!
//! fn main() {
//!     // create a new kmod context
//!     let ctx = kmod::Context::new();
//!
//!     // get a kmod_list of all loaded modules
//!     for module in ctx.modules_loaded() {
//!         let name = module.name();
//!         let refcount = module.refcount();
//!         let size = module.size();
//!
//!         let holders: Vec<_> = module.holders()
//!             .map(|x| x.name())
//!             .collect();
//!
//!         println!("{:<19} {:8}  {} {:?}", name, size, refcount, holders);
//!     }
//! }
//! ```
#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;
extern crate errno;
extern crate reduce;
extern crate kmod_sys;

mod errors {
    use std;
    use errno::Errno;

    error_chain! {
        errors {
            Errno(err: Errno) {
                description("got error")
                display("{}", err)
            }
        }
        foreign_links {
            NulError(std::ffi::NulError);
        }
    }
}
pub use errors::{Result, Error, ErrorKind};

mod ctx;
mod modules;

pub use ctx::*;
pub use modules::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lsmod() {
        let ctx = Context::new();

        for module in ctx.modules_loaded() {
            let name = module.name();
            let refcount = module.refcount();
            let size = module.size();

            let holders: Vec<_> = module.holders()
                                    .map(|x| x.name())
                                    .collect();

            println!("{:<19} {:8}  {} {:?}", name, size, refcount, holders);
        }
    }
}
