use super::guiposition::GUIPosition;

// #[derive(Clone, Copy)]
#[derive(Clone)]
pub struct GUIPolygon {
    // pub vertices: &'a [GUIPosition],
    // pub indices: &'a [u16],
    pub vertices: Vec<GUIPosition>,
    pub indices: Vec<u16>,
}
