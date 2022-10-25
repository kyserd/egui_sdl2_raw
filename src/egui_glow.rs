use crate::egui_sdl::{self, Platform};
use egui::RawInput;
use glow::HasContext;

struct BlendState {
    mode_rgb: u32,
    mode_a: u32,
    mode_src_rgb: u32,
    mode_src_a: u32,
    mode_dst_rgb: u32,
    mode_dst_a: u32,
}

pub struct EguiGlow {
    pub egui_ctx: egui::Context,
    pub painter: egui_glow::Painter,

    sdl: Option<egui_sdl::Platform>,
    shapes: Vec<egui::epaint::ClippedShape>,
    textures_delta: egui::TexturesDelta,
    blend_state: BlendState,
}

impl EguiGlow {
    pub fn new(gl: std::sync::Arc<glow::Context>, screen_size: (u32, u32)) -> Self {
        let painter = egui_glow::Painter::new(gl, None, "").unwrap();

        let mode_rgb = unsafe { painter.gl().get_parameter_i32(glow::BLEND_EQUATION_RGB) as u32 };
        let mode_a = unsafe { painter.gl().get_parameter_i32(glow::BLEND_EQUATION_ALPHA) as u32 };
        let mode_src_rgb = unsafe { painter.gl().get_parameter_i32(glow::BLEND_SRC_RGB) as u32 };
        let mode_src_a = unsafe { painter.gl().get_parameter_i32(glow::BLEND_SRC_ALPHA) as u32 };
        let mode_dst_rgb = unsafe { painter.gl().get_parameter_i32(glow::BLEND_DST_RGB) as u32 };
        let mode_dst_a = unsafe { painter.gl().get_parameter_i32(glow::BLEND_DST_ALPHA) as u32 };

        Self {
            egui_ctx: Default::default(),
            painter,
            sdl: None,
            shapes: Default::default(),
            textures_delta: Default::default(),
            blend_state: BlendState {
                mode_rgb,
                mode_a,
                mode_src_rgb,
                mode_src_a,
                mode_dst_rgb,
                mode_dst_a,
            },
        }
    }

    pub fn run(&mut self, run_ui: impl FnMut(&egui::Context)) -> std::time::Duration {
        let input = match self.sdl.take() {
            Some(input) => input.raw_input,
            None => Default::default(),
        };
        // Get input here
        let egui::FullOutput {
            platform_output: _,
            repaint_after,
            textures_delta,
            shapes,
        } = self.egui_ctx.run(input, run_ui);

        self.shapes = shapes;
        self.textures_delta.append(textures_delta);
        repaint_after
    }

    pub fn paint(&mut self, dimensions: [u32; 2]) {
        let shapes = std::mem::take(&mut self.shapes);
        let mut textures_delta = std::mem::take(&mut self.textures_delta);

        for (id, image_delta) in textures_delta.set {
            self.painter.set_texture(id, &image_delta);
        }

        let clipped_primitives = self.egui_ctx.tessellate(shapes);
        self.painter.paint_primitives(
            dimensions,
            self.egui_ctx.pixels_per_point(),
            &clipped_primitives,
        );

        for id in textures_delta.free.drain(..) {
            self.painter.free_texture(id);
        }

        // Restore the proper blend functions after rendering
        unsafe {
            self.painter.gl().blend_equation_separate(
                self.blend_state.mode_rgb as u32,
                self.blend_state.mode_a as u32,
            );
            self.painter.gl().blend_func_separate(
                self.blend_state.mode_src_rgb,
                self.blend_state.mode_dst_rgb,
                self.blend_state.mode_src_a,
                self.blend_state.mode_dst_a,
            );
        };
    }

    pub fn destroy(&mut self) {
        self.painter.destroy()
    }

    pub fn set_raw_input(&mut self, input: Option<Platform>) {
        self.sdl = input;
    }
}
