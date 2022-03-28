use crate::guiproperties::guiposition::{GUILength, GUIPosition};

pub mod arcs {
    use crate::guiproperties::guiposition::GUIPosition;

    const PI: f64 = 3.141592653589793238;

    /// Creates a set of vertices to approximate the top right quadrent of a circle.
    /// The center of the arc has the coordinates 0, 0.
    /// fascets is the number of fascets in the arc, so one quater of the
    /// number of fascets that's be in a similarly detialed circle.
    /// The bottom right most vertice is the first with them in counter clockwise order.
    pub fn make_top_right_arc(radius: f64, fascets: u8) -> Vec<GUIPosition> {
        let vertices = (fascets + 1) as usize;
        let mut positions = Vec::with_capacity(vertices);

        for i in 0..vertices {
            let angle = 2. * PI * i as f64 / (fascets * 4) as f64;

            positions.push(GUIPosition {
                x: radius * angle.cos(),
                y: radius * angle.sin(),
            });
        }

        positions
    }

    /// Creates a set of vertices to approximate the top left quadrent of a circle.
    /// The center of the arc has the coordinates 0, 0.
    /// fascets is the number of fascets in the arc, so one quater of the
    /// number of fascets that's be in a similarly detialed circle.
    /// The top right most vertice is the first with them in counter clockwise order.
    pub fn make_top_left_arc(radius: f64, fascets: u8) -> Vec<GUIPosition> {
        let mut top_right = make_top_right_arc(radius, fascets);
        top_right.reverse();
        top_right
            .iter()
            .map(|position| GUIPosition {
                x: position.x * -1.,
                y: position.y,
            })
            .collect()
    }

    /// Creates a set of vertices to approximate the bottom right quadrent of a circle.
    /// The center of the arc has the coordinates 0, 0.
    /// fascets is the number of fascets in the arc, so one quater of the
    /// number of fascets that's be in a similarly detialed circle.
    /// The bottom left most vertice is the first with them in counter clockwise order.
    pub fn make_bottom_right_arc(radius: f64, fascets: u8) -> Vec<GUIPosition> {
        let mut top_right = make_top_right_arc(radius, fascets);
        top_right.reverse();
        top_right
            .iter()
            .map(|position| GUIPosition {
                x: position.x,
                y: position.y * -1.,
            })
            .collect()
    }

    /// Creates a set of vertices to approximate the bottom left quadrent of a circle.
    /// The center of the arc has the coordinates 0, 0.
    /// fascets is the number of fascets in the arc, so one quater of the
    /// number of fascets that's be in a similarly detialed circle.
    /// The top left most vertice is the first with them in counter clockwise order.
    pub fn make_bottom_left_arc(radius: f64, fascets: u8) -> Vec<GUIPosition> {
        let top_right = make_top_right_arc(radius, fascets);
        top_right
            .iter()
            .map(|position| GUIPosition {
                x: position.x * -1.,
                y: position.y * -1.,
            })
            .collect()
    }
}

pub fn transpose(
    mut vertices: Vec<GUIPosition>,
    dx: &GUILength,
    dy: &GUILength,
) -> Vec<GUIPosition> {
    for vertice in vertices.iter_mut() {
        vertice.x += dx.length;
        vertice.y += dy.length;
    }

    vertices
}

#[cfg(test)]
mod tests {
    use crate::guiproperties::guiposition::GUIPosition;

    use crate::guiwidgets::widget_utils::arcs::*;

    #[test]
    fn make_top_right_arc_good() {
        let actual = make_top_right_arc(5., 4);
        let expected = Vec::from([
            GUIPosition { x: 5., y: 0. },
            GUIPosition {
                x: 4.6193976625564339,
                y: 1.913417161825449,
            },
            GUIPosition {
                x: 3.5355339059327378,
                y: 3.5355339059327373,
            },
            GUIPosition {
                x: 1.9134171618254492,
                y: 4.6193976625564339,
            },
            GUIPosition { x: 0., y: 5. },
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn make_top_left_arc_good() {
        let actual = make_top_left_arc(5., 4);
        let expected = Vec::from([
            GUIPosition { x: 0., y: 5. },
            GUIPosition {
                x: -1.9134171618254492,
                y: 4.6193976625564339,
            },
            GUIPosition {
                x: -3.5355339059327378,
                y: 3.5355339059327373,
            },
            GUIPosition {
                x: -4.6193976625564339,
                y: 1.913417161825449,
            },
            GUIPosition { x: -5., y: 0. },
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn make_bottom_right_arc_good() {
        let actual = make_bottom_right_arc(5., 4);
        let expected = Vec::from([
            GUIPosition { x: 0., y: -5. },
            GUIPosition {
                x: 1.9134171618254492,
                y: -4.6193976625564339,
            },
            GUIPosition {
                x: 3.5355339059327378,
                y: -3.5355339059327373,
            },
            GUIPosition {
                x: 4.6193976625564339,
                y: -1.913417161825449,
            },
            GUIPosition { x: 5., y: 0. },
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn make_bottom_left_arc_good() {
        let actual = make_bottom_left_arc(5., 4);
        let expected = Vec::from([
            GUIPosition { x: -5., y: 0. },
            GUIPosition {
                x: -4.6193976625564339,
                y: -1.913417161825449,
            },
            GUIPosition {
                x: -3.5355339059327378,
                y: -3.5355339059327373,
            },
            GUIPosition {
                x: -1.9134171618254492,
                y: -4.6193976625564339,
            },
            GUIPosition { x: 0., y: -5. },
        ]);
        assert_eq!(actual, expected);
    }
}
