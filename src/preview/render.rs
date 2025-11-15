use super::PreviewContent;
use macroquad::prelude::*;

pub struct PreviewLayout {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Default for PreviewLayout {
    fn default() -> Self {
        Self {
            w: 350.0,
            h: 300.0,
            x: screen_width() - 370.0,
            y: 20.0,
        }
    }
}

pub fn render_preview(content: &PreviewContent, layout: PreviewLayout) {
    draw_rectangle(
        layout.x,
        layout.y,
        layout.w,
        layout.h,
        Color::new(0.05, 0.05, 0.05, 0.85),
    );

    draw_rectangle_lines(layout.x, layout.y, layout.w, layout.h, 2.0, WHITE);

    match content {
        PreviewContent::None => {}
        PreviewContent::Info(info) => {
            draw_text(info, layout.x + 10.0, layout.y + 30.0, 20.0, WHITE);
        }
        PreviewContent::Text(text) => {
            for (i, line) in text.lines().enumerate() {
                draw_text(
                    line,
                    layout.x + 10.0,
                    layout.y + 25.0 + i as f32 * 18.0,
                    18.0,
                    WHITE,
                );
            }
        }
        PreviewContent::Image(tex) => {
            let iw = tex.width();
            let ih = tex.height();
            let scale = (layout.w - 20.0) / iw;

            draw_texture_ex(
                tex,
                layout.x + 10.0,
                layout.y + 10.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(iw * scale, ih * scale)),
                    ..Default::default()
                },
            );
        }
    }
}
