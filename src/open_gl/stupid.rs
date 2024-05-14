use std::ffi::{CStr, CString};
use crate::ez_files;
use crate::geometry::{
    bezier::Bezier,
};

use gl::types::*;

use winit::{
    window::{Window, WindowBuilder},
    event_loop::EventLoop,
};

use glutin_winit::DisplayBuilder;

use raw_window_handle::HasRawWindowHandle;

use glutin::{
    config::{Config, ConfigTemplateBuilder},
    context::{ContextApi, ContextAttributesBuilder, NotCurrentContext, Version},
    display::GetGlDisplay,
    prelude::*,
};

pub struct Graphics {
    pub window: Option<Window>,
    pub gl_config: Config,
    pub not_current_gl_context: Option<NotCurrentContext>,
}

impl Graphics {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        // Only windows requires the window to be present before creating the display.
        // Other platforms don't really need one.
        //
        // XXX if you don't care about running on android or so you can safely remove
        // this condition and always pass the window builder.
        let window_builder = if cfg!(windows) {
            Some(WindowBuilder::new().with_transparent(true))
        } else {
            None
        };

        // The template will match only the configurations supporting rendering
        // to windows.
        //
        // XXX We force transparency only on macOS, given that EGL on X11 doesn't
        // have it, but we still want to show window. The macOS situation is like
        // that, because we can query only one config at a time on it, but all
        // normal platforms will return multiple configs, so we can find the config
        // with transparency ourselves inside the `reduce`.
        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_transparency(cfg!(target_os = "macos"));

        let display_builder = DisplayBuilder::new().with_window_builder(window_builder);

