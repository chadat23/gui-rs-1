use crate::guiproperties::guicolor::GUIColor;
use crate::guiproperties::guiposition::GUISize;

/// Represents a gui button.
pub struct GUIButton {
    // /// The tile of the window.
    pub text: &'static str,
    /// The size of the window.
    pub size: GUISize,
    /// The background color for the window.
    pub background_color: GUIColor,
}

impl Default for GUIButton {
    // Returns a windows with all of the default values.
    fn default() -> GUIButton {
        GUIButton {
            text: "Button",
            size: GUISize {
                width: 20,
                height: 10,
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
