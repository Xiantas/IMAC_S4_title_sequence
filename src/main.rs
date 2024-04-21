mod geometry;
mod utils;
mod open_gl;
mod ez_files;

use std::num::NonZeroU32;

use crate::open_gl::stupid::*;

use winit::{
    event_loop::EventLoopBuilder,
    event::{Event, WindowEvent},
    window::WindowBuilder,
};

use glutin::{
    display::{GlDisplay, GetGlDisplay},
    surface::SwapInterval,
    prelude::*,
};

use crate::geometry::{
    bezier::Bezier,
    Point,
};

use glutin_winit::GlWindow;

fn main() {
    let event_loop = EventLoopBuilder::new().build();

    let mut graphics = Graphics::new(&event_loop);
    let gl_display = graphics.gl_config.display();

    let mut state = None;
    let mut renderer = None;

    let la_bez = Bezier {
        start: Point::new(0.5, 0.5),
        anchor1: Point::new(-0.5, 0.5),
        anchor2: Point::new(0.5, -0.5),
        end: Point::new(-0.5, -0.5),
        start_size: 0.0,
        mid_size: 0.3,
        end_size: 0.0,
        color: image::Rgb([50, 205, 200]),
    };

    event_loop.run(move |event, window_target, control_flow| {
        control_flow.set_wait();

        match event {
            Event::Resumed => {
                let window = graphics.window.take().unwrap_or_else(|| {
                    let window_builder = WindowBuilder::new().with_transparent(true);
                    glutin_winit::finalize_window(
                        window_target,
                        window_builder,
                        &graphics.gl_config,
                    )
                    .unwrap()
                });

                let attrs = window.build_surface_attributes(<_>::default());
                let gl_surface = unsafe {
                    graphics
                        .gl_config
                        .display()
                        .create_window_surface(&graphics.gl_config, &attrs)
                        .unwrap()
                };

                let gl_context = graphics
                    .not_current_gl_context
                    .take()
                    .unwrap()
                    .make_current(&gl_surface)
                    .unwrap();

                let mut renderer = renderer.get_or_insert_with(|| Renderer::new(&gl_display));

                renderer.set_res(200);
                renderer.set_beziers(vec![la_bez].as_slice());

                if let Err(res) = gl_surface
                    .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap())) {
                    eprintln!("Error setting vsync: {res:?}");
                }

                assert!(state.replace((gl_context, gl_surface, window)).is_none());
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    if size.width != 0 && size.height != 0 {
                        if let Some((gl_context, gl_surface, _)) = &state {
                            gl_surface.resize(
                                gl_context,
                                NonZeroU32::new(size.width).unwrap(),
                                NonZeroU32::new(size.height).unwrap(),
                            );
                            let renderer = renderer.as_ref().unwrap();
                            renderer.resize(size.width as i32, size.height as i32);
                        }
                    }
                }
                WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                }
                _ => (),
            }
            Event::RedrawEventsCleared => {
                if let Some((gl_context, gl_surface, window)) = &state {
                    let renderer = renderer.as_ref().unwrap();
                    renderer.draw();
                    window.request_redraw();

                    gl_surface.swap_buffers(gl_context).unwrap();
                }
            }
            _ => (),
        }
    })
}
