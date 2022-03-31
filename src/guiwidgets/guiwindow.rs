use  winit::window::Window;

use crate::guiproperties::guiposition::{GUIPosition, GUISize};
use crate::guiproperties::guitraits::{AreaFamily, AreaWidget, Parent, PointChild, Widget, Wind};
use crate::guiproperties::GUIColor;
use crate::guiproperties::GUIIcon;
use crate::guiproperties::guiposition::GUILength;

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
    /// A list of child widgets that have an area
    pub area_children: Option<Vec<Box<dyn AreaFamily>>>,
    /// A list of child widgets that don't have an area
    pub point_children: Option<Vec<Box<dyn PointChild>>>,
    /// The scale that converts between the devices logical and physical pixels.
    pub logical_scale: Option<f64>,
}

impl Default for GUIWindow {
    // Returns a windows with all of the default values.
    fn default() -> GUIWindow {
        GUIWindow {
            title: "Form1",
            size: GUISize {
                width: GUILength::from_logical_pixels(500.),
                height: GUILength::from_logical_pixels(500.),
            },
            min_size: GUISize {
                width: GUILength::from_logical_pixels(100.),
                height: GUILength::from_logical_pixels(100.),
            },
            max_size: GUISize {
                width: GUILength::from_logical_pixels(800.),
                height: GUILength::from_logical_pixels(800.),
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
            area_children: None,
            point_children: None,
            logical_scale: None,
        }
    }
}

impl Widget for GUIWindow {
    fn is_rendered(&self) -> bool {
        true
    }
}

impl AreaWidget for GUIWindow {
    /// Set the size (width and height) of the window in units of logical pixels.
    fn set_size(&mut self, size: GUISize) {
        self.size = size;
    }

    // Set background color of the window.
    fn set_background_color(&mut self, color: GUIColor) {
        self.background_color = color;
    }
}

impl Wind for GUIWindow {
    /// Set the minimum size (width and height) of the window in units of logical pixels.
    fn set_min_size(&mut self, size: GUISize) {
        self.size = size;
    }

    /// Sets the title of the window.
    fn set_title(&mut self, title: &'static str) {
        self.title = title;
    }

    // Set whether or not a window is resizable.
    fn set_resizable(&mut self, resizable: bool) {
        self.resizable = resizable;
    }

    // Set whether or not a window is always on top.
    fn set_always_on_top(&mut self, always_on_top: bool) {
        self.always_on_top = always_on_top;
    }

    // Sets the window icon.
    fn set_window_icon(&mut self, icon: GUIIcon) {
        self.window_icon = Some(icon);
    }
}

impl Parent for GUIWindow {
    /// Adds a child to the GUIWindow.
    /// Children, and grandchildren will be rendered in the order
    /// in which they're added so children that should be
    /// visually obscured by other children should be added
    /// before the obscuring children.
    fn add_area_child(&mut self, child: Box<dyn AreaFamily>) {
        match &mut self.area_children {
            Some(children) => {
                children.push(child);
            }
            _ => {
                self.area_children = Some(Vec::from([child]));
            }
        };
    }

    fn add_point_child(&mut self, child: Box<dyn PointChild>) {
        match &mut self.point_children {
            Some(children) => {
                children.push(child);
            }
            _ => {
                self.point_children = Some(Vec::from([child]));
            }
        };
    }

    /// Returns the number of children.
    fn children_len(&self) -> usize {
        match &self.area_children {
            Some(children) => children.len(),
            None => 0,
        }
    }

    /// Gets the children.
    fn get_area_children(&self) -> &Option<Vec<Box<dyn AreaFamily>>> {
        &self.area_children
    }
}
