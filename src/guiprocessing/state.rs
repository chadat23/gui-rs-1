use std::iter;

use wgpu::util::DeviceExt;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::window::Window;

use crate::guiproperties::guiposition::GUISize;
use crate::guiresources::GUIResources;
use crate::guiwidgets::{GUIButton, GUIWindow};

use crate::guiprocessing::vertices::Vertex;
// use crate::guiprocessing::vertices::{Vertex, INDICES, VERTICES};

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    render_pipeline: wgpu::RenderPipeline,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,

    challenge_vertex_buffer: wgpu::Buffer,
    challenge_index_buffer: wgpu::Buffer,
    num_challenge_indices: u32,
    use_complex: bool,

    pub guiwindow: GUIWindow,
}

impl State {
    pub async fn new(window: &Window, guiwindow: GUIWindow, guiresources: GUIResources) -> Self {
        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(guiresources.backend());
        // The surface is part of the window that's drawn to.
        let surface = unsafe { instance.create_surface(window) };
        // The adapter is the handle to the actual graphics card.
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: guiresources.power_preference(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                // Some(&std::path::Path::new("trace")), // Trace path
                None,
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: guiwindow.size.get_width_in_pixels(),
            height: guiwindow.size.get_height_in_pixels(),
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
        });

        // let mut button = &guiwindow.buttons.unwrap();
        let mut button = match guiwindow.buttons {
            Some(ref button) => GUIButton {
                text: button.text.clone(),
                size: button.size.clone(),
                radius: button.radius.clone(),
                background_color: button.background_color.clone(),
                polygon: button.polygon.clone(),
            },
            None => todo!(),
        };

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            // contents: bytemuck::cast_slice(VERTICES),
            contents: bytemuck::cast_slice(&button.vertices()[..]),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            // contents: bytemuck::cast_slice(INDICES),
            contents: bytemuck::cast_slice(&button.indices()[..]),
            usage: wgpu::BufferUsages::INDEX,
        });
        // let num_indices = INDICES.len() as u32;
        let num_indices = button.indices().len() as u32;

        let num_vertices = 16;
        let angle = std::f32::consts::PI * 2.0 / num_vertices as f32;
        let challenge_verts = (0..num_vertices)
            .map(|i| {
                let theta = angle * i as f32;
                Vertex {
                    position: [0.5 * theta.cos(), -0.5 * theta.sin(), 0.0],
                    color: [(1.0 + theta.cos()) / 2.0, (1.0 + theta.sin()) / 2.0, 1.0],
                }
            })
            .collect::<Vec<_>>();

        let num_triangles = num_vertices - 2;
        let challenge_indices = (1u16..num_triangles + 1)
            .into_iter()
            .flat_map(|i| vec![i + 1, i, 0])
            .collect::<Vec<_>>();
        let num_challenge_indices = challenge_indices.len() as u32;

        let challenge_vertex_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Challenge Vertex Buffer"),
                contents: bytemuck::cast_slice(&challenge_verts),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let challenge_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Challenge Index Buffer"),
            contents: bytemuck::cast_slice(&challenge_indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let use_complex = false;

        Self {
            surface,
            device,
            queue,
            config,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            challenge_vertex_buffer,
            challenge_index_buffer,
            num_challenge_indices,
            use_complex,
            guiwindow,
        }
    }
    pub fn resize(&mut self, new_size: GUISize) {
        self.guiwindow.size = GUISize {
            width: new_size.width,
            height: new_size.height,
        };
        self.config.width = new_size.get_width_in_pixels();
        self.config.height = new_size.get_height_in_pixels();
        self.surface.configure(&self.device, &self.config);
    }

    #[allow(unused_variables)]
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        ..
                    },
                ..
            } => {
                self.use_complex = *state == ElementState::Pressed;
                true
            }
            _ => false,
        }
    }

    #[warn(dead_code)]
    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: self.guiwindow.background_color.r,
                            g: self.guiwindow.background_color.g,
                            b: self.guiwindow.background_color.b,
                            a: self.guiwindow.background_color.a,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);

            let data = if self.use_complex {
                (
                    &self.challenge_vertex_buffer,
                    &self.challenge_index_buffer,
                    self.num_challenge_indices,
                )
            } else {
                (&self.vertex_buffer, &self.index_buffer, self.num_indices)
            };
            render_pass.set_vertex_buffer(0, data.0.slice(..));
            render_pass.set_index_buffer(data.1.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..data.2, 0, 0..1);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
