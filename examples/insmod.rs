use kmod;
use log::info;
use env_logger;

use std::env;

fn main() {
    env_logger::init();

    let ctx = kmod::Context::new().expect("kmod ctx failed");

    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1 {
        panic!("missing argument");
    }
    let filename = args.remove(0);

    let module = ctx.module_new_from_path(&filename).expect("new_from_path failed");
    info!("got module: {:?}", module.name());
    module.insert_module(0, &args.iter().map(|x| x.as_str()).collect::<Vec<_>>()).expect("insert_module failed");
}
