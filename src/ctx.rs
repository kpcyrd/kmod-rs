use kmod_sys;

use std::ptr;
use std::mem;
use std::ffi::CString;

use modules::{Module, ModuleIterator};
use errors::Result;


pub struct Context {
    ctx: *mut kmod_sys::kmod_ctx,
}

impl Drop for Context {
    fn drop(&mut self) {
        trace!("dropping kmod: {:?}", self.ctx);
        unsafe { kmod_sys::kmod_unref(self.ctx) };
    }
}

impl Context {
    #[inline]
    pub fn new() -> Context {
        let ctx = unsafe { kmod_sys::kmod_new(ptr::null(), ptr::null()) };
        trace!("creating kmod: {:?}", ctx);
        Context {
            ctx,
        }
    }

    #[inline]
    pub fn modules_loaded(&self) -> ModuleIterator {
        let mut list = Box::into_raw(Box::new(unsafe { mem::uninitialized() })) as *mut kmod_sys::kmod_list;

        let ret = unsafe { kmod_sys::kmod_module_new_from_loaded(self.ctx, &mut list) };
        trace!("kmod_module_new_from_loaded: {:?}", ret);

        ModuleIterator::new(list)
    }

    #[inline]
    pub fn module_new_from_path(&self, filename: &str) -> Result<Module> {
        let mut module = Box::into_raw(Box::new(unsafe { mem::uninitialized() })) as *mut kmod_sys::kmod_module;

        let filename = CString::new(filename)?;
        unsafe { kmod_sys::kmod_module_new_from_path(self.ctx, filename.as_ptr(), &mut module) };
        trace!("kmod_module_new_from_path: {:?}", module);
        Ok(Module::new(module))
    }

    pub fn module_new_from_name(&self, name: &str) -> Result<Module> {
        let mut module = Box::into_raw(Box::new(unsafe { mem::uninitialized() })) as *mut kmod_sys::kmod_module;

        let name = CString::new(name)?;
        unsafe { kmod_sys::kmod_module_new_from_name(self.ctx, name.as_ptr(), &mut module) };
        trace!("kmod_module_new_from_name: {:?}", module);
        Ok(Module::new(module))
    }
}
