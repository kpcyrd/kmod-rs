fn main() -> anyhow::Result<()> {
    env_logger::init();

    let ctx = kmod::Context::new()?;

    for module in ctx.modules_loaded()? {
        let name = module.name().to_string_lossy();
        let refcount = module.refcount();
        let size = module.size();

        let holders: Vec<_> = module
            .holders()
            .map(|x| x.name().to_string_lossy().into_owned())
            .collect();

        println!("{:<19} {:8}  {} {:?}", name, size, refcount, holders);
    }

    Ok(())
}
