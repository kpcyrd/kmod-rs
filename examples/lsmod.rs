extern crate kmod;
extern crate env_logger;

fn main() {
    env_logger::init();

    let ctx = kmod::Context::new();

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
