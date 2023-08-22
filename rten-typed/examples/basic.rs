use rten_core::device::Device;
use rten_typed::core::tensor::{CpuTensor, Tensor};

fn main() {
    let zero: CpuTensor<f32> = Tensor::zeros(&[1, 2, 3], Device::CPU);
    dbg!(zero);
}
