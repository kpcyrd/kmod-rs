extern crate kmod;
extern crate env_logger;

fn main() {
    env_logger::init();

    let ctx = kmod::Context::new().expect("kmod ctx failed");
    println!("{:?}", ctx.dirname());
}
