use super::{GUIColor, GUIIcon};
use super::guiposition::GUISize;

pub trait Widget {}

pub trait AreaWidget: Widget {
    /// Set the size (width and height) of the window in units of logical pixels.
    fn set_size(&mut self, size: GUISize) -> &mut Self;

    // Set background color of the window.
    #[allow(non_snake_case)]
    fn set_background_color(&mut self, color: GUIColor) -> &mut Self;
}

pub trait Wind: AreaWidget {
    /// Set the minimum size (width and height) of the window in units of logical pixels.
    fn set_min_size(&mut self, size: GUISize) -> &mut Self;

    /// Sets the title of the window.
    fn set_title(&mut self, title: &'static str) -> &mut Self;

    // Set whether or not a window is resizable.
    fn is_resizable(&mut self, resizable: bool) -> &mut Self;

    // Set whether or not a window is always on top.
    fn is_always_on_top(&mut self, always_on_top: bool) -> &mut Self;

    // Sets the window icon.
    fn set_window_icon(&mut self, icon: GUIIcon) -> &mut Self;
}

pub trait Parent: Widget {
    fn add_child(&mut self, child: Box<dyn Parent>);
}

pub trait Child: Widget {
    // fn render(&mut self);
}
