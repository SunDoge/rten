const MAX_INLINE_SIZE: usize = 5;

pub fn calculate_contiguous_strides(shape: &[i64]) -> Vec<i64> {
    let rank = shape.len();
    let mut strides = vec![1; rank];
    for i in (0..rank - 1).rev() {
        strides[i] = strides[i + 1] * shape[i + 1];
    }
    strides
}

pub enum ShapeAndStrides {
    Inline {
        len: usize,
        storage: [i64; MAX_INLINE_SIZE * 2],
    },
    OutOfLine(Box<[i64]>),
}

impl Default for ShapeAndStrides {
    fn default() -> Self {
        let mut storage = [0; MAX_INLINE_SIZE * 2];
        storage[MAX_INLINE_SIZE] = 1; // stride 0 is 1 by default.
        Self::Inline { len: 1, storage }
    }
}

impl ShapeAndStrides {
    pub fn new(shape: &[i64], strides: &[i64]) -> Self {
        assert_eq!(shape.len(), strides.len());
        let len = shape.len();
        if len <= MAX_INLINE_SIZE {
            let mut storage = [0; MAX_INLINE_SIZE * 2];
            storage[..len].copy_from_slice(shape);
            storage[MAX_INLINE_SIZE..][..len].copy_from_slice(strides);
            Self::Inline { len, storage }
        } else {
            let mut v = Vec::with_capacity(len * 2);
            v.extend_from_slice(shape);
            v.extend_from_slice(strides);
            Self::OutOfLine(v.into_boxed_slice())
        }
    }

    pub fn new_contiguous(shape: &[i64]) -> Self {
        let strides = calculate_contiguous_strides(shape);
        Self::new(shape, &strides)
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

    /// Get num elements in Tensor.
    pub fn num_elements(&self) -> usize {
        self.shape().iter().product::<i64>() as usize
    }
}

impl std::fmt::Debug for ShapeAndStrides {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShapeAndStrides")
            .field("shape", &self.shape())
            .field("strides", &self.strides())
            .finish()
    }
}
