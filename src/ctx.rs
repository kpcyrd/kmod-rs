use kmod_sys;
use errno;

use std::ptr;
use std::mem;
use std::ffi::CString;

use modules::{Module, ModuleIterator};
use errors::{Result, ErrorKind};


/// The kmod context
///
/// ```
/// let ctx = kmod::Context::new();
/// ```
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
    /// Create a new kmod context.
    ///
    /// ```
    /// let ctx = kmod::Context::new();
    /// ```
    #[inline]
    pub fn new() -> Context {
        let ctx = unsafe { kmod_sys::kmod_new(ptr::null(), ptr::null()) };
        trace!("creating kmod: {:?}", ctx);
        Context {
            ctx,
        }
    }

    /// Get an iterator of all loaded modules.
    ///
    /// ```
    /// let ctx = kmod::Context::new();
    /// for module in ctx.modules_loaded() {
    ///     // ...
    /// }
    /// ```
    #[inline]
    pub fn modules_loaded(&self) -> ModuleIterator {
        let mut list = Box::into_raw(Box::new(unsafe { mem::uninitialized() })) as *mut kmod_sys::kmod_list;

        let ret = unsafe { kmod_sys::kmod_module_new_from_loaded(self.ctx, &mut list) };
        trace!("kmod_module_new_from_loaded: {:?}", ret);

        ModuleIterator::new(list)
    }

    /// Create a module struct by path.
    ///
    /// ```
    /// let ctx = kmod::Context::new();
    /// let module = ctx.module_new_from_path("foo.ko");
    /// ```
    #[inline]
    pub fn module_new_from_path(&self, filename: &str) -> Result<Module> {
        let mut module = Box::into_raw(Box::new(unsafe { mem::uninitialized() })) as *mut kmod_sys::kmod_module;

        let filename = CString::new(filename)?;
        let ret = unsafe { kmod_sys::kmod_module_new_from_path(self.ctx, filename.as_ptr(), &mut module) };

        if ret < 0 {
            Err(ErrorKind::Errno(errno::errno()).into())
        } else {
            trace!("kmod_module_new_from_path: {:?}", module);
            Ok(Module::new(module))
        }
    }

    /// Create a module struct by name.
    ///
    /// ```
    /// let ctx = kmod::Context::new();
    /// let module = ctx.module_new_from_name("tun").unwrap();
    /// ```
    pub fn module_new_from_name(&self, name: &str) -> Result<Module> {
        let mut module = Box::into_raw(Box::new(unsafe { mem::uninitialized() })) as *mut kmod_sys::kmod_module;

        let name = CString::new(name)?;
        let ret = unsafe { kmod_sys::kmod_module_new_from_name(self.ctx, name.as_ptr(), &mut module) };

        if ret < 0 {
            Err(ErrorKind::Errno(errno::errno()).into())
        } else {
            trace!("kmod_module_new_from_name: {:?}", module);
            Ok(Module::new(module))
        }
    }
}
