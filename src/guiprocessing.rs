use std::env;

use wgpu::{Backends, PowerPreference};

#[derive(Debug)]
pub enum GUIBackend {
    Vulkan,
    Metal,
    Dx11,
    Gl,
}

impl Default for GUIBackend {
    fn default() -> Self {
        //https://doc.rust-lang.org/std/env/consts/constant.OS.html
        match env::consts::OS {
            "linux" | "android" => Self::Vulkan,
            "macos" | "ios" => Self::Metal,
            "windows" => Self::Dx11,
            _ => Self::Vulkan,
        }
    }
}

pub enum GUIPowerPreference {
    /// Adapter that uses the least possible power. This is often an integrated GPU.
    LowPower,
    /// Adapter that has the highest performance. This is often a discrete GPU.
    HighPerformance,
}

impl Default for GUIPowerPreference {
    fn default() -> Self {
        Self::LowPower
    }
}

pub struct GUIProcessing {
    power_preference: GUIPowerPreference,
    backend: GUIBackend,
}

impl Default for GUIProcessing {
    fn default() -> Self {
        GUIProcessing {
            power_preference: GUIPowerPreference::default(),
            backend: GUIBackend::default(),
        }
    }
}

impl GUIProcessing {
    pub fn power_preference(&self) -> PowerPreference {
        match self.power_preference {
            GUIPowerPreference::LowPower => PowerPreference::LowPower,
            GUIPowerPreference::HighPerformance => PowerPreference::HighPerformance,
        }
    }
    pub fn backend(&self) -> Backends {
        match self.backend {
            GUIBackend::Vulkan => Backends::VULKAN,
            GUIBackend::Metal => Backends::METAL,
            GUIBackend::Dx11 => Backends::DX11,
            GUIBackend::Gl => Backends::GL,
        }
    }
}
