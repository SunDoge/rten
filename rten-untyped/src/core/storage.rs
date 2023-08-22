use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::utils::data_ptr::DataPtr;

use super::allocator::Allocator;
use super::cpu_allocator::CpuAllocator;

use rten_core::device::{Device, DeviceType};

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

    pub fn as_slice<T>(&self) -> &[T] {
        self.data_ptr.as_slice()
    }

    pub fn as_slice_mut<T>(&mut self) -> &mut [T] {
        self.data_ptr.as_slice_mut()
    }
}

#[derive(Clone)]
pub struct Storage(pub Arc<RwLock<StorageImpl>>);

impl From<StorageImpl> for Storage {
    fn from(value: StorageImpl) -> Self {
        Self::new(Arc::new(RwLock::new(value)))
    }
}

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

    pub fn read(&self) -> RwLockReadGuard<StorageImpl> {
        self.0.read().unwrap()
    }

    pub fn write(&self) -> RwLockWriteGuard<StorageImpl> {
        self.0.write().unwrap()
    }
}
