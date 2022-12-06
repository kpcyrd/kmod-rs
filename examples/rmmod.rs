use anyhow::Context;
use kmod::errors::*;
use std::env;
use std::fs;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let ctx = kmod::Context::new().context("kmod ctx failed")?;
    let filename = env::args().nth(1).context("missing argument")?;

    let module = if fs::metadata(&filename).is_ok() {
        // it's a file
        ctx.module_new_from_path(&filename)?
    } else {
        // it's probably a name
        ctx.module_new_from_name(&filename)?
    };

    info!("got module: {:?}", module.name());
    module.remove_module(0)?;

    Ok(())
}
