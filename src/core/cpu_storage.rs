use std::sync::{Arc, RwLock};

use super::{data_type::Zero, storage::Storage};

#[derive(Debug)]
pub struct CpuStorageImpl<T> {
    data: Box<[T]>,
}

#[derive(Debug, Clone)]
pub struct CpuStorage<T>(Arc<RwLock<CpuStorageImpl<T>>>);

impl<T> Storage for CpuStorage<T>
where
    T: Zero,
{
    type Elem = T;

    fn zeros(size: usize) -> Self {
        let data = vec![T::zero(); size].into_boxed_slice();
        Self(Arc::new(RwLock::new(CpuStorageImpl { data })))
    }
}
