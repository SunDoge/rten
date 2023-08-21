use rten_untyped::core::tensor::Tensor;

fn main() {
    let x = Tensor::full(&[1, 2, 3], 1.0f32);
    dbg!(x.device(), x.shape(), x.strides());
    dbg!(x.storage().read().as_slice::<f32>());
}
