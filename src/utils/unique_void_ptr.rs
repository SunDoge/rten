use std::ptr::NonNull;

#[derive(Debug)]
pub struct Context {
    ptr: NonNull<std::ffi::c_void>,
    deleter: fn(NonNull<std::ffi::c_void>),
}

impl Drop for Context {
    fn drop(&mut self) {
        (self.deleter)(self.ptr);
    }
}

impl Context {
    pub fn new(ptr: NonNull<std::ffi::c_void>, deleter: fn(NonNull<std::ffi::c_void>)) -> Self {
        Self { ptr, deleter }
    }
}

#[derive(Debug)]
pub enum UniqueVoidPtr {
    Owned(Box<u8>),
    Managed(Context),
}

// impl UniqueVoidPtr {
//     pub fn as_non_null(&self) -> NonNull<()> {
//         match self {
//             UniqueVoidPtr::Owned(ptr) => *ptr,
//             UniqueVoidPtr::Managed(ctx) => ctx.ptr.cast(),
//         }
//     }

//     pub fn as_ptr(&self) -> *mut () {
//         self.as_non_null().as_ptr()
//     }
// }
