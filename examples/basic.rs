use rten::core::tensor::{CpuTensor, Tensor, TensorLike};

fn main() {
    let x: CpuTensor<f32> = Tensor::zeros(&[1, 2, 3]);
    dbg!(x);
}
