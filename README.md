# kmod-rs [![Build Status][travis-img]][travis] [![crates.io][crates-img]][crates] [![docs.rs][docs-img]][docs]

[travis-img]:   https://travis-ci.org/kpcyrd/kmod-rs.svg?branch=master
[travis]:       https://travis-ci.org/kpcyrd/kmod-rs
[crates-img]:   https://img.shields.io/crates/v/kmod.svg
[crates]:       https://crates.io/crates/kmod
[docs-img]:     https://docs.rs/kmod/badge.svg
[docs]:         https://docs.rs/kmod

Bindings to libkmod to manage linux kernel modules.

```
# Cargo.toml
[dependencies]
kmod = "0.2"
```

To get started, see the [docs] and the examples/ folder.
```rust
extern crate kmod;
extern crate env_logger;

fn main() {
    env_logger::init();

    let ctx = kmod::Context::new().expect("kmod ctx failed");

    for module in ctx.modules_loaded().unwrap() {
        let name = module.name();
        let refcount = module.refcount();
        let size = module.size();

        let holders: Vec<_> = module.holders()
                                .map(|x| x.name())
                                .collect();

        println!("{:<19} {:8}  {} {:?}", name, size, refcount, holders);
    }
}
```

## License

MIT/Apache-2.0
