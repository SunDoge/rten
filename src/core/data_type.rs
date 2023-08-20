#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    F32,
    F64,
    U8,
    U32,
    U64,
    I8,
    I32,
    I64,
}

pub trait InferDataType {
    fn infer_data_type() -> DataType;
}

impl InferDataType for f32 {
    fn infer_data_type() -> DataType {
        DataType::F32
    }
}

pub trait Zero: Sized + Copy {
    fn zero() -> Self;
}

macro_rules! impl_zero {
    ($name:ty, $value:expr) => {
        impl Zero for $name {
            fn zero() -> Self {
                $value
            }
        }
    };
}
impl_zero!(f32, 0.0);
impl_zero!(f64, 0.0);
impl_zero!(u8, 0);
impl_zero!(u32, 0);
impl_zero!(u64, 0);
impl_zero!(i8, 0);
impl_zero!(i32, 0);
impl_zero!(i64, 0);
impl_zero!(bool, false);
