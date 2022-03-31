use crate::guiprocessing::vertices::Vertex;
use crate::guiproperties::guiposition::{GUILength, GUISize};
use crate::guiproperties::guitraits::{
    AreaChild, AreaFamily, AreaWidget, Child, Parent, PointChild, Widget,
};
use crate::guiproperties::{GUIColor, GUIPolygon};
use crate::guiwidgets::{widget_utils, widget_utils::arcs};

/// Represents a gui button.
// #[derive(Clone, Copy)]
pub struct GUIButton {
    // /// The tile of the button.
    pub text: &'static str,
    /// The size of the button.
    pub size: GUISize,
    /// Radius of the button corners.
    pub radius: GUILength,
    /// The background color for the button.
    pub background_color: GUIColor,
    // pub polygon: Option<GUIPolygon>,
    /// A list of child widgets
    pub area_children: Option<Vec<Box<dyn AreaFamily>>>,
    /// A list of child widgets
    pub point_children: Option<Vec<Box<dyn PointChild>>>,
}

impl Default for GUIButton {
    // Returns a button with all of the default values.
    fn default() -> GUIButton {
        GUIButton {
            text: "Button",
            size: GUISize {
                width: GUILength::from_logical_pixels(20.),
                height: GUILength::from_logical_pixels(10.),
            },
            radius: GUILength::from_logical_pixels(3.),
            background_color: GUIColor {
                r: 0.7,
                g: 0.1,
                b: 0.4,
                a: 1.0,
            },
            // polygon: None,
            point_children: None,
            area_children: None,
        }
    }
}

impl Widget for GUIButton {
    fn is_rendered(&self) -> bool {
        true
    }
}

impl AreaWidget for GUIButton {
    /// Set the size (width and height) of the window in units of logical pixels.
    fn set_size(&mut self, size: GUISize) {
        self.size = size;
    }

    // Set background color of the window.
    fn set_background_color(&mut self, color: GUIColor) {
        self.background_color = color;
    }
}

impl Parent for GUIButton {
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

impl Child for GUIButton {}

impl AreaChild for GUIButton {
    fn get_vertices_and_indices(&self, scale: f64) -> (Vec<Vertex>, Vec<u16>) {
        const RADIUS_FASCETS: usize = 10;
        let top_left_radius = arcs::make_top_left_arc(self.radius, RADIUS_FASCETS);
        let mut top_left_radius =
            widget_utils::translate(top_left_radius, &self.radius, &self.radius.negative());

        let top_right_radius = arcs::make_top_right_arc(self.radius, RADIUS_FASCETS);
        let top_right_radius = widget_utils::translate(
            top_right_radius,
            &GUILength::default(),
            &self.radius.negative(),
        );

        let bottom_left_radius = arcs::make_bottom_left_arc(self.radius, RADIUS_FASCETS);
        let bottom_left_radius =
            widget_utils::translate(bottom_left_radius, &self.radius, &GUILength::default());

        let bottom_right_radius = arcs::make_bottom_right_arc(self.radius, RADIUS_FASCETS);
        let bottom_right_radius =
            widget_utils::translate(bottom_right_radius, &self.radius, &GUILength::default());

        top_left_radius.extend(top_right_radius);
        top_left_radius.extend(bottom_right_radius);
        top_left_radius.extend(bottom_left_radius);

        let mut vertices = Vec::with_capacity(top_left_radius.len());
        for position in top_left_radius.iter() {
            vertices.push(Vertex {
                position: [position.x.get_physical_length(scale) as f32, position.y.get_physical_length(scale) as f32, 0.],
                color: [
                    self.background_color.r as f32,
                    self.background_color.g as f32,
                    self.background_color.b as f32,
                ],
            });
        }
        let number_of_triangles = top_left_radius.len() - 2;
        let mut indices = Vec::with_capacity(number_of_triangles * 3);
        for i in 0..number_of_triangles {
            indices.push(0);
            indices.push((i + 1) as u16);
            indices.push((i + 2) as u16);
        }

        (vertices, indices)
    }
}

impl AreaFamily for GUIButton {}
