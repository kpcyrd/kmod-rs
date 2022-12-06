use kmod::errors::*;
use std::env;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let ctx = kmod::Context::new()?;

    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        anyhow::bail!("Missing argument");
    }
    let name = args.remove(0);
    let module = ctx.module_new_from_name(&name)?;

    info!("got module: {:?}", module.name());
    module.insert_module(0, &args.iter().map(|x| x.as_str()).collect::<Vec<_>>())?;

    Ok(())
}
