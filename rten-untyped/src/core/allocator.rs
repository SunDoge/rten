use crate::utils::data_ptr::DataPtr;

pub trait Allocator: Send + Sync {
    fn allocate(&self, nbytes: usize) -> DataPtr;
}
