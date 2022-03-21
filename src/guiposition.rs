pub mod guisizes {
    pub struct GUIHeight {
        size: f64,
    }

    impl GetSize for GUIHeight {
        fn get_size(&self) -> u16 {
            self.size.round() as u16
        }
    }

    pub struct GUIWidth {
        size: f64,
    }

    impl GetSize for GUIWidth {
        fn get_size(&self) -> u16 {
            self.size.round() as u16
        }
    }

    pub struct GUISize {
        size: f64,
    }

    impl GetSize for GUISize {
        fn get_size(&self) -> u16 {
            self.size.round() as u16
        }
    }

    impl SetSize for GUISize {
        fn from_pixels(pixels: f64) -> GUISize {
            GUISize { size: pixels }
        }
    }

    pub trait GetSize {
        fn get_size(&self) -> u16;
    }

    pub trait SetSize {
        fn from_pixels(pixels: f64) -> Self;
    }
}
