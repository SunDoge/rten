// pub struct Tensor<T, const RANK: usize, Storage, Desc> {
//     storage: Storage,
// }

use std::sync::{Arc, RwLock};

use super::{
    data_type::{DataType, InferDataType, Zero},
    device::Device,
    shape_and_strides::ShapeAndStrides,
    storage::CpuStorage,
};

pub type Void = std::ffi::c_void;

// impl Tensor<Void> {}

// impl Tensor<Void> {
//     pub fn data_type(&self) -> DataType {
//         self.data_type.unwrap()
//     }
// }

#[derive(Debug)]
pub struct Tensor<T> {
    data: Arc<RwLock<CpuStorage<T>>>,
    shape_and_strides: ShapeAndStrides,
    device: Device,
    // data_type: Option<DataType>,
}

pub struct TensorBuilder<'a, T> {
    data: &'a [T],
    shape: &'a [i64],
    strides: Option<&'a [i64]>,
    device: Device,
}

// impl<'a, T> TensorBuilder<'a, T> {
//     pub fn new(data: &[T], )
// }

impl<T> Tensor<T>
where
    T: Zero + Clone,
{
    pub fn zeros(shape: &[i64], device: Device) -> Self {
        let shape_and_strides = ShapeAndStrides::new_contiguous(shape);
        let data = Arc::new(RwLock::new(CpuStorage::zeros(
            shape_and_strides.num_elements(),
        )));
        Self {
            data,
            shape_and_strides,
            device,
        }
    }
}

impl<T> Tensor<T>
where
    T: InferDataType,
{
    pub fn infer_data_type() -> DataType {
        T::infer_data_type()
    }

    pub fn data_type(&self) -> DataType {
        Self::infer_data_type()
    }
}
