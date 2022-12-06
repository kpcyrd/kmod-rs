use crate::errors::*;
use crate::modules::{Module, ModuleIterator};
use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::{fmt, ptr};

/// The kmod context
///
/// ```
/// let ctx = kmod::Context::new().unwrap();
/// ```
pub struct Context {
    ctx: *mut kmod_sys::kmod_ctx,
}

impl Drop for Context {
    fn drop(&mut self) {
        trace!("dropping kmod: {:?}", self.ctx);
        let _ = unsafe { kmod_sys::kmod_unref(self.ctx) };
    }
}

impl Context {
    /// Create a new kmod context.
    ///
    /// ```
    /// let ctx = kmod::Context::new().unwrap();
    /// ```
    #[inline]
    pub fn new() -> Result<Context> {
        let ctx = unsafe { kmod_sys::kmod_new(ptr::null(), ptr::null()) };
        if ctx.is_null() {
            Err(Error::NewCtx)
        } else {
            trace!("creating kmod: {:?}", ctx);
            Ok(Context { ctx })
        }
    }

    /// Create a new kmod context with given directory to search for kernel modules.
    ///
    /// ```
    /// use std::path::Path;
    /// let ctx = kmod::Context::new_with_dirname(&Path::new("/lib/modules/6.0.9")).unwrap();
    /// ```
    pub fn new_with_dirname(dirname: &Path) -> Result<Context> {
        let dirname = CString::new(dirname.as_os_str().as_bytes())?;

        let ctx = unsafe { kmod_sys::kmod_new(dirname.as_ptr(), ptr::null()) };

        if ctx.is_null() {
            Err(Error::NewCtx)
        } else {
            trace!("creating kmod: {:?}", ctx);
            Ok(Context { ctx })
        }
    }

    /// Get an iterator of all loaded modules.
    ///
    /// ```
    /// let ctx = kmod::Context::new().unwrap();
    /// for module in ctx.modules_loaded().unwrap() {
    ///     // ...
    /// }
    /// ```
    #[inline]
    pub fn modules_loaded(&self) -> Result<ModuleIterator> {
        let mut list = ptr::null::<kmod_sys::kmod_list>() as *mut kmod_sys::kmod_list;
        let ret = unsafe { kmod_sys::kmod_module_new_from_loaded(self.ctx, &mut list) };

        if ret < 0 {
            Err(Error::LoadedModules)
        } else {
            trace!("kmod_module_new_from_loaded: {:?}", list);
            Ok(ModuleIterator::new(list))
        }
    }

    /// Create a module struct by looking up a name or alias.
    ///
    /// ```
    /// # fn main() { foo(); }
    /// # fn foo() -> Result<(), Box<std::error::Error>> {
    /// use std::ffi::{OsStr, OsString};
    /// let ctx = kmod::Context::new()?;
    /// let module = ctx.module_new_from_lookup(&OsString::from("vfat"))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn module_new_from_lookup<S: AsRef<OsStr>>(&self, alias: S) -> Result<ModuleIterator> {
        let mut list = ptr::null::<kmod_sys::kmod_list>() as *mut kmod_sys::kmod_list;
        let alias = CString::new(alias.as_ref().as_bytes())?;
        let ret =
            unsafe { kmod_sys::kmod_module_new_from_lookup(self.ctx, alias.as_ptr(), &mut list) };

        if ret < 0 {
            Err(Error::ModuleFromLookup)
        } else {
            trace!("kmod_module_new_from_lookup: {:?}", list);
            Ok(ModuleIterator::new(list))
        }
    }

    /// Create a module struct by path.
    ///
    /// ```
    /// let ctx = kmod::Context::new().unwrap();
    /// let module = ctx.module_new_from_path("foo.ko");
    /// ```
    pub fn module_new_from_path<S: AsRef<OsStr>>(&self, filename: S) -> Result<Module> {
        let mut module = ptr::null::<kmod_sys::kmod_module>() as *mut kmod_sys::kmod_module;

        let filename = CString::new(filename.as_ref().as_bytes())?;
        let ret = unsafe {
            kmod_sys::kmod_module_new_from_path(self.ctx, filename.as_ptr(), &mut module)
        };

        if ret < 0 {
            Err(Error::ModuleFromPath(errno::errno()))
        } else {
            trace!("kmod_module_new_from_path: {:?}", module);
            Ok(Module::new(module))
        }
    }

    /// Create a module struct by name.
    ///
    /// ```
    /// let ctx = kmod::Context::new().unwrap();
    /// let module = ctx.module_new_from_name("tun").unwrap();
    /// ```
    pub fn module_new_from_name(&self, name: &str) -> Result<Module> {
        let mut module = ptr::null::<kmod_sys::kmod_module>() as *mut kmod_sys::kmod_module;

        let name = CString::new(name)?;
        let ret =
            unsafe { kmod_sys::kmod_module_new_from_name(self.ctx, name.as_ptr(), &mut module) };

        if ret < 0 {
            Err(Error::ModuleFromName)
        } else {
            trace!("kmod_module_new_from_name: {:?}", module);
            Ok(Module::new(module))
        }
    }

    /// Get the directory where kernel modules are stored
    ///
    /// ```
    /// let ctx = kmod::Context::new().unwrap();
    /// let dirname = ctx.dirname();
    /// ```
    pub fn dirname(&self) -> OsString {
        use std::os::unix::ffi::OsStringExt;

        let dirname = unsafe { kmod_sys::kmod_get_dirname(self.ctx) };
        let dirname = unsafe { CStr::from_ptr(dirname) };
        OsString::from_vec(dirname.to_bytes().to_vec())
    }
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("Context { .. }")
    }
}
