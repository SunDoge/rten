// pub struct Tensor<T, const RANK: usize, Storage, Desc> {
//     storage: Storage,
// }

use std::sync::{Arc, RwLock};

use super::{
    cpu_storage::CpuStorage,
    data_type::{DataType, InferDataType, Zero},
    device::Device,
    shape_and_strides::ShapeAndStrides,
    storage::Storage,
};

pub type Void = std::ffi::c_void;

// impl Tensor<Void> {}

// impl Tensor<Void> {
//     pub fn data_type(&self) -> DataType {
//         self.data_type.unwrap()
//     }
// }

pub type CpuTensor<T> = Tensor<T, CpuStorage<T>>;

#[derive(Debug)]
pub struct Tensor<T, S>
where
    S: Storage<Elem = T>,
{
    storage: S,
    shape_and_strides: ShapeAndStrides,
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

// impl<T> Tensor<T>
// where
//     T: Zero + Clone,
// {
//     pub fn zeros(shape: &[i64], device: Device) -> Self {
//         let shape_and_strides = ShapeAndStrides::new_contiguous(shape);
//         let data = Arc::new(RwLock::new(CpuStorage::zeros(
//             shape_and_strides.num_elements(),
//         )));
//         Self {
//             data,
//             shape_and_strides,
//             device,
//         }
//     }
// }

impl<T, S> Tensor<T, S>
where
    S: Storage<Elem = T>,
{
    pub fn zeros(shape: &[i64]) -> Self {
        let shape_and_strides = ShapeAndStrides::new_contiguous(shape);
        let storage = S::zeros(shape_and_strides.num_elements());
        Self {
            storage,
            shape_and_strides,
        }
    }
}

// impl<T> Tensor<T>
// where
//     T: InferDataType,
// {
//     pub fn infer_data_type() -> DataType {
//         T::infer_data_type()
//     }

//     pub fn data_type(&self) -> DataType {
//         Self::infer_data_type()
//     }
// }
