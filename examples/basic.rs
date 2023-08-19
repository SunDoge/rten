use rten::core::{device::Device, tensor::Tensor};

fn main() {
    let x: Tensor<f32> = Tensor::zeros(&[1, 2, 3], Device::CPU);
    dbg!(x);
}
