use std::{ptr::NonNull, sync::Arc};

use rten_core::device::Device;

use super::data_type::{Element, Zero};

// pub struct Storage<T> {
//     data: DataPtr<T>,
// }

pub trait Storage {
    type Elem: Element;

    fn zeros(size: usize, device: Device) -> Self;
    fn ones(size: usize) -> Self;
    fn to_vec(&self) -> Vec<Self::Elem>;
}

pub struct RawPointerBuffer<T, A> {
    alloc: A,
    data: Arc<T>,
}

/// Lifetime is tied to ctx if not None.
pub struct DataPtr<T> {
    data: NonNull<T>,
    // ctx: Option<Context>,
}

impl<T> DataPtr<T> {
    pub fn data(&self) -> &T {
        unsafe { self.data.as_ref() }
    }

    pub fn data_mut(&mut self) -> &mut T {
        unsafe { self.data.as_mut() }
    }

    pub fn data_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    pub fn data_ptr_mut(&mut self) -> *mut T {
        self.data.as_ptr()
    }
}
