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

pub trait One: Sized + Copy {
    fn one() -> Self;
}

pub trait Element: Zero + One {}

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

macro_rules! impl_one {
    ($name:ty, $value:expr) => {
        impl One for $name {
            fn one() -> Self {
                $value
            }
        }
    };
}
impl_one!(f32, 1.0);
impl_one!(f64, 1.0);
impl_one!(u8, 1);
impl_one!(u32, 1);
impl_one!(u64, 1);
impl_one!(i8, 1);
impl_one!(i32, 1);
impl_one!(i64, 1);
impl_one!(bool, true);

impl<T> Element for T where T: Zero + One {}
