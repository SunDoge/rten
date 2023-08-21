pub enum UniqueVoidPtr {
    Owned(Box<[u8]>),
}

impl UniqueVoidPtr {
    pub fn new_owned(data: Box<[u8]>) -> Self {
        Self::Owned(data)
    }

    pub fn as_ptr(&self) -> *const u8 {
        match self {
            Self::Owned(data) => data.as_ptr(),
        }
    }

    pub fn as_ref(&self) -> &[u8] {
        match self {
            Self::Owned(data) => data.as_ref(),
        }
    }

    pub fn as_mut(&mut self) -> &mut [u8] {
        match self {
            Self::Owned(data) => data.as_mut(),
        }
    }

    pub fn as_slice<T>(&self) -> &[T] {
        let ptr = self.as_ptr() as *const T;
        let len = self.as_ref().len() * std::mem::size_of::<u8>() / std::mem::size_of::<T>();
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }

    pub fn as_slice_mut<T>(&mut self) -> &mut [T] {
        let ptr = self.as_ptr() as *mut T;
        let len = self.as_ref().len() * std::mem::size_of::<u8>() / std::mem::size_of::<T>();
        unsafe { std::slice::from_raw_parts_mut(ptr, len) }
    }
}
