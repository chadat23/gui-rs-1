pub use guilengths::{GUIHeight, GUILength, GUIWidth};
pub use guiposition::GUIPosition;
pub use guisize::GUISize;

/// A module of structs and tools for representing linear lengths.
pub mod guilengths {
    /// Represents heights.
    pub struct GUIHeight {
        length: f64,
    }

    impl GetLength for GUIHeight {
        /// Returns the stored height.
        fn get_length(&self) -> u32 {
            self.length.round() as u32
        }
    }

    /// Represents widths.
    pub struct GUIWidth {
        length: f64,
    }

    impl GetLength for GUIWidth {
        /// Returns the stored width
        fn get_length(&self) -> u32 {
            self.length.round() as u32
        }
    }

    /// Represents a linear dimension (height or width).
    #[derive(Clone, Copy)]
    pub struct GUILength {
        pub length: f64,
    }

    impl GetLength for GUILength {
        /// Returns the stored length
        fn get_length(&self) -> u32 {
            self.length.round() as u32
        }
    }

    impl SetLength for GUILength {
        /// Sets the length of the GUILength in logical pixels.
        fn from_pixels(pixels: f64) -> Self {
            GUILength { length: pixels }
        }
    }

    impl GUILength {
        pub fn zero() -> Self {
            GUILength { length: 0. }
        }

        pub fn negative(&self) -> Self {
            GUILength {
                length: -self.length,
            }
        }
    }

    pub trait GetLength {
        /// Returns the length of the property
        fn get_length(&self) -> u32;
    }

    pub trait SetLength {
        /// Sets the length of the property in logical pixels.
        fn from_pixels(pixels: f64) -> Self;
    }
}

/// A module of structs and tools for representing areas, width and height.
mod guisize {
    use super::guilengths::{GUILength, GetLength};

    /// Represents an area (width and height)
    #[derive(Copy, Clone, Debug)]
    pub struct GUISize {
        pub width: f64,
        pub height: f64,
    }

    impl Default for GUISize {
        /// Creates a GUISize with default values.
        /// Width: 500,
        /// Height: 500
        fn default() -> GUISize {
            GUISize {
                width: 500.,
                height: 500.,
            }
        }
    }

    impl GUISize {
        /// Creates a size from the input size in units of pixels.
        pub fn from_pixels(width: u32, height: u32) -> Self {
            GUISize {
                width: width as f64,
                height: height as f64,
            }
        }

        /// Creates a size from the input size from two lengths,
        /// width and height.
        pub fn from_lengths(width: GUILength, height: GUILength) -> Self {
            GUISize {
                width: width.length,
                height: height.length,
            }
        }

        pub fn get_width_in_pixels(&self) -> u32 {
            self.width.round() as u32
        }

        pub fn get_height_in_pixels(&self) -> u32 {
            self.height.round() as u32
        }
    }
}

mod guiposition {
    use super::guilengths::GUILength;

    #[derive(Clone, Copy, Debug)]
    pub struct GUIPosition {
        pub x: f64,
        pub y: f64,
    }

    impl PartialEq for GUIPosition {
        fn eq(&self, other: &GUIPosition) -> bool {
            let decimal_places = 9;
            let multiplyer = (10u32.pow(decimal_places)) as f64;

            (self.x * multiplyer).round() == (other.x * multiplyer).round()
                && (self.y * multiplyer).round() == (other.y * multiplyer).round()
        }
    }

    impl GUIPosition {
        pub fn new(x: f64, y: f64) -> Self {
            GUIPosition { x, y }
        }

        pub fn from_lengths(x: GUILength, y: GUILength) -> Self {
            GUIPosition {
                x: x.length,
                y: y.length,
            }
        }
    }
}
