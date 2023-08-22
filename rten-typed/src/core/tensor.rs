// pub struct Tensor<T, const RANK: usize, Storage, Desc> {
//     storage: Storage,
// }

use std::sync::{Arc, RwLock};

use super::{
    cpu_storage::CpuStorage,
    data_type::{DataType, Element, InferDataType, Zero},
    storage::Storage,
};

#[cfg(feature = "cuda")]
use super::cuda_storage::CudaStorage;

use rten_core::device::Device;
use rten_core::shape_and_strides::ShapeAndStrides;

// impl Tensor<Void> {}

// impl Tensor<Void> {
//     pub fn data_type(&self) -> DataType {
//         self.data_type.unwrap()
//     }
// }

pub type CpuTensor<T> = Tensor<T, CpuStorage<T>>;

#[cfg(feature = "cuda")]
pub type CudaTensor<T> = Tensor<T, CudaStorage<T>>;

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

// pub trait TensorLike {
//     fn zeros(shape: &[i64]) -> Self;
//     fn ones(shape: &[i64]) -> Self;
// }

impl<T, S> Tensor<T, S>
where
    S: Storage<Elem = T>,
{
    pub fn zeros(shape: &[i64], device: Device) -> Self {
        let shape_and_strides = ShapeAndStrides::new_contiguous(shape);
        let storage = S::zeros(shape_and_strides.num_elements(), device);
        Self {
            storage,
            shape_and_strides,
        }
    }

    pub fn ones(shape: &[i64]) -> Self {
        let shape_and_strides = ShapeAndStrides::new_contiguous(shape);
        let storage = S::ones(shape_and_strides.num_elements());
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

impl<T, S> Tensor<T, S>
where
    S: Storage<Elem = T>,
{
    pub fn to_vec(&self) -> Vec<T> {
        self.storage.to_vec()
    }
}

pub enum UntypedTensor {
    Cpu(UnTypedCpuTensor),
}

pub enum UnTypedCpuTensor {
    F32(CpuTensor<f32>),
    F64(CpuTensor<f64>),
}
