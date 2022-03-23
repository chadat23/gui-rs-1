use winit::dpi::PhysicalSize;

use crate::guiposition::guilengths::GetLength;

/// Represents a gui window.
/// Given the number of properties that a window has,
/// rather than accounting for all of them in the creation
/// of the window, a mutable window struct is created with
/// default values, and then setters are used to set
/// individual properties that need to be customized.
pub struct GUIWindow {
    // /// The tile of the window.
    pub title: &'static str,
    /// The size of the window.
    pub size: winit::dpi::PhysicalSize<u32>,
    /// The minimum size of the window.
    pub min_size: winit::dpi::PhysicalSize<u32>,
}

impl Default for GUIWindow {
    // Returns a windows with all of the default values.
    fn default() -> GUIWindow {
        GUIWindow {
            title: "Form1",
            size: PhysicalSize { width: 500, height: 500 },
            min_size: PhysicalSize { width: 100, height: 100 },
        }
    }
}

impl GUIWindow {
    /// Sets the width of the window in units of logical pixels.
    pub fn set_width(&mut self, width: impl GetLength) -> &mut Self {
        self.size.width = width.get_length();
        self
    }

    /// Sets the height of the window in units of logical pixels.
    pub fn set_height(&mut self, height: impl GetLength) -> &mut Self {
        self.size.height = height.get_length();
        self
    }

    /// Sets the minimum width of the window in units of logical pixels.
    pub fn set_min_width(&mut self, width: impl GetLength) -> &mut Self {
        let width = width.get_length();
        self.min_size.width = if width > 0 {width} else {1};
        self
    }

    /// Sets the minimum height of the window in units of logical pixels.
    pub fn set_min_height(&mut self, height: impl GetLength) -> &mut Self {
        let height = height.get_length();
        self.min_size.height = if height > 0 {height} else {1};
        self
    }

    /// Sets the title of the window.
    pub fn set_title(&mut self, title: &'static str) -> &mut Self {
        self.title = title;
        self
    }
}