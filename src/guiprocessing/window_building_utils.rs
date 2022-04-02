use winit::dpi::PhysicalSize;
use winit::window::Window;

use super::vertices::Vertex;
use crate::guiproperties::guiposition::GUISize;
use crate::guiproperties::guitraits::*;
use crate::guiwidgets::GUIWindow;

pub fn set_window_properties(window: Window, guiwindow: &GUIWindow) -> Window {
    window.set_title(guiwindow.title);
    // window.set_inner_size(PhysicalSize::new(width: 8, height: 8));
    window.set_inner_size(PhysicalSize::new(
        guiwindow
            .size
            .width
            .get_physical_length(&guiwindow.logical_scale.unwrap()) as u32,
        guiwindow
            .size
            .height
            .get_physical_length(&guiwindow.logical_scale.unwrap()) as u32,
    ));
    window.set_min_inner_size(Some(PhysicalSize::new(
        guiwindow
            .min_size
            .width
            .get_physical_length(&guiwindow.logical_scale.unwrap()) as u32,
        guiwindow
            .min_size
            .height
            .get_physical_length(&guiwindow.logical_scale.unwrap()) as u32,
    )));
    window.set_max_inner_size(Some(PhysicalSize::new(
        guiwindow
            .max_size
            .width
            .get_physical_length(&guiwindow.logical_scale.unwrap()) as u32,
        guiwindow
            .max_size
            .height
            .get_physical_length(&guiwindow.logical_scale.unwrap()) as u32,
    )));
    window
}

pub fn make_vertices_and_indices(
    parent_size: &GUISize,
    children: &Option<Vec<Box<dyn Family>>>,
    scale: &f64,
) -> (Vec<Vertex>, Vec<u16>) {
    let mut all_vertices: Vec<Vertex> = Vec::new();
    let mut all_indices: Vec<u16> = Vec::new();
    match children {
        Some(children) => {
            for child in children.iter() {
                let child = child;
                let (vertices, indices) = child.get_vertices_and_indices(&parent_size, &scale);
                all_vertices.extend(vertices);
                all_indices.extend(indices);
                if child.children_len() > 0 {
                    let (vertices, indices) = child.get_vertices_and_indices(&parent_size, &scale);
                    all_vertices.extend(vertices);
                    all_indices.extend(indices);
                }
            }
        }
        None => {}
    };
    (all_vertices, all_indices)
}
