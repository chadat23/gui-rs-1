use winit::dpi::PhysicalSize;
use winit::window::Window;

use super::vertices::Vertex;
use crate::guiproperties::guitraits::*;
use crate::guiwidgets::GUIWindow;

pub fn set_window_properties(window: Window, guiwindow: &GUIWindow) -> Window {
    window.set_title(guiwindow.title);
    window.set_inner_size(PhysicalSize::new(
        guiwindow.size.width,
        guiwindow.size.height,
    ));
    window.set_min_inner_size(Some(PhysicalSize::new(
        guiwindow.min_size.width,
        guiwindow.min_size.height,
    )));
    window.set_max_inner_size(Some(PhysicalSize::new(
        guiwindow.max_size.width,
        guiwindow.max_size.height,
    )));
    window
}

pub fn we_will_see(children: &Option<Vec<Box<dyn AreaFamily>>>) -> (Vec<Vertex>, Vec<u16>) {
    let mut all_vertices: Vec<Vertex> = Vec::new();
    let mut all_indices: Vec<u16> = Vec::new();
    match children {
        Some(children) => {
            for child in children.iter() {
                let child = child;
                let (vertices, indices) = child.get_vertices_and_indices();
                all_vertices.extend(vertices);
                all_indices.extend(indices);
                if child.children_len() > 0 {
                    let (vertices, indices) = child.get_vertices_and_indices();
                    all_vertices.extend(vertices);
                    all_indices.extend(indices);
                }
            }
        }
        None => {}
    };
    (all_vertices, all_indices)
}
