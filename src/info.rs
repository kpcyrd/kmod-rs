use std::collections::HashMap;
use std::ffi::CStr;

use log::trace;

use crate::Module;

pub struct Info<'i> {
    _module: &'i Module,
    list: *mut kmod_sys::kmod_list,
    iter: *mut kmod_sys::kmod_list,
}

impl<'i> Info<'i> {
    pub fn new(
        _module: &'i Module,
        list: *mut kmod_sys::kmod_list,
        iter: *mut kmod_sys::kmod_list,
    ) -> Self {
        Info {
            _module,
            list,
            iter,
        }
    }
}

impl<'i> Drop for Info<'i> {
    fn drop(&mut self) {
        unsafe { kmod_sys::kmod_module_info_free_list(self.list) }
    }
}

impl<'i> Iterator for Info<'i> {
    type Item = (String, String);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        trace!("kmod_list->next: {:?}", self.iter);
        self.iter = unsafe { kmod_sys::kmod_list_next(self.list, self.iter) };
        if !self.iter.is_null() {
            let key = unsafe { kmod_sys::kmod_module_info_get_key(self.iter) };
            let value = unsafe { kmod_sys::kmod_module_info_get_value(self.iter) };
            let key = unsafe { CStr::from_ptr(key) }
                .to_string_lossy()
                .into_owned();
            let value = unsafe { CStr::from_ptr(value) }
                .to_string_lossy()
                .into_owned();
            Some((key, value))
        } else {
            None
        }
    }
}

impl<'i> Info<'i> {
    pub fn to_map(self) -> HashMap<String, Vec<String>> {
        let mut result = HashMap::new();
        for (k, v) in self {
            let e = result.entry(k).or_insert(Vec::new());
            e.push(v);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Context;
    #[test]
    fn info() {
        let ctx = Context::new().expect("kmod ctx failed");

        for module in ctx.modules_loaded().unwrap() {
            let name = module.name();
            let refcount = module.refcount();
            let size = module.size();

            let holders: Vec<_> = module.holders().map(|x| x.name().to_owned()).collect();
            println!("{:<19} {:8}  {} {:?}", name, size, refcount, holders);

            let info = module.info().unwrap();
            for i in info {
                println!("{}: {}", i.0, i.1);
            }
        }
    }

    #[test]
    fn info_map() {
        let ctx = Context::new().expect("kmod ctx failed");

        for module in ctx.modules_loaded().unwrap() {
            let name = module.name();
            let refcount = module.refcount();
            let size = module.size();

            let holders: Vec<_> = module.holders().map(|x| x.name().to_owned()).collect();
            println!("{:<19} {:8}  {} {:?}", name, size, refcount, holders);

            let info = module.info().unwrap().to_map();
            println!("{:?}", info);
        }
    }
}
