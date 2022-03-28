use crate::guiproperties::guitraits::{AreaWidget, Parent, Wind, Widget};
use crate::guiproperties::GUIColor;
use crate::guiproperties::GUIIcon;
use crate::guiproperties::guiposition::{GUIPosition, GUISize};

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
    /// Whether or not the window is resizable.
    pub resizable: bool,
    /// Whether or not the window is always on top of other windows.
    pub always_on_top: bool,
    /// The window's icon
    pub window_icon: Option<GUIIcon>,
    /// The window's IME position
    pub ime_position: Option<GUIPosition>,
    /// The background color for the window.
    pub background_color: GUIColor,
    /// A list of child widgets
    pub children: Option<Vec<Box<dyn Parent>>>,
}

impl Default for GUIWindow {
    // Returns a windows with all of the default values.
    fn default() -> GUIWindow {
        GUIWindow {
            title: "Form1",
            size: GUISize {
                width: 500.,
                height: 500.,
            },
            min_size: GUISize {
                width: 100.,
                height: 100.,
            },
            max_size: GUISize {
                width: 800.,
                height: 800.,
            },
            resizable: true,
            always_on_top: false,
            window_icon: None,
            ime_position: None,
            background_color: GUIColor {
                r: 0.4,
                g: 0.4,
                b: 0.4,
                a: 1.0,
            },
            children: None,
        }
    }
}

impl Widget for GUIWindow {}

impl AreaWidget for GUIWindow {
    /// Set the size (width and height) of the window in units of logical pixels.
    fn set_size(&mut self, size: GUISize) -> &mut Self {
        self.size = size;
        self
    } 
    
    // Set background color of the window.
    fn set_background_color(&mut self, color: GUIColor) -> &mut Self {
        self.background_color = color;
        self
    }
}

impl Wind for GUIWindow {
    /// Set the minimum size (width and height) of the window in units of logical pixels.
    fn set_min_size(&mut self, size: GUISize) -> &mut Self {
        self.size = size;
        self
    }

    /// Sets the title of the window.
    fn set_title(&mut self, title: &'static str) -> &mut Self {
        self.title = title;
        self
    }

    // Set whether or not a window is resizable.
    fn is_resizable(&mut self, resizable: bool) -> &mut Self {
        self.resizable = resizable;
        self
    }

    // Set whether or not a window is always on top.
    fn is_always_on_top(&mut self, always_on_top: bool) -> &mut Self {
        self.always_on_top = always_on_top;
        self
    }

    // Sets the window icon.
    fn set_window_icon(&mut self, icon: GUIIcon) -> &mut Self {
        self.window_icon = Some(icon);
        self
    }
}

impl Parent for GUIWindow {
    fn add_child(&mut self, child: Box<dyn Parent>) {
        match self.children {
            Some(children) => {
                children.push(child);
            }
            _ => {
                self.children = Some(Vec::from([child]));
            }
        };
    }
}
