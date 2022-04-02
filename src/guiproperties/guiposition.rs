pub use guilengths::GUILength;
pub use guiposition::GUIPosition;
pub use guisize::GUISize;

/// A module of structs and tools for representing linear lengths.
pub mod guilengths {

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum LengthType {
        Generic,
        PhysicalLength,
        LogicalLength,
    }

    /// Represents a linear dimension (height or width).
    #[derive(Clone, Copy, Debug)]
    pub struct GUILength {
        pub length: f64,
        pub length_type: LengthType,
    }

    impl Default for GUILength {
        fn default() -> Self {
            GUILength {
                length: 0.,
                length_type: LengthType::LogicalLength,
            }
        }
    }

    impl PartialEq for GUILength {
        fn eq(&self, other: &GUILength) -> bool {
            let decimal_places = 8f64;
            let multiplyer = (10f64).powf(decimal_places);

            (self.length * multiplyer) as u64 == (other.length * multiplyer) as u64
                && self.length_type == other.length_type
        }
    }

    impl GUILength {
        pub fn negative(&self) -> Self {
            GUILength {
                length: -self.length,
                length_type: self.length_type,
            }
        }

        pub fn get_logical_length(&self, scale: &f64) -> f32 {
            let length = match self.length_type {
                LengthType::Generic => self.length,
                LengthType::LogicalLength => self.length,
                LengthType::PhysicalLength => self.length * scale,
            };

            // todo!("Percent clearly needs some work");

            length.round() as f32
        }

        pub fn get_physical_length(&self, scale: &f64) -> f32 {
            let length = match self.length_type {
                LengthType::Generic => self.length,
                LengthType::LogicalLength => self.length / scale,
                LengthType::PhysicalLength => self.length,
            };

            length.round() as f32
        }

        pub fn from_logical_pixels(pixels: f64) -> Self {
            GUILength {
                length: pixels,
                length_type: LengthType::LogicalLength,
            }
        }

        pub fn from_physical_pixels(pixels: f64) -> Self {
            GUILength {
                length: pixels,
                length_type: LengthType::PhysicalLength,
            }
        }

        pub fn add(&self, other: &GUILength) -> Self {
            if (self.length_type == other.length_type) || (self.length == 0. || other.length == 0.)
            {
                GUILength {
                    length: self.length + other.length,
                    length_type: self.length_type,
                }
            } else {
                panic!("Can only currently add lengths that are of the same type")
            }
        }

        pub fn subtract(&self, other: &GUILength) -> Self {
            if (self.length_type == other.length_type) || (self.length == 0. || other.length == 0.)
            {
                GUILength {
                    length: self.length - other.length,
                    length_type: self.length_type,
                }
            } else {
                panic!("Can only currently subtract lengths that are of the same type")
            }
        }

        pub fn multiply(&self, other: &GUILength) -> Self {
            if (self.length_type == other.length_type)
                || (self.length == 0. || other.length == 0.)
                || (self.length_type == LengthType::Generic
                    || other.length_type == LengthType::Generic)
            {
                GUILength {
                    length: self.length * other.length,
                    length_type: self.length_type,
                }
            } else {
                panic!("Can only currently multiply lengths that are of the same type")
            }
        }

        pub fn devide_by(&self, other: &GUILength) -> Self {
            if (self.length_type == other.length_type)
                || (self.length == 0. || other.length == 0.)
                || (self.length_type == LengthType::Generic
                    || other.length_type == LengthType::Generic)
            {
                GUILength {
                    length: self.length / other.length,
                    length_type: self.length_type,
                }
            } else {
                panic!("Can only currently devide lengths that are of the same type")
            }
        }
    }
}

/// A module of structs and tools for representing areas, width and height.
mod guisize {
    use super::guilengths::GUILength;

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
            GUISize { width, height }
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
            GUIPosition { x, y }
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
