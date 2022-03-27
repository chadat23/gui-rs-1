use crate::guiproperties::guicolor::GUIColor;
use crate::guiproperties::guiposition::GUISize;

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
    pub size: GUISize,
    /// The minimum size of the window.
    pub min_size: GUISize,
    /// The minimum size of the window.
    pub max_size: GUISize,
    /// The background color for the window.
    pub background_color: GUIColor,
}

impl Default for GUIWindow {
    // Returns a windows with all of the default values.
    fn default() -> GUIWindow {
        GUIWindow {
            title: "Form1",
            size: GUISize {
                width: 500,
                height: 500,
            },
            min_size: GUISize {
                width: 100,
                height: 100,
            },
            max_size: GUISize {
                width: 800,
                height: 800,
            },
            background_color: GUIColor {
                r: 0.4,
                g: 0.4,
                b: 0.4,
                a: 1.0,
            },
        }
    }
}

impl GUIWindow {
    /// Set the size (width and height) of the window in units of logical pixels.
    pub fn set_size(&mut self, size: GUISize) -> &mut Self {
        self.size = size;
        self
    }

    /// Set the minimum size (width and height) of the window in units of logical pixels.
    pub fn set_min_size(&mut self, size: GUISize) -> &mut Self {
        self.size = size;
        self
    }

    /// Sets the title of the window.
    pub fn set_title(&mut self, title: &'static str) -> &mut Self {
        self.title = title;
        self
    }

    // Set background color of the window.
    #[allow(non_snake_case)]
    pub fn set_background_color(&mut self, color: GUIColor) -> &mut Self {
        self.background_color = color;
        self
    }
}
