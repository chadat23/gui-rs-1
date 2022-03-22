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
        fn from_pixels(pixels: f64) -> GUILength {
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
