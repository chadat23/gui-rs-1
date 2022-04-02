use ::std::time::SystemTime;

use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use crate::guiproperties::guiposition::GUISize;
use crate::guiresources::GUIResources;
use crate::guiwidgets::GUIWindow;

mod state;
pub mod vertices;
pub mod window_building_utils;

use state::State;

/// The main funciton that executes everthing.
pub fn run(mut guiwindow: GUIWindow, guiresources: GUIResources) {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    guiwindow.logical_scale = Some(window.scale_factor());
    let guiwindow = guiwindow;
    let window = window_building_utils::set_window_properties(window, &guiwindow);

    // State::new uses async code, so we're going to wait for it to finish
    let mut state: State = pollster::block_on(State::new(&window, guiwindow, guiresources));

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            state.resize(GUISize::from_physical_pixels(
                                physical_size.width as f64,
                                physical_size.height as f64,
                                &state.guiwindow.logical_scale.unwrap(),
                            ));
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so w have to dereference it twice
                            state.resize(GUISize::from_physical_pixels(
                                new_inner_size.width as f64,
                                new_inner_size.height as f64,
                                &state.guiwindow.logical_scale.unwrap(),
                            ));
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                // state.update();
                match state.render() {
                    Ok(_) => {
                        // println!("good!");
                        // println!("{:?}", SystemTime::now());
                    }
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => {
                        state.guiwindow.size;
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::RedrawEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}
