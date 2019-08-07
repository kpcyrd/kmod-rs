use env_logger;
use kmod;

fn main() {
    env_logger::init();

    let ctx = kmod::Context::new().expect("kmod ctx failed");

    for module in ctx.modules_loaded().unwrap() {
        let name = module.name();
        let refcount = module.refcount();
        let size = module.size();

        let holders: Vec<_> = module.holders()
                                .map(|x| x.name().to_owned())
                                .collect();

        println!("{:<19} {:8}  {} {:?}", name, size, refcount, holders);
    }
}
