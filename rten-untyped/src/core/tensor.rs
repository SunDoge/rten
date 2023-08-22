use rten_core::shape_and_strides::ShapeAndStrides;

use crate::utils::data_ptr::DataPtr;

use super::{
    cpu_allocator::CPU_ALLOCATOR,
    storage::{Storage, StorageImpl},
};

use rten_core::device::Device;

pub struct Tensor {
    storage: Storage,
    shape_and_strides: ShapeAndStrides,
}

impl Tensor {
    pub fn new(storage: Storage, shape_and_strides: ShapeAndStrides) -> Self {
        Self {
            storage,
            shape_and_strides,
        }
    }

    pub fn shape(&self) -> &[i64] {
        self.shape_and_strides.shape()
    }

    pub fn strides(&self) -> &[i64] {
        self.shape_and_strides.strides()
    }
    pub fn device(&self) -> Device {
        self.storage.device()
    }

    pub fn is_contiguous(&self) -> bool {
        self.shape_and_strides.is_contiguous()
    }

    pub fn storage(&self) -> Storage {
        self.storage.clone()
    }

    pub fn full<T>(shape: &[i64], value: T) -> Self
    where
        T: Copy,
    {
        let shape_and_strides = ShapeAndStrides::new_contiguous(shape);
        let mut storage_impl = StorageImpl::new_with_size(
            shape.iter().product::<i64>() as usize * std::mem::size_of::<T>(),
            CPU_ALLOCATOR.clone(),
            true,
        );
        storage_impl.as_slice_mut().fill(value);
        let storage = Storage::from(storage_impl);

        Self {
            storage,
            shape_and_strides,
        }
    }
}
