mod geometry;
mod utils;
mod open_gl;
mod ez_files;
mod generator;

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

use image::{
    ImageBuffer,
    Rgba,
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
        start: Point::new(0.5, -0.5),
        anchor1: Point::new(-0.5, -0.5),
        anchor2: Point::new(0.5, 0.5),
        end: Point::new(-0.5, 0.5),
    };

    let les_tas_de_bez = generator::drawings();

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

                let r = renderer.get_or_insert_with(|| Renderer::new(&gl_display));

                r.set_res(200);
                r.set_beziers(&[la_bez]);

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
//                     let mut handles = Vec::new();
                    let renderer = renderer.as_mut().unwrap();
//                     for (i, les_bez) in les_tas_de_bez.iter().enumerate() {
//                         renderer.set_beziers([la_bez].as_slice());
                        renderer.draw();
//                         let picture_data = renderer.get_data();
//                         handles.push(std::thread::spawn(move || {
//                             let buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(4096, 4096, picture_data)
//                                 .expect("ImageBuffer de con");
//                             buffer.save(format!("target/pictures/omg{i:04}.png"))
//                                 .unwrap_or_else(|e| println!("{:?}", e));
//                         }));
//                     }

//                     for h in handles {
//                         h.join()
//                             .unwrap_or_else(|_| println!("Handle joined badly"));
//                     }

                    window.request_redraw();

                    gl_surface.swap_buffers(gl_context).unwrap();
//                     control_flow.set_exit();
                }
            }
            _ => (),
        }
    })
}
