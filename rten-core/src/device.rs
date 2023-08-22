#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceType {
    Cpu,
    Cuda,
}

pub type DeviceIndex = i8;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Device {
    device_type: DeviceType,
    device_index: DeviceIndex,
}

impl Device {
    pub const CPU: Self = Self {
        device_type: DeviceType::Cpu,
        device_index: 0,
    };

    pub fn new(device_type: DeviceType, device_index: DeviceIndex) -> Self {
        Self {
            device_type,
            device_index,
        }
    }

    pub fn device_type(&self) -> DeviceType {
        self.device_type
    }

    pub fn device_index(&self) -> DeviceIndex {
        self.device_index
    }

    pub fn supports_as_strided(&self) -> bool {
        match self.device_type {
            DeviceType::Cpu | DeviceType::Cuda => true,
            _ => false,
        }
    }

    fn validate(&self) {}
}
