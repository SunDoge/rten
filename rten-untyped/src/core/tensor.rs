use rten_core::shape_and_strides::ShapeAndStrides;

use super::{storage::Storage, device::Device};

pub struct Tensor {
    storage: Storage,
    shape_and_strides: ShapeAndStrides,
}


impl Tensor {
    pub fn device(&self) -> Device {
        self.storage.device()
    }
}
