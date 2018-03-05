use kmod_sys::{self, kmod_list, kmod_module};

use std::ffi::CStr;


pub struct Module {
    inner: *mut kmod_module,
}

impl Drop for Module {
    fn drop(&mut self) {
        trace!("dropping kmod_module: {:?}", self.inner);
        unsafe { kmod_sys::kmod_module_unref(self.inner) };
    }
}

impl Module {
    #[inline]
    fn new(module: *mut kmod_module) -> Module {
        trace!("creating kmod_module: {:?}", module);
        Module {
            inner: module,
        }
    }

    #[inline]
    pub fn name(&self) -> String {
        let name = unsafe { kmod_sys::kmod_module_get_name(self.inner) };
        let name = unsafe { CStr::from_ptr(name) };
        name.to_string_lossy().into_owned()
    }

    #[inline]
    pub fn size(&self) -> i64 {
        unsafe { kmod_sys::kmod_module_get_size(self.inner) }
    }

    #[inline]
    pub fn refcount(&self) -> i32 {
        unsafe { kmod_sys::kmod_module_get_refcnt(self.inner) }
    }

    #[inline]
    pub fn holders(&self) -> ModuleIterator {
        let holders = unsafe { kmod_sys::kmod_module_get_holders(self.inner) };
        ModuleIterator::new(holders)
    }
}


pub struct ModuleIterator {
    list: *mut kmod_list,
    iter: *mut kmod_list,
}

impl Drop for ModuleIterator {
    fn drop(&mut self) {
        trace!("dropping kmod_list: {:?}", self.list);
        unsafe { kmod_sys::kmod_module_unref_list(self.list) };
    }
}

impl ModuleIterator {
    #[inline]
    pub(crate) fn new(list: *mut kmod_list) -> ModuleIterator {
        trace!("creating kmod_list: {:?}", list);
        ModuleIterator {
            list: list,
            iter: list,
        }
    }
}

impl Iterator for ModuleIterator {
    type Item = Module;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter = unsafe { kmod_sys::kmod_list_next(self.list, self.iter) };
        trace!("kmod_list->next: {:?}", self.iter);
        if !self.iter.is_null() {
            let module = unsafe { kmod_sys::kmod_module_get_module(self.iter) };
            Some(Module::new(module))
        } else {
            None
        }
    }
}
