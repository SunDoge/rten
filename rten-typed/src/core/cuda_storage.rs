use cudarc::driver::{CudaDevice, CudaSlice, DeviceRepr, DeviceSlice, ValidAsZeroBits};
use std::sync::{Arc, RwLock};

use super::{data_type::Element, storage::Storage};

#[derive(Debug)]
pub struct CudaStorageImpl<T> {
    data: CudaSlice<T>,
}

#[derive(Debug, Clone)]
pub struct CudaStorage<T>(Arc<RwLock<CudaStorageImpl<T>>>);

impl<T> Storage for CudaStorage<T>
where
    T: Element + ValidAsZeroBits + DeviceRepr + Unpin,
{
    type Elem = T;

    fn zeros(size: usize) -> Self {
        let dev = CudaDevice::new(0).unwrap();
        let data = dev.alloc_zeros(size).unwrap();
        Self(Arc::new(RwLock::new(CudaStorageImpl { data })))
    }

    fn ones(size: usize) -> Self {
        let dev = CudaDevice::new(0).unwrap();

        let data = dev.htod_copy(vec![T::one(); size]).unwrap();

        Self(Arc::new(RwLock::new(CudaStorageImpl { data })))
    }

    fn to_vec(&self) -> Vec<Self::Elem> {
        let data = &self.0.write().unwrap().data;

        let dev = data.device();
        let cpu_data = dev.dtoh_sync_copy(data).unwrap();

        cpu_data
    }
}

pub struct Cuda {
    
}
