use std::ffi::{CStr, CString, OsStr};
use std::fmt;
use std::os::unix::ffi::OsStrExt;

use errno;
use kmod_sys::{self, kmod_list, kmod_module};
use log::trace;

use crate::errors::{ErrorKind, Result};

/// Wrapper around a kmod_module
pub struct Module {
    inner: *mut kmod_module,
}

impl Drop for Module {
    fn drop(&mut self) {
        trace!("dropping kmod_module: {:?}", self.inner);
        let _ = unsafe { kmod_sys::kmod_module_unref(self.inner) };
    }
}

impl Module {
    #[inline]
    pub(crate) fn new(module: *mut kmod_module) -> Module {
        trace!("creating kmod_module: {:?}", module);
        Module {
            inner: module,
        }
    }

    /// Get the name of the module
    #[inline]
    pub fn name(&self) -> &OsStr {
        unsafe {
            kmod_sys::kmod_module_get_name(self.inner)
                .as_ref()
                .map(|ptr| CStr::from_ptr(ptr))
        }
        .map(CStr::to_bytes)
        .map(OsStr::from_bytes)
        .unwrap() // Don't account for NULL as libkmod states that name is always available
    }

    /// Get the size of the module
    #[inline]
    pub fn size(&self) -> i64 {
        unsafe { kmod_sys::kmod_module_get_size(self.inner) }
    }

    /// Get the number of references to this module
    #[inline]
    pub fn refcount(&self) -> i32 {
        unsafe { kmod_sys::kmod_module_get_refcnt(self.inner) }
    }

    /// Iterate over the modules depending on this module
    #[inline]
    pub fn holders(&self) -> ModuleIterator {
        let holders = unsafe { kmod_sys::kmod_module_get_holders(self.inner) };
        ModuleIterator::new(holders)
    }

    /// Get this modules dependencies
    #[inline]
    pub fn dependencies(&self) -> ModuleIterator {
        let dependencies = unsafe { kmod_sys::kmod_module_get_dependencies(self.inner) };
        ModuleIterator::new(dependencies)
    }

    /// Get this modules soft dependencies
    #[inline]
    pub fn soft_dependencies(&self) -> Result<(ModuleIterator, ModuleIterator)> {
        let mut pre: *mut kmod_list = std::ptr::null_mut();
        let mut post: *mut kmod_list = std::ptr::null_mut();

        let ret = unsafe { kmod_sys::kmod_module_get_softdeps(self.inner, &mut pre, &mut post) };

        if ret < 0 {
            Err(ErrorKind::Errno(errno::errno()).into())
        } else {
            Ok((ModuleIterator::new(pre), ModuleIterator::new(post)))
        }
    }

    /// Get this modules dependencies
    #[inline]
    pub fn dependency_symbols(&self) -> Result<SymbolIterator> {
        let mut dependencies: *mut kmod_list = std::ptr::null_mut();

        let ret =
            unsafe { kmod_sys::kmod_module_get_dependency_symbols(self.inner, &mut dependencies) };
        if ret < 0 {
            Err(ErrorKind::Errno(errno::errno()).into())
        } else {
            Ok(SymbolIterator::new(dependencies))
        }
    }

    /// Get module path
    #[inline]
    pub fn path(&self) -> Option<&OsStr> {
        unsafe {
            kmod_sys::kmod_module_get_path(self.inner)
                .as_ref()
                .map(|ptr| CStr::from_ptr(ptr))
        }
        .map(CStr::to_bytes)
        .map(OsStr::from_bytes)
    }

    /// Get module options
    #[inline]
    pub fn options(&self) -> Option<&OsStr> {
        unsafe {
            kmod_sys::kmod_module_get_options(self.inner)
                .as_ref()
                .map(|ptr| CStr::from_ptr(ptr))
                .map(CStr::to_bytes)
                .map(OsStr::from_bytes)
        }
    }

    /// Insert the module into the kernel
    #[inline]
    pub fn insert_module(&self, flags: u32, opts: &[&str]) -> Result<()> {
        let opts = opts.join(" ");

        let opts = CString::new(opts)?;

        let ret = unsafe { kmod_sys::kmod_module_insert_module(self.inner, flags, opts.as_ptr()) };
        if ret < 0 {
            Err(ErrorKind::Errno(errno::errno()).into())
        } else {
            Ok(())
        }
    }

    /// Remove the module from the kernel
    #[inline]
    pub fn remove_module(&self, flags: u32) -> Result<()> {
        let ret = unsafe { kmod_sys::kmod_module_remove_module(self.inner, flags) };
        if ret < 0 {
            Err(ErrorKind::Errno(errno::errno()).into())
        } else {
            Ok(())
        }
    }
}

impl fmt::Debug for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("Module { .. }")
    }
}

/// Iterator over a kmod_list of modules
pub struct ModuleIterator {
    list: *mut kmod_list,
    iter: *mut kmod_list,
}

impl Drop for ModuleIterator {
    fn drop(&mut self) {
        trace!("dropping kmod_list: {:?}", self.list);
        let _ = unsafe { kmod_sys::kmod_module_unref_list(self.list) };
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

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        trace!("kmod_list->next: {:?}", self.iter);
        if !self.iter.is_null() {
            let module = unsafe { kmod_sys::kmod_module_get_module(self.iter) };
            self.iter = unsafe { kmod_sys::kmod_list_next(self.list, self.iter) };
            Some(Module::new(module))
        } else {
            None
        }
    }
}

impl fmt::Debug for ModuleIterator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("ModuleIterator { .. }")
    }
}

// Iterator over a kmod_list of Symbols
pub struct SymbolIterator {
    list: *mut kmod_list,
    iter: *mut kmod_list,
}

impl Drop for SymbolIterator {
    fn drop(&mut self) {
        trace!("dropping kmod_list: {:?}", self.list);
        unsafe { kmod_sys::kmod_module_symbols_free_list(self.list) };
    }
}

impl SymbolIterator {
    #[inline]
    pub(crate) fn new(list: *mut kmod_list) -> SymbolIterator {
        trace!("creating kmod_list: {:?}", list);
        SymbolIterator { list, iter: list }
    }
}

impl Iterator for SymbolIterator {
    type Item = String;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        trace!("kmod_list->next: {:?}", self.iter);

        if self.iter.is_null() {
            return None;
        }

        let symbol = unsafe { kmod_sys::kmod_module_dependency_symbol_get_symbol(self.iter) };
        let new_iter = unsafe { kmod_sys::kmod_list_next(self.list, self.iter) };
        self.iter = new_iter;

        if symbol.is_null() {
            panic!("Empty symbol");
        }

        let symbol = unsafe { CStr::from_ptr(symbol) };

        Some(symbol.to_string_lossy().into_owned())
    }
}

impl fmt::Debug for SymbolIterator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("SymbolIterator { .. }")
    }
}
