//! Module d'abstraction matériel (device)
pub mod auto;
pub mod cpu;
pub mod device_trait;
pub mod gpu;

pub use auto::best_device;
pub use cpu::CpuDevice;
pub use device_trait::Device;
pub use gpu::GpuDevice;
