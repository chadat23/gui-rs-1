pub use guilengths::GUILength;
pub use guiposition::GUIPosition;
pub use guisize::GUISize;

/// A module of structs and tools for representing linear lengths.
pub mod guilengths {

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum LengthType {
        Generic,
        PhysicalSize,
        LogicalSize,
    }

    /// Represents a linear dimension (height or width).
    #[derive(Clone, Copy, Debug)]
    pub struct GUILength {
        pub length: f64,
        pub length_type: LengthType,
    }

    impl Default for GUILength {
        fn default() -> Self {
            GUILength { length: 0., length_type: LengthType::LogicalSize }
        }
    }

    impl PartialEq for GUILength {
        fn eq(&self, other: &GUILength) -> bool {
            let decimal_places = 8f64;
            let multiplyer = (10f64).powf(decimal_places);

            (self.length * multiplyer) as u64 == (other.length * multiplyer) as u64 && self.length_type == other.length_type
        }
    }

    impl GUILength {
        pub fn negative(&self) -> Self {
            GUILength {
                length: -self.length,
                length_type: self.length_type,
            }
        }

        pub fn get_logical_length(&self, scale: f64) -> f32 {
            let length = match self.length_type {
                LengthType::Generic => self.length,
                LengthType::LogicalSize => self.length,
                LengthType::PhysicalSize => self.length * scale,
            };

            length.round() as f32
        }

        pub fn get_physical_length(&self, scale: f64) -> f32 {
            let length = match self.length_type {
                LengthType::Generic => self.length,
                LengthType::LogicalSize => self.length / scale,
                LengthType::PhysicalSize => self.length,
            };

            length.round() as f32
        }

        pub fn from_logical_pixels(pixels: f64) -> Self {
            GUILength {
                length: pixels,
                length_type: LengthType::LogicalSize
            }
        }

        pub fn from_physical_pixels(pixels: f64) -> Self {
            GUILength {
                length: pixels,
                length_type: LengthType::PhysicalSize
            }
        }
    }
}

/// A module of structs and tools for representing areas, width and height.
mod guisize {
use super::guilengths::{GUILength, LengthType};

    /// Represents an area (width and height)
    #[derive(Copy, Clone, Debug)]
    pub struct GUISize {
        pub width: GUILength,
        pub height: GUILength,
    }

    impl Default for GUISize {
        fn default() -> GUISize {
            GUISize::from_logical_pixels(500., 500.)
        }
    }

    impl GUISize {
        pub fn from_lengths(width: GUILength, height: GUILength) -> Self {
            GUISize {
                width,
                height,
            }
        }

        pub fn from_logical_pixels(width: f64, height: f64) -> Self {
            GUISize {
                width: GUILength::from_logical_pixels(width),
                height: GUILength::from_logical_pixels(height),
            }
        }

        pub fn from_physical_pixels(width: f64, height: f64) -> Self {
            GUISize {
                width: GUILength::from_physical_pixels(width),
                height: GUILength::from_physical_pixels(height),
            }
        }
    }
}

mod guiposition {
    use super::guilengths::GUILength;

    #[derive(Clone, Copy, Debug)]
    pub struct GUIPosition {
        pub x: GUILength,
        pub y: GUILength,
    }

    impl PartialEq for GUIPosition {
        fn eq(&self, other: &GUIPosition) -> bool {
            self.x == other.x && self.y == other.y
        }
    }

    impl GUIPosition {
        pub fn new(x: GUILength, y: GUILength) -> Self {
            GUIPosition { x, y }
        }

        pub fn from_lengths(x: GUILength, y: GUILength) -> Self {
            GUIPosition {
                x,
                y,
            }
        }

        pub fn from_logical_pixels(x: f64, y: f64) -> Self {
            GUIPosition {
                x: GUILength::from_logical_pixels(x),
                y: GUILength::from_logical_pixels(y),
            }
        }

        pub fn from_physical_pixels(x: f64, y: f64) -> Self {
            GUIPosition {
                x: GUILength::from_physical_pixels(x),
                y: GUILength::from_physical_pixels(y),
            }
        }
    }
}
