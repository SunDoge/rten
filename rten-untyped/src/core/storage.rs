use std::sync::{Arc, RwLock};

use crate::utils::data_ptr::DataPtr;

use super::device::{Device, DeviceType};

struct StorageImpl {
    data_ptr: DataPtr,
}

impl StorageImpl {
    pub fn new(data_ptr: DataPtr) -> Self {
        Self { data_ptr }
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
    pub fn new(data_ptr: DataPtr) -> Self {
        Self(Arc::new(RwLock::new(StorageImpl::new(data_ptr))))
    }

    pub fn is_unique(&self) -> bool {
        Arc::strong_count(&self.0) == 1
    }
}
