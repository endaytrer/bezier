use bezier::canvas::BezierCanvas;
use bezier::convert::PNGCompatible;
use bezier::types::blend::BlendMode;
use bezier::types::colortype::*;
use bezier::linalg::*;

fn main() {
    let mut canvas = BezierCanvas::<u32, RGB>::new(800, 800);
    canvas.fill_rect(&Vec2::new(0.0, 0.0), &Vec2::new(0.5, 0.5), &RGB {r: 0xf2, g: 0x50, b: 0x22}, BlendMode::Override);
    canvas.fill_rect(&Vec2::new(0.5, 0.0), &Vec2::new(0.5, 0.5), &RGB {r: 0x7f, g: 0xba, b: 0x00}, BlendMode::Override);
    canvas.fill_rect(&Vec2::new(0.0, 0.5), &Vec2::new(0.5, 0.5), &RGB {r: 0x00, g: 0xa4, b: 0xef}, BlendMode::Override);
    canvas.fill_rect(&Vec2::new(0.5, 0.5), &Vec2::new(0.5, 0.5), &RGB {r: 0xff, g: 0xb9, b: 0x00}, BlendMode::Override);
    canvas.stroke_line(&Vec2::new(0.0, 0.0), &Vec2::new(1.0, 1.0), &RGB {r: 0xff, g: 0x00, b: 0x00}, BlendMode::Override);
    canvas.stroke_line(&Vec2::new(1.0, 0.0), &Vec2::new(0.0, 1.0), &RGB {r: 0xff, g: 0x00, b: 0x00}, BlendMode::Override);
    canvas.export_png("rect.png");
}
