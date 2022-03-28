use crate::guiprocessing::vertices::Vertex;
use crate::guiproperties::guicolor::GUIColor;
use crate::guiproperties::guipolygon::GUIPolygon;
use crate::guiproperties::guiposition::{GUILength, GUISize};
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
    pub polygon: Option<GUIPolygon>,
}

impl Default for GUIButton {
    // Returns a button with all of the default values.
    fn default() -> GUIButton {
        GUIButton {
            text: "Button",
            size: GUISize {
                width: 20.,
                height: 10.,
            },
            radius: GUILength { length: 3. },
            background_color: GUIColor {
                r: 0.7,
                g: 0.1,
                b: 0.4,
                a: 1.0,
            },
            polygon: None,
        }
    }
}

impl GUIButton {
    pub fn vertices(&mut self) -> Vec<Vertex> {
        let polygon = match &self.polygon {
            Some(polygon) => polygon,
            None => {
                &self.make_polygon();
                match &self.polygon {
                    Some(polygon) => polygon,
                    None => {
                        todo!()
                    }
                }
                // &self.polygon.unwrap()
            }
        };

        let vertices: Vec<Vertex> = polygon
            .vertices
            .iter()
            .map(|position| Vertex {
                position: [position.x as f32 / 50., position.y as f32 / 50., 0.],
                color: [
                    self.background_color.r as f32,
                    self.background_color.g as f32,
                    self.background_color.b as f32,
                ],
            })
            .collect();

        vertices
    }

    pub fn indices(&mut self) -> Vec<u16> {
        let polygon = match &self.polygon {
            Some(polygon) => polygon,
            None => {
                &self.make_polygon();
                match &self.polygon {
                    Some(polygon) => polygon,
                    None => {
                        todo!()
                    }
                }
                // &self.polygon.unwrap()
            }
        };

        polygon.indices.iter().map(|i| *i).collect()
    }

    pub fn make_polygon(&mut self) {
        let fascets = 4;

        let top_left_radius = arcs::make_top_left_arc(self.radius.length, fascets);
        let mut top_left_radius =
            widget_utils::transpose(top_left_radius, &self.radius, &self.radius.negative());

        let top_right_radius = arcs::make_top_right_arc(self.radius.length, fascets);
        let top_right_radius = widget_utils::transpose(
            top_right_radius,
            &GUILength::zero(),
            &self.radius.negative(),
        );

        let bottom_left_radius = arcs::make_bottom_left_arc(self.radius.length, fascets);
        let bottom_left_radius =
            widget_utils::transpose(bottom_left_radius, &self.radius, &GUILength::zero());

        let bottom_right_radius = arcs::make_bottom_right_arc(self.radius.length, fascets);
        let bottom_right_radius =
            widget_utils::transpose(bottom_right_radius, &self.radius, &GUILength::zero());

        top_left_radius.extend(top_right_radius);
        top_left_radius.extend(bottom_right_radius);
        top_left_radius.extend(bottom_left_radius);

        let number_of_triangles = (top_left_radius.len() - 2) * 3;

        let mut indices: Vec<u16> = Vec::with_capacity(number_of_triangles);
        for triangle_index in 0..number_of_triangles {
            indices.push(0);
            indices.push((triangle_index + 1) as u16);
            indices.push((triangle_index + 2) as u16);
        }

        self.polygon = Some(GUIPolygon {
            // vertices: &top_left_radius[..],
            // indices: &indices[..],
            vertices: top_left_radius,
            indices: indices,
        })
    }
}
