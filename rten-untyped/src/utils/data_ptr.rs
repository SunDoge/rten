use crate::core::device::Device;

use super::unique_void_ptr::UniqueVoidPtr;

pub struct DataPtr {
    ptr: UniqueVoidPtr,
    device: Device,
}

impl DataPtr {
    pub fn new(ptr: UniqueVoidPtr, device: Device) -> Self {
        Self { ptr, device }
    }

    pub fn device(&self) -> Device {
        self.device
    }

    pub fn as_ref(&self) -> &[u8] {
        self.ptr.as_ref()
    }

    pub fn as_mut(&mut self) -> &mut [u8] {
        self.ptr.as_mut()
    }

    pub fn as_slice<T>(&self) -> &[T] {
        self.ptr.as_slice()
    }

    pub fn as_slice_mut<T>(&mut self) -> &mut [T] {
        self.ptr.as_slice_mut()
    }
}
