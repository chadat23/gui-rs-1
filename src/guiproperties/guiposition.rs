pub use guilengths::{GUIHeight, GUILength, GUIWidth};
pub use guisize::GUISize;

/// A module of structs and tools for representing linear lengths.
mod guilengths {
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
    pub struct GUILength {
        length: f64,
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
        pub width: u32,
        pub height: u32,
    }

    impl Default for GUISize {
        /// Creates a GUISize with default values.
        /// Width: 500,
        /// Height: 500
        fn default() -> GUISize {
            GUISize {
                width: 500,
                height: 500,
            }
        }
    }

    impl GUISize {
        /// Creates a size from the input size in units of pixels.
        pub fn from_pixels(width: u32, height: u32) -> Self {
            GUISize { width, height }
        }

        /// Creates a size from the input size from two lengths,
        /// width and height.
        pub fn from_lengths(width: GUILength, height: GUILength) -> Self {
            GUISize {
                width: width.get_length(),
                height: height.get_length(),
            }
        }
    }
}
