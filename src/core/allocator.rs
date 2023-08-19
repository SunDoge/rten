use crate::utils::data_ptr::DataPtr;

pub trait Allocator {
    fn allocate(n: usize) -> DataPtr;
}
