use winit::dpi::PhysicalSize;
use winit::window::Window;

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
