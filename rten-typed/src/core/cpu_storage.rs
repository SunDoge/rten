use std::sync::{Arc, RwLock};

use rten_core::device::Device;

use super::{
    data_type::{Element, Zero},
    storage::Storage,
};

#[derive(Debug)]
pub struct CpuStorageImpl<T> {
    data: Box<[T]>,
}

#[derive(Debug, Clone)]
pub struct CpuStorage<T>(Arc<RwLock<CpuStorageImpl<T>>>);

impl<T> Storage for CpuStorage<T>
where
    T: Element,
{
    type Elem = T;

    fn zeros(size: usize, _device: Device) -> Self {
        let data = vec![T::zero(); size].into_boxed_slice();
        Self(Arc::new(RwLock::new(CpuStorageImpl { data })))
    }

    fn ones(size: usize) -> Self {
        let data = vec![T::one(); size].into_boxed_slice();
        Self(Arc::new(RwLock::new(CpuStorageImpl { data })))
    }

    fn to_vec(&self) -> Vec<Self::Elem> {
        self.0.read().unwrap().data.to_vec()
    }
}
