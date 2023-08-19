use crate::core::device::Device;

use super::unique_void_ptr::UniqueVoidPtr;

#[derive(Debug)]
pub struct DataPtr {
    ptr: UniqueVoidPtr,
    device: Device,
}

impl DataPtr {
    fn new(ptr: UniqueVoidPtr, device: Device) -> Self {
        Self { ptr, device }
    }

    fn device(&self) -> &Device {
        &self.device
    }
}
