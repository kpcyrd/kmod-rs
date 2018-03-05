#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;
extern crate kmod_sys;

mod errors {
    error_chain! {
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
