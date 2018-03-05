use kmod_sys;

use std::ptr;
use std::mem;

use modules::ModuleIterator;

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

}
