use std::time::Instant;

use uuid::Uuid;

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
    pub children: Vec<Box<dyn Family>>,
    /// The human readable name of the button
    pub name: &'static str,
    pub id: u128,
}

const DEFAULT_BUTTON_NAME: &str = "this is the default name of the window";

// #[cfg(feature = "v4")] 
impl Default for GUIButton {
    // Returns a button with all of the default values.
    // 
    fn default() -> GUIButton {

        GUIButton {
            text: "Button",
            size: GUISize {
                width: GUILength::from_pixels(200.),
                height: GUILength::from_pixels(100.),
            },
            position: GUIPosition::from_pixels(0., 0.),
            radius: GUILength::from_pixels(25.),
            background_color: GUIColor {
                r: 0.7,
                g: 0.1,
                b: 0.4,
                a: 1.0,
            },
            children: Vec::new(),
            name: DEFAULT_BUTTON_NAME,
            id: Uuid::new_v4().as_u128(),
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

    fn set_id(&mut self, id: u128) {
        self.id = id;
    }
}

impl Parent for GUIButton {
    /// Adds a child to the GUIWindow.
    /// Children, and grandchildren will be rendered in the order
    /// in which they're added so children that should be
    /// visually obscured by other children should be added
    /// before the obscuring children.
    fn add_child(&mut self, child: Box<dyn Family>) {
        self.children.push(child);
    }

    fn set_children(&mut self, children: Vec<Box<dyn Family>>) {
        self.children = children;
    }

    /// Gets the children.
    fn get_children(&self) -> &Vec<Box<dyn Family>> {
        &self.children
    }

    // /// Gets the children.
    // fn get_children_mut(&mut self) -> &mut Vec<Box<dyn Family>> {
    //     &mut self.children
    // }

    // fn give_children(&mut self) -> Vec<Box<dyn Family>> {
    //     self.children
    // }
}

impl Child for GUIButton {
    fn get_vertices_and_indices(
        &self,
        parent_size: &GUISize,
        indice_offset: u16,
    ) -> (Vec<Vertex>, Vec<u16>) {
        const FASCET_COUNT: usize = 7;
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
                    (position.x.get_length() / parent_size.width.get_length() - 1.) as f32,
                    (-position.y.get_length() / parent_size.height.get_length() + 1.) as f32,
                    0.,
                ],
                color: [
                    self.background_color.r as f32,
                    self.background_color.g as f32,
                    self.background_color.b as f32,
                ],
                id: self.id
            });
        }
        let number_of_triangles = vertices.len() - 2;
        let mut indices = Vec::with_capacity(number_of_triangles * 3);
        for i in 0..number_of_triangles {
            indices.push(indice_offset + 0);
            indices.push(indice_offset + (i + 1) as u16);
            indices.push(indice_offset + (i + 2) as u16);
        }

        (vertices, indices)
    }

    fn set_position_from_pixels(&mut self, x: f64, y: f64) {
        self.position = GUIPosition::from_pixels(x, y);
    }

    fn set_position_from_lengths(&mut self, x: GUILength, y: GUILength) {
        self.position = GUIPosition::from_lengths(x, y);
    }

    fn set_position_from_position(&mut self, position: GUIPosition) {
        self.position = position;
    }
}

impl Family for GUIButton {}

impl GUIButton {
    pub fn set_radius_from_pixels(&mut self, pixels: f64) {
        self.radius = GUILength::from_pixels(pixels);
    }

    pub fn set_radius_from_length(&mut self, length: GUILength) {
        self.radius = length;
    }
}
