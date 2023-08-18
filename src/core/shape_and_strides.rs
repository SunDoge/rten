const MAX_INLINE_SIZE: usize = 5;

pub enum ShapeAndStrides {
    Inline {
        len: usize,
        storage: [i64; MAX_INLINE_SIZE * 2],
    },
    OutOfLine(Box<[i64]>),
}

impl Default for ShapeAndStrides {
    fn default() -> Self {
        Self::Inline {
            len: 1,
            storage: [0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
        }
    }
}

impl ShapeAndStrides {
    pub fn new(shape: &[i64], strides: &[i64]) -> Self {
        assert_eq!(shape.len(), strides.len());
        let len = shape.len();
        if len <= MAX_INLINE_SIZE {
            let mut storage = [0; MAX_INLINE_SIZE * 2];
            storage[..len].copy_from_slice(shape);
            storage[MAX_INLINE_SIZE..].copy_from_slice(strides);
            Self::Inline { len, storage }
        } else {
            let mut v = Vec::with_capacity(len * 2);
            v.extend_from_slice(shape);
            v.extend_from_slice(strides);
            Self::OutOfLine(v.into_boxed_slice())
        }
    }

    pub fn len(&self) -> usize {
        match self {
            ShapeAndStrides::Inline { len, .. } => *len,
            ShapeAndStrides::OutOfLine(v) => v.len() / 2,
        }
    }

    pub fn shape(&self) -> &[i64] {
        let len = self.len();
        match self {
            ShapeAndStrides::Inline { storage, .. } => &storage[..len],
            ShapeAndStrides::OutOfLine(v) => &v[..len],
        }
    }

    pub fn strides(&self) -> &[i64] {
        let len = self.len();
        match self {
            ShapeAndStrides::Inline { storage, .. } => &storage[MAX_INLINE_SIZE..][..len],
            ShapeAndStrides::OutOfLine(v) => &v[len..],
        }
    }

    pub fn shape_mut(&mut self) -> &mut [i64] {
        let len = self.len();
        match self {
            ShapeAndStrides::Inline { storage, .. } => &mut storage[..len],
            ShapeAndStrides::OutOfLine(v) => &mut v[..len],
        }
    }

    pub fn strides_mut(&mut self) -> &mut [i64] {
        let len = self.len();
        match self {
            ShapeAndStrides::Inline { storage, .. } => &mut storage[MAX_INLINE_SIZE..][..len],
            ShapeAndStrides::OutOfLine(v) => &mut v[len..],
        }
    }

    pub fn is_inline(&self) -> bool {
        match self {
            ShapeAndStrides::Inline { .. } => true,
            ShapeAndStrides::OutOfLine { .. } => false,
        }
    }
}
