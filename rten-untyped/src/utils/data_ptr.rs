use crate::core::device::Device;

use super::unique_void_ptr::UniqueVoidPtr;

pub struct DataPtr {
    ptr: UniqueVoidPtr,
    device: Device,
}
