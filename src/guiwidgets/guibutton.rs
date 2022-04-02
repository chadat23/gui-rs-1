use crate::guiprocessing::vertices::Vertex;
use crate::guiproperties::guiposition::{GUILength, GUIPosition, GUISize};
use crate::guiproperties::guitraits::{Child, Family, Parent, Widget};
use crate::guiproperties::GUIColor;
use crate::guiwidgets::{widget_utils, widget_utils::arcs};

/// Represents a gui button.
// #[derive(Clone, Copy)]
pub struct GUIButton {
    /// The tile of the button.
    pub text: &'static str,
    /// The size of the button.
    pub size: GUISize,
    /// The location of the button.
    pub position: GUIPosition,
    /// Radius of the button corners.
    pub radius: GUILength,
    /// The background color for the button.
    pub background_color: GUIColor,
    /// A list of child widgets
    pub children: Option<Vec<Box<dyn Family>>>,
}

impl Default for GUIButton {
    // Returns a button with all of the default values.
    fn default() -> GUIButton {
        GUIButton {
            text: "Button",
            size: GUISize {
                width: GUILength::from_logical_pixels(200.),
                height: GUILength::from_logical_pixels(200.),
            },
            position: GUIPosition::from_logical_pixels(50., 100.),
            radius: GUILength::from_logical_pixels(50.),
            background_color: GUIColor {
                r: 0.7,
                g: 0.1,
                b: 0.4,
                a: 1.0,
            },
            children: None,
        }
    }
}

impl Widget for GUIButton {
    fn is_rendered(&self) -> bool {
        true
    }

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
    fn add_child(&mut self, child: Box<dyn Family>) {
        match &mut self.children {
            Some(children) => {
                children.push(child);
            }
            _ => {
                self.children = Some(Vec::from([child]));
            }
        };
    }

    /// Returns the number of children.
    fn children_len(&self) -> usize {
        match &self.children {
            Some(children) => children.len(),
            None => 0,
        }
    }

    /// Gets the children.
    fn get_area_children(&self) -> &Option<Vec<Box<dyn Family>>> {
        &self.children
    }
}

impl Child for GUIButton {
    fn get_vertices_and_indices(
        &self,
        parent_size: &GUISize,
        scale: &f64,
    ) -> (Vec<Vertex>, Vec<u16>) {
        const FASCET_COUNT: usize = 10;
        let mut top_left_radius = arcs::make_top_left_arc(self.radius, FASCET_COUNT);
        top_left_radius = widget_utils::translate(
            top_left_radius,
            &self.radius.add(&self.position.x),
            &self.radius.add(&self.position.y),
        );

        let top_right_radius = arcs::make_top_right_arc(self.radius, FASCET_COUNT);
        let top_right_radius = widget_utils::translate(
            top_right_radius,
            &self.size.width.subtract(&self.radius).add(&self.position.x),
            &self.radius.add(&self.position.y),
        );

        let bottom_left_radius = arcs::make_bottom_left_arc(self.radius, FASCET_COUNT);
        let bottom_left_radius = widget_utils::translate(
            bottom_left_radius,
            &self.radius.add(&self.position.x),
            &self
                .size
                .height
                .subtract(&self.radius)
                .add(&self.position.y),
        );

        let bottom_right_radius = arcs::make_bottom_right_arc(self.radius, FASCET_COUNT);
        let bottom_right_radius = widget_utils::translate(
            bottom_right_radius,
            &self.size.width.subtract(&self.radius).add(&self.position.x),
            &self
                .size
                .height
                .subtract(&self.radius)
                .add(&self.position.y),
        );

        top_left_radius.extend(bottom_left_radius);
        top_left_radius.extend(bottom_right_radius);
        top_left_radius.extend(top_right_radius);

        let mut vertices = Vec::with_capacity(top_left_radius.len());
        for position in top_left_radius.iter() {
            vertices.push(Vertex {
                position: [
                    position.x.get_logical_length(scale)
                        / parent_size.width.get_logical_length(scale)
                        - 1.,
                    -position.y.get_logical_length(scale)
                        / parent_size.height.get_logical_length(scale)
                        + 1.,
                    0.,
                ],
                color: [
                    self.background_color.r as f32,
                    self.background_color.g as f32,
                    self.background_color.b as f32,
                ],
            });
        }
        let number_of_triangles = vertices.len() - 2;
        let mut indices = Vec::with_capacity(number_of_triangles * 3);
        for i in 0..number_of_triangles {
            indices.push(0);
            indices.push((i + 1) as u16);
            indices.push((i + 2) as u16);
        }

        (vertices, indices)
    }
}

impl Family for GUIButton {}
