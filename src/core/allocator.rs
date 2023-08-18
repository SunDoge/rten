use std::ptr::NonNull;

pub struct Context {
    ptr: NonNull<std::ffi::c_void>,
    deleter: fn(NonNull<std::ffi::c_void>),
}

impl Drop for Context {
    fn drop(&mut self) {
        (self.deleter)(self.ptr);
    }
}

pub enum UniquePtr<T> {
    Owned(NonNull<T>),
    Managed(Context),
}
