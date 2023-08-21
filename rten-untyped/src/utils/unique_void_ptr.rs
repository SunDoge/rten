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
}