        let (window, gl_config) = display_builder
            .build(event_loop, template, |configs| {
                let configs: Vec<_> = configs.collect();

                /*
                #[derive(Debug)]
                #[allow(unused)]
                struct ConfigInfo {
                    color_buffer_type: Option<ColorBufferType>,
                    float_pixels: bool,
                    alpha_size: u8,
                    depth_size: u8,
                    stencil_size: u8,
                    num_samples: u8,
                    srgb_capable: bool,
                    supports_transparency: Option<bool>,
                    hardware_accelerated: bool,
                    config_surface_types: ConfigSurfaceTypes,
                    api: Api,
                }
                println!(
                    "{:#?}",
                    configs
                        .iter()
                        .map(|c| ConfigInfo {
                            color_buffer_type: c.color_buffer_type(),
                            float_pixels: c.float_pixels(),
                            alpha_size: c.alpha_size(),
                            depth_size: c.depth_size(),
                            stencil_size: c.stencil_size(),
                            num_samples: c.num_samples(),
                            srgb_capable: c.srgb_capable(),
                            supports_transparency: c.supports_transparency(),
                            hardware_accelerated: c.hardware_accelerated(),
                            config_surface_types: c.config_surface_types(),
                            api: c.api()
                        })
                        .collect::<Vec<_>>()
                );
                */
                println!("Number of configs {}", configs.iter().count());
                configs
                    .into_iter()
                    .reduce(|accum, config| {
                        let hardware_check = config.hardware_accelerated() && !accum.hardware_accelerated();

                        if hardware_check || (config.srgb_capable() && !accum.srgb_capable()) {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        println!("Config > hardware : {} | srgb : {}", gl_config.hardware_accelerated(), gl_config.srgb_capable());

        let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle());

        // XXX The display could be obtained from the any object created by it, so we
        // can query it from the config.
        let gl_display = gl_config.display();

        // The context creation part. It can be created before surface and that's how
        // it's expected in multithreaded + multiwindow operation mode, since you
        // can send NotCurrentContext, but not Surface.
        let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

        // Since glutin by default tries to create OpenGL core context, which may not be
        // present we should try gles.
        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(raw_window_handle);

        // There are also some old devices that support neither modern OpenGL nor GLES.
        // To support these we can try and create a 2.1 context.
        let legacy_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
            .build(raw_window_handle);

        let not_current_gl_context = Some(unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap_or_else(|_| {
                    gl_display
                        .create_context(&gl_config, &fallback_context_attributes)
                        .unwrap_or_else(|_| {
                            gl_display
                                .create_context(&gl_config, &legacy_context_attributes)
                                .expect("failed to create context")
                        })
                })
        });

        Self {
            window,
            gl_config,
            not_current_gl_context,
        }
    }
}

pub struct Renderer {
    program: gl::types::GLuint,
    buf: GLuint,
    tex: GLuint,
    vao: gl::types::GLuint,
    vertices_vbo: gl::types::GLuint,
    instances_vbo: gl::types::GLuint,
    beziers_count: GLsizeiptr,
    res: GLsizeiptr,
}

impl Renderer {
    pub fn new<D: GlDisplay>(gl_display: &D) -> Self {
        println!("Le renderer ouais ouais");
        unsafe {
            gl::load_with(|symbol| {
                let symbol = CString::new(symbol).unwrap();
                gl_display.get_proc_address(symbol.as_c_str()).cast()
            });

            if let Some(renderer) = get_gl_string(gl::RENDERER) {
                println!("Running on {}", renderer.to_string_lossy());
            }
            if let Some(version) = get_gl_string(gl::VERSION) {
                println!("OpenGL Version {}", version.to_string_lossy());
            }

            if let Some(shaders_version) = get_gl_string(gl::SHADING_LANGUAGE_VERSION) {
                println!("Shaders version on {}", shaders_version.to_string_lossy());
            }

            let mut code = ez_files::read_to_string("shaders/bezier.vs.glsl").unwrap();
            let vertex_shader = create_shader(gl::VERTEX_SHADER, code.as_bytes());
            code = ez_files::read_to_string("shaders/bezier.fs.glsl").unwrap();
            let fragment_shader = create_shader(gl::FRAGMENT_SHADER, code.as_bytes());

            let program = gl::CreateProgram();

            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);

            gl::LinkProgram(program);

            gl::UseProgram(program);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            let mut vao = std::mem::zeroed();
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);


            let vertex_truc_attrib = 0;
            let bezier_point1_attrib = 1;
            let bezier_point2_attrib = 2;
            let bezier_point3_attrib = 3;
            let bezier_point4_attrib = 4;

            let mut vertices_vbo = std::mem::zeroed();
            gl::GenBuffers(1, &mut vertices_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertices_vbo);
            gl::EnableVertexAttribArray(vertex_truc_attrib);
            gl::VertexAttribPointer(
                vertex_truc_attrib,
                1,
                gl::INT,
                0,
                std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );

            let mut instances_vbo = std::mem::zeroed();
            gl::GenBuffers(1, &mut instances_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, instances_vbo);
            gl::EnableVertexAttribArray(bezier_point1_attrib);
            gl::VertexAttribPointer(
                bezier_point1_attrib,
                2,
                gl::FLOAT,
                0,
                8 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl::VertexAttribDivisor(bezier_point1_attrib, 1);
            gl::EnableVertexAttribArray(bezier_point2_attrib);
            gl::VertexAttribPointer(
                bezier_point2_attrib,
                2,
                gl::FLOAT,
                0,
                8 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (2 * std::mem::size_of::<f32>()) as *const () as *const _,
            );
            gl::VertexAttribDivisor(bezier_point1_attrib, 2);
            gl::EnableVertexAttribArray(bezier_point3_attrib);
            gl::VertexAttribPointer(
                bezier_point3_attrib,
                2,
                gl::FLOAT,
                0,
                8 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (4 * std::mem::size_of::<f32>()) as *const () as *const _,
            );
            gl::VertexAttribDivisor(bezier_point1_attrib, 3);
            gl::EnableVertexAttribArray(bezier_point4_attrib);
            gl::VertexAttribPointer(
                bezier_point4_attrib,
                2,
                gl::FLOAT,
                0,
                8 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (6 * std::mem::size_of::<f32>()) as *const () as *const _,
            );
            gl::VertexAttribDivisor(bezier_point1_attrib, 4);

            let mut buf = 0;
//             gl::GenFramebuffers(1, &mut buf);
//             gl::BindFramebuffer(gl::FRAMEBUFFER, buf);

            let mut tex = 0;
//             gl::GenTextures(1, &mut tex);
//             gl::BindTexture(gl::TEXTURE_2D, tex);
//             gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as GLint, 4096, 4096, 0, gl::RGBA, gl::UNSIGNED_BYTE, std::ptr::null());
//             gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
//             gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
//             gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, tex, 0);

//             if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
//                 panic!("C koi se fmrame buffer de m***e ?!?!");
//             }
//             gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::Enable(gl::BLEND);

            Self { program, buf, tex, vao, vertices_vbo, instances_vbo, beziers_count: 0, res: 0}
        }
    }

    pub fn set_res(&mut self, res: GLsizeiptr) {
        println!("Rezo");
        self.res = res as GLsizeiptr;

        unsafe {
            gl::NamedBufferData(
                self.vertices_vbo,
                self.res,
                vec![0i32; res as usize].as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let name = b"uTotalVerticesCount\0".as_ptr() as *const i8;
            let addr = gl::GetUniformLocation(self.program, name);
            println!("C laddressss :: {addr}");
            gl::Uniform1i(addr, res as GLint);
        }
    }

    pub fn set_beziers(&mut self, beziers: &[Bezier]) {
        self.beziers_count = beziers.len() as isize;
        println!("Bezi {}", self.beziers_count);
        println!("{}", std::mem::size_of::<Bezier>());

        unsafe {
            gl::NamedBufferData(
                self.instances_vbo,
                (beziers.len()* std::mem::size_of::<Bezier>()) as GLsizeiptr,
                beziers.as_ptr() as *const () as *const _,
                gl::DYNAMIC_DRAW,
            );
        }
    }

    pub fn draw(&self) {
        unsafe {
//             gl::Viewport(0, 0, 4096, 4096);
//             gl::BindFramebuffer(gl::FRAMEBUFFER, self.buf);
            gl::UseProgram(self.program);

            gl::BindVertexArray(self.vao);

            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArraysInstanced(gl::TRIANGLE_STRIP, 0, self.res as GLint, self.beziers_count as GLint);
        }
    }

    pub fn get_data(&self) -> Vec<u8> {
        let mut data = Vec::<u8>::with_capacity(4096*4096*4);

        unsafe {
            data.set_len(4096*4096*4);

            gl::GetTextureImage(
                self.tex, 0, gl::RGBA, gl::UNSIGNED_BYTE,
                4096*4096*4, data.as_mut_ptr() as *mut _);
        }

        data
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
            gl::DeleteBuffers(1, &self.instances_vbo);
            gl::DeleteBuffers(1, &self.vertices_vbo);
            gl::DeleteTextures(1, &self.tex);
            gl::DeleteFramebuffers(1, &self.buf);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

unsafe fn create_shader(shader: gl::types::GLenum, source: &[u8]) -> gl::types::GLuint {
    let shader = gl::CreateShader(shader);
    gl::ShaderSource(
        shader,
        1,
        [source.as_ptr().cast()].as_ptr(),
        [source.len() as gl::types::GLint].as_ptr(),
    );
    gl::CompileShader(shader);

    let mut is_compiled = 0;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut is_compiled);
    if is_compiled == gl::FALSE.into() {
        println!("Shader didn't compile !");

        let mut max_length = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut max_length);

        let mut error_log = vec![0u8; max_length as usize];
        gl::GetShaderInfoLog(shader, max_length, &mut max_length, error_log.as_mut_ptr() as *mut () as *mut _);
        let error_log = String::from_utf8_unchecked(error_log);

        gl::DeleteShader(shader);
        panic!("{}", error_log);

    }
    shader
}

fn get_gl_string(variant: gl::types::GLenum) -> Option<&'static CStr> {
    unsafe {
        let s = gl::GetString(variant);
        (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
    }
}
