// pub struct Tensor<T, const RANK: usize, Storage, Desc> {
//     storage: Storage,
// }

use std::sync::{Arc, RwLock};

use super::{device::Device, shape_and_strides::ShapeAndStrides, storage::CpuStorage};

pub struct Tensor<T> {
    data: Arc<RwLock<CpuStorage<T>>>,
    shape_and_strides: ShapeAndStrides,
    device: Device,
}
