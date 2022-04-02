use super::guiposition::GUIPosition;

#[derive(Clone)]
pub struct GUIPolygon {
    pub vertices: Vec<GUIPosition>,
    pub indices: Vec<u16>,
}
