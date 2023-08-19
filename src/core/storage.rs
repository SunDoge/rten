use std::{ptr::NonNull, sync::Arc};

use super::data_type::Zero;

pub struct Storage<T> {
    data: DataPtr<T>,
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

#[derive(Debug)]
pub struct CpuStorage<T> {
    data: Box<[T]>,
}

impl<T> CpuStorage<T> {
    pub fn new(data: Box<[T]>) -> Self {
        Self { data }
    }
}

impl<T> CpuStorage<T>
where
    T: Zero + Clone,
{
    pub fn zeros(size: usize) -> Self {
        let data = vec![T::zero(); size].into_boxed_slice();
        Self { data }
    }
}
