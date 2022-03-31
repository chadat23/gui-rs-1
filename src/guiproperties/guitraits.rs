use super::guiposition::GUISize;
use super::{GUIColor, GUIIcon};
use crate::guiprocessing::vertices::Vertex;

pub trait Widget {
    fn is_rendered(&self) -> bool;
}

pub trait AreaWidget: Widget {
    /// Set the size (width and height) of the window in units of logical pixels.
    fn set_size(&mut self, size: GUISize);

    // Set background color of the window.
    #[allow(non_snake_case)]
    fn set_background_color(&mut self, color: GUIColor);
}

pub trait Wind: AreaWidget {
    /// Set the minimum size (width and height) of the window in units of logical pixels.
    fn set_min_size(&mut self, size: GUISize);

    /// Sets the title of the window.
    fn set_title(&mut self, title: &'static str);

    // Set whether or not a window is resizable.
    fn set_resizable(&mut self, resizable: bool);

    // Set whether or not a window is always on top.
    fn set_always_on_top(&mut self, always_on_top: bool);

    // Sets the window icon.
    fn set_window_icon(&mut self, icon: GUIIcon);
}

pub trait Parent: Widget {
    fn add_area_child(&mut self, child: Box<dyn AreaFamily>);
    fn add_point_child(&mut self, child: Box<dyn PointChild>);

    fn children_len(&self) -> usize;

    fn get_area_children(&self) -> &Option<Vec<Box<dyn AreaFamily>>>;
}

pub trait Child: Widget {
    // fn render(&mut self);
}

pub trait AreaChild: Child {
    fn get_vertices_and_indices(&self, scale: f64) -> (Vec<Vertex>, Vec<u16>);
}

pub trait AreaFamily: AreaChild + Parent + AreaWidget {}

pub trait PointChild: Child {} 
