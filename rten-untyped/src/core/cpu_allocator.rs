use std::sync::{Arc, Mutex};

use once_cell::sync::{Lazy, OnceCell};

use crate::utils::{data_ptr::DataPtr, unique_void_ptr::UniqueVoidPtr};

use super::{allocator::Allocator, device::Device};

pub static CPU_ALLOCATOR: Lazy<Arc<Mutex<dyn Allocator>>> =
    Lazy::new(|| Arc::new(Mutex::new(CpuAllocator {})));

pub struct CpuAllocator {}

// unsafe impl Send for CpuAllocator {}
// unsafe impl Sync for CpuAllocator {}

impl Allocator for CpuAllocator {
    fn allocate(&self, nbytes: usize) -> DataPtr {
        let data = vec![0u8; nbytes];
        let unique_void_ptr = UniqueVoidPtr::new_owned(data.into_boxed_slice());
        DataPtr::new(unique_void_ptr, Device::CPU)
    }
}
