use num::One;

use crate::BezierCanvasFactory;
use crate::linalg::{Vec2, Matrix2};
use crate::types::{RGB, R, RGBA, blend::BlendMode};

#[test]
fn init_canvas() {
    let canvas = BezierCanvasFactory::new()
        .set_size(1200, 800)
        .create_canvas::<u32, RGB>();

    assert_eq!(canvas.get_pixel(0, 0), RGB {r: 0, g: 0, b: 0});
    assert_eq!(canvas.get_pixel(1199, 799), RGB {r: 0, g: 0, b: 0});
    canvas.export_png("target/debug/examples/hello.png");
}
#[test]
fn rectangle_and_lines() {
    let mut canvas = BezierCanvasFactory::new()
        .set_size(1200, 800)
        .create_canvas::<u32, RGB>();

    canvas.fill_rect(&Vec2 {v: [100f32, 100f32]}, &Vec2{v: [300f32, 200f32]}, &RGB { r: 255, g: 0, b: 0 }, BlendMode::Override);
    canvas.stroke_line(&Vec2 {v: [43f32, 28f32]}, &Vec2 {v: [782f32, 123f32]}, &RGB { r: 0, g: 127, b: 255 }, BlendMode::Override);
    canvas.stroke_line(&Vec2 {v: [-123f32, 28f32]}, &Vec2 {v: [212f32, 680f32]}, &RGB { r: 0, g: 255, b: 255 }, BlendMode::Override);
    canvas.stroke_line(&Vec2 {v: [12f32, 28f32]}, &Vec2 {v: [12f32, 350f32]}, &RGB { r: 255, g: 0, b: 255 }, BlendMode::Override);
    assert_eq!(canvas.get_pixel(0, 0), RGB {r: 0, g: 0, b: 0});
    assert_eq!(canvas.get_pixel(120, 120), RGB {r: 255, g: 0, b: 0});
    canvas.export_png("target/debug/examples/rect.png");
}
#[test]
#[should_panic]
fn read_outside() {
    let canvas = BezierCanvasFactory::new()
        .set_size(1200, 800)
        .create_canvas::<u8, R>();
    canvas.get_pixel(1200, 0);
}

#[test]
fn linear_algebra() {
    let identity = Matrix2::one();
    let x = Vec2 {v: [0.75, 0.25]};
    assert_eq!(x + x, Vec2 {v: [1.5, 0.5]});
    assert_eq!(identity * x, x);
    assert_eq!(x * identity, x);
    assert_eq!(x * identity * x, x * x);
}

#[test]
fn bezier_curve() {
    let mut canvas = BezierCanvasFactory::new()
        .set_size(400, 400)
        .create_canvas::<u32, RGBA>();

    canvas.fill_oval(&Vec2{v: [200f32, 200f32]}, &Vec2{v: [180f32, 120f32]}, &RGBA { r: 255, g: 0, b: 0, a: 200 }, BlendMode::Alpha);
    let poses = [
        Vec2{v: [100f32, 100f32]},
        Vec2{v: [130f32, 130f32]},
        Vec2{v: [300f32, 100f32]},
        Vec2{v: [350f32, 150f32]},
        Vec2{v: [400f32, 200f32]},
        Vec2{v: [-5f32, 150f32]},
        Vec2{v: [150f32, 380f32]}
    ];
    for pnt in poses {
        canvas.fill_circle(&pnt, 4.8f32, &RGBA { r: 255, g: 100, b: 100, a: 235 }, BlendMode::Alpha);
    }
    for i in (1..poses.len()).step_by(3) {
        canvas.stroke_line(&poses[i - 1], &poses[i], &RGBA{r: 100, g: 100, b: 100, a: 255}, BlendMode::Alpha);
        canvas.stroke_line(&poses[i + 1], &poses[i + 2], &RGBA{r: 100, g: 100, b: 100, a: 255}, BlendMode::Alpha);
    }
    for i in (1..poses.len()).step_by(3) {
        canvas.stroke_bezier::<4>(&poses[i - 1..i + 3], &RGBA {r: 255, g: 255, b: 255, a: 255}, 50, BlendMode::Alpha);
    }
    canvas.export_png("target/debug/examples/bezier.png");

}

#[test]
fn blend_modes() {
    let mut canvas = BezierCanvasFactory::new()
        .set_size(400, 400)
        .create_canvas::<u32, RGB>();

    canvas.fill_rect(&Vec2 { v: [0f32, 0f32]}, &Vec2 { v: [400f32, 400f32]}, &RGB { r: 0, g: 255, b: 0 }, BlendMode::Override);
    canvas.fill_rect(&Vec2 { v: [100f32, 100f32]}, &Vec2 { v: [150f32, 150f32]}, &RGB { r: 255, g: 255, b: 0 }, BlendMode::Screen);
    canvas.fill_rect(&Vec2 { v: [150f32, 150f32]}, &Vec2 { v: [150f32, 150f32]}, &RGB { r: 255, g: 0, b: 255 }, BlendMode::Multiply);

    assert_eq!(canvas.get_pixel(50, 50), RGB{r: 0, g: 255, b: 0});
    assert_eq!(canvas.get_pixel(125, 125), RGB{r: 255, g: 255, b: 0});
    assert_eq!(canvas.get_pixel(200, 200), RGB{r: 255, g: 0, b: 0});
    assert_eq!(canvas.get_pixel(275, 275), RGB{r: 0, g: 0, b: 0});

    canvas.export_png("target/debug/examples/blend.png");

}