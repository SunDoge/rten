use std::sync::{Arc, Mutex, RwLock};

use crate::utils::data_ptr::DataPtr;

use super::allocator::Allocator;
use super::{
    cpu_allocator::CpuAllocator,
    device::{Device, DeviceType},
};

pub struct StorageImpl {
    data_ptr: DataPtr,
    allocator: Option<Arc<Mutex<dyn Allocator>>>,
    resizable: bool,
}

impl StorageImpl {
    pub fn new(
        data_ptr: DataPtr,
        allocator: Option<Arc<Mutex<dyn Allocator>>>,
        resizable: bool,
    ) -> Self {
        assert!(resizable && allocator.is_some());
        Self {
            data_ptr,
            allocator,
            resizable,
        }
    }

    pub fn new_with_size(
        size_bytes: usize,
        allocator: Arc<Mutex<dyn Allocator>>,
        resizable: bool,
    ) -> Self {
        let data_ptr = allocator.lock().unwrap().allocate(size_bytes);
        Self::new(data_ptr, Some(allocator), resizable)
    }

    pub fn data_ptr(&self) -> &DataPtr {
        &self.data_ptr
    }

    pub fn data_ptr_mut(&mut self) -> &mut DataPtr {
        &mut self.data_ptr
    }

    pub fn device(&self) -> Device {
        self.data_ptr.device()
    }

    pub fn device_type(&self) -> DeviceType {
        self.data_ptr.device().device_type()
    }
}

pub struct Storage(Arc<RwLock<StorageImpl>>);

impl Storage {
    pub fn new(ptr: Arc<RwLock<StorageImpl>>) -> Self {
        Self(ptr)
    }

    pub fn new_with_size(
        size_bytes: usize,
        allocator: Arc<Mutex<dyn Allocator>>,
        resizable: bool,
    ) -> Self {
        let ptr = StorageImpl::new_with_size(size_bytes, allocator, resizable);
        Self::new(Arc::new(RwLock::new(ptr)))
    }

    pub fn new_with_data(
        data_ptr: DataPtr,
        allocator: Option<Arc<Mutex<dyn Allocator>>>,
        resizable: bool,
    ) -> Self {
        let ptr = StorageImpl::new(data_ptr, allocator, resizable);
        Self::new(Arc::new(RwLock::new(ptr)))
    }

    pub fn is_unique(&self) -> bool {
        Arc::strong_count(&self.0) == 1
    }

    pub fn device(&self) -> Device {
        self.0.read().unwrap().device()
    }
}
