use rten::core::tensor::{CpuTensor, Tensor, TensorLike};

fn main() {
    let zero: CpuTensor<f32> = Tensor::zeros(&[1, 2, 3]);
    dbg!(zero);
    let one: CpuTensor<f64> = Tensor::ones(&[1, 2, 3]);
}
